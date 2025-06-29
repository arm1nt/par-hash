use std::{fs, thread};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::time::Duration;
use crate::models::{InternalState, InternalStateUpdate, TargetType};
use crate::util::error_exit;
use crate::util::fs::{get_dir_entry, get_metadata, is_supported_filetype};

pub struct ProgressTracker {
    internal_state: Arc<Mutex<InternalState>>
}

impl ProgressTracker {

    /// Create a ProgressTracker with initialized internal state
    pub fn init(target: &PathBuf) -> Self {
        let mut internal_state: InternalState = InternalState::default();
        init_internal_state(target, &mut internal_state);

        ProgressTracker { internal_state: Arc::new(Mutex::new(internal_state)) }
    }

    /// Reads messages from the producer threads an updates the internal state accordingly
    pub fn track_progress(mut self, rx: Receiver<InternalStateUpdate>) {

        // Start a thread that continuously prints the current progress
        let running: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
        let running_clone: Arc<AtomicBool> = Arc::clone(&running);
        let state_clone: Arc<Mutex<InternalState>> = Arc::clone(&self.internal_state);

        let progress_printer = thread::spawn(move || {
            progress_formatter(running_clone, state_clone);
        });

        for state_update in rx {
            self.update_internal_state(state_update);
        }

        running.store(false, Ordering::Relaxed);
        progress_printer.join().unwrap_or_else(|_| {
            eprintln!("An error occurred while waiting for the progress printer thread!");
        });
    }

    fn update_internal_state(&mut self, update: InternalStateUpdate) {
        let mut state = self.internal_state.lock().unwrap();

        match update.processed_bytes {
            Some(val) => state.processed_size += val,
            None => {}
        }

        match update.target_type {
            TargetType::FILE => state.nr_of_processed_files += 1,
            TargetType::DIRECTORY => state.nr_of_processed_sub_dirs +=1
        }
    }

}

fn progress_formatter(running: Arc<AtomicBool>, state: Arc<Mutex<InternalState>>) {

    while running.load(Ordering::Relaxed) {
        print!("\x1B[2K\r:{:?}", state.lock().unwrap());
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(300));
    }
    std::io::stdout().flush().unwrap();
    thread::sleep(Duration::from_millis(300));
    print!("\x1B[2K\rDone!\n");
}

fn init_internal_state(path: &PathBuf, state: &mut InternalState) {

    if path.is_file() {
        add_file_impact_to_state(path, state);
        return;
    }

    let entries = fs::read_dir(path).unwrap_or_else(|e| {
        error_exit(Some(format!("Unable to unwrap ReadDir iterator over '{:?}': {e:?}", path)));
    });

    for entry in entries.map(|e| get_dir_entry(path, e)) {

        if !is_supported_filetype(&entry.path()) {
            continue
        } else if entry.path().is_file() {
            add_file_impact_to_state(&entry.path(), state);
            continue
        } else if entry.path().is_dir() {
            add_sub_dir_impact_to_state(&entry.path(), state);
            init_internal_state(&entry.path(), state);
            continue
        }
    }
}

fn add_file_impact_to_state(path: &PathBuf, state: &mut InternalState) {
    state.nr_of_files += 1;
    state.total_size_to_process += get_metadata(path).len();
}

fn add_sub_dir_impact_to_state(path: &PathBuf, state: &mut InternalState) {
    state.nr_of_sub_dirs += 1;
}
