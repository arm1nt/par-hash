use std::io::Write;
use std::sync::{mpsc, Arc};
use std::sync::mpsc::{Receiver, Sender};
use crate::hashing::HashComputer;
use crate::models::{HashFunctionType, InternalStateUpdate};
use crate::util::cli::{parse_cli_arguments, Cli};
use crate::util::input::{get_hash_function, get_hash_target};

mod hasher;
mod hashing;
mod progress_tracker;
mod util;
mod models;
mod merkle_tree;


fn main() {

}
