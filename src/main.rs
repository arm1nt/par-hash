use std::path::PathBuf;
use std::sync::{mpsc, Arc};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use colored::Colorize;
use crate::hashing::HashComputer;
use crate::models::{HashFunctionType, HashingConfig, InternalStateUpdate};
use crate::progress_tracker::ProgressTracker;
use input::cli::{parse_cli_arguments, Cli};
use input::input::{get_hash_function, get_hash_target};

mod hasher;
mod hashing;
mod progress_tracker;
mod util;
mod models;
mod merkle_tree;
mod input;

fn print_banner() {
    let banner = r#"
                                __               __
        ____  ____ ______      / /_  ____ ______/ /_
       / __ \/ __ `/ ___/_____/ __ \/ __ `/ ___/ __ \
      / /_/ / /_/ / /  /_____/ / / / /_/ (__  ) / / /
     / .___/\__,_/_/        /_/ /_/\__,_/____/_/ /_/
    /_/"#.bold().magenta();

    println!("{banner}\n");
}

fn get_messaging_channel(cli: &Cli) -> (Option<Sender<InternalStateUpdate>>, Option<Receiver<InternalStateUpdate>>) {
    if cli.no_progress == false {
        let (tx, rx) = mpsc::channel();
        (Some(tx), Some(rx))
    } else {
        (None, None)
    }
}

fn init_progress_tracker(cli: &Cli, target: &PathBuf, rx: Option<Receiver<InternalStateUpdate>>) -> Option<JoinHandle<()>> {
    if cli.no_progress == true {
        return None;
    }

    println!("> Initializing progress tracker...");
    let progress_tracker: ProgressTracker = ProgressTracker::init(target);

    let thread = thread::spawn(move || {
        progress_tracker.track_progress(rx.unwrap());
    });

    println!("> Completed progress tracker initialization!");
    Some(thread)
}

fn main() {

    let cli: Cli = parse_cli_arguments();

    print_banner();

    let hash_target = get_hash_target(&cli);
    let hash_function: HashFunctionType = get_hash_function(&cli);
    let hashing_config: HashingConfig = HashingConfig {
        split_threshold: cli.split_size.clone(),
        chunk_size: cli.chunk_size.clone()
    };

    let input = format!("Computing {:?}-based hash value for {:?}", hash_function, hash_target).magenta().bold();
    println!("\n{input}\n");

    // Messaging channel to update the internal state and total progress
    let (tx, rx) = get_messaging_channel(&cli);

    let progress_tracker: Option<JoinHandle<()>> = init_progress_tracker(&cli, &hash_target, rx);

    let hash_computer: Arc<HashComputer> = HashComputer::new(hashing_config, hash_function, tx);
    println!("> Starting to compute hash value...\n");
    let output: Vec<u8> = hash_computer.compute_hash(hash_target);
    drop(hash_computer);

    // Terminate progress tracker thread
    match progress_tracker {
        Some(progress_tracker) => progress_tracker.join().unwrap_or_else(|_| {
            eprintln!("An error occurred while waiting for the progress tracker thread to terminate!");
        }),
        _ => {}
    }

    let encoded_hash_val = format!("{:?}-based hash: {:?}", hash_function, hex::encode(output)).cyan().bold();
    println!("\n\n{}\n\n", encoded_hash_val);
}
