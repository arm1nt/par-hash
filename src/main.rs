use std::path::PathBuf;
use std::sync::{mpsc, Arc};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use crate::hashing::HashComputer;
use crate::models::{HashFunctionType, HashingConfig, InternalStateUpdate};
use crate::progress_tracker::ProgressTracker;
use crate::util::cli::{parse_cli_arguments, Cli};
use crate::util::input::{get_hash_function, get_hash_target};

mod hasher;
mod hashing;
mod progress_tracker;
mod util;
mod models;
mod merkle_tree;


fn get_messaging_channel(cli: &Cli) -> (Option<Sender<InternalStateUpdate>>, Option<Receiver<InternalStateUpdate>>) {
    if cli.progress {
        let (tx, rx) = mpsc::channel();
        (Some(tx), Some(rx))
    } else {
        (None, None)
    }
}

fn init_progress_tracker(cli: &Cli, target: &PathBuf, rx: Option<Receiver<InternalStateUpdate>>) -> Option<JoinHandle<()>> {
    if cli.progress == false {
        return None;
    }

    let mut progress_tracker: ProgressTracker = ProgressTracker::init(target);

    let thread = thread::spawn(move || {
        progress_tracker.track_progress(rx.unwrap());
    });

    Some(thread)
}


fn main() {

    let cli: Cli = parse_cli_arguments();

    let hash_target = get_hash_target(&cli);
    let hash_function: HashFunctionType = get_hash_function(&cli);
    let hashing_config: HashingConfig = HashingConfig {
        split_threshold: cli.split_size.clone(),
        chunk_size: cli.chunk_size.clone()
    };

    // Messaging channel to update the internal state and total progress
    let (tx, rx) = get_messaging_channel(&cli);

    let progress_tracker: Option<JoinHandle<()>> = init_progress_tracker(&cli, &hash_target, rx);

    let hash_computer: Arc<HashComputer> = HashComputer::new(hashing_config, hash_function, tx);
    println!("Starting to compute hash value...\n");
    let output: Vec<u8> = hash_computer.compute_hash(hash_target);
    drop(hash_computer);

    // Terminate progress tracker thread
    match progress_tracker {
        Some(progress_tracker) => progress_tracker.join().unwrap_or_else(|_| {
            eprintln!("An error occurred while waiting for the progress tracker thread to terminate!");
        }),
        _ => {}
    }

    println!("\n--------------------------\n");
    println!("Hash value: {:?}", hex::encode(output));
    println!("\n--------------------------\n");
}
