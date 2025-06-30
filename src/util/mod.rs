use std::process;

pub mod fs;
pub mod math;

pub const PAR_HASH_DEFAULT_ERROR_CODE: i32 = 1;

pub fn error_exit(msg: Option<String>) -> ! {
    match msg {
        Some(msg) => eprintln!("{}", msg),
        _ => {}
    }
    process::exit(PAR_HASH_DEFAULT_ERROR_CODE)
}
