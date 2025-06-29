use std::process;
use crate::util::constants::PAR_HASH_DEFAULT_ERROR_CODE;

pub mod cli;
pub mod input;
pub mod constants;

pub fn error_exit(msg: Option<String>) -> ! {
    match msg {
        Some(msg) => eprintln!("{}", msg),
        _ => {}
    }
    process::exit(PAR_HASH_DEFAULT_ERROR_CODE)
}
