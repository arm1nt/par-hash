use std::io::{Error, ErrorKind, Write};
use std::path::PathBuf;
use std::env;
use std::str::FromStr;
use crate::models::HashFunctionType;
use crate::input::cli::Cli;
use crate::util::error_exit;
use crate::util::fs::is_supported_filetype;

fn query_cli_line(prompt: &str) -> std::io::Result<String> {

    print!("{}", prompt);
    match std::io::stdout().flush() {
        Err(e) => {
            return Err(Error::new(ErrorKind::Other, format!("Unable to print user prompt: {e:?}")));
        },
        _ => {}
    }

    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Err(e) => {
            return Err(Error::new(ErrorKind::Other, format!("Unable to query prompt response: {e:?}")));
        },
        _ => {}
    }

    if input.ends_with("\n") {
        input.pop();
    }

    Ok(input.trim().to_string())
}

fn validate_hash_target(target: &PathBuf) -> std::io::Result<()> {

    if !target.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Path '{:?}' not found", target)
        ));
    }

    if !is_supported_filetype(target) {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("'{:?}' refers to neither a file nor a directory!", target)
        ));
    }

    Ok(())
}

fn get_current_working_directory() -> PathBuf {
    match env::current_dir() {
        Ok(cwd) => cwd,
        Err(e) => error_exit(Some(format!("Unable to determine CWD: {e:?}")))
    }
}

fn query_hash_target() -> PathBuf {
    let prompt= "Enter the file or directory path: [Empty for CWD]\n> ";

    let target: String = query_cli_line(prompt).unwrap_or_else(|e| {
        error_exit(Some(format!("An error occurred while querying the target path: {e:?}")));
    });

    if target.is_empty() {
        get_current_working_directory()
    } else {
        PathBuf::from(target)
    }
}

fn validate_hash_function(algorithm: &String) -> std::io::Result<()> {
    match HashFunctionType::from_str(algorithm.as_str()) {
        Ok(_) => Ok(()),
        Err(_) => {
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!("'{algorithm}' is not a supported hashing function").as_str()
            ))
        }
    }
}

fn query_hash_function() -> String {
    let prompt = format!("Choose one of the following supported hash functions: {}\n[Empty for MD5]\n> ", HashFunctionType::str_overview());

    let response = query_cli_line(prompt.as_str()).unwrap_or_else(|e| {
        error_exit(Some(format!("An error occurred while querying the desired hash function: {e:?}")));
    });

    if response.is_empty() {
        String::from_str("md5").unwrap()
    } else {
        response
    }
}

pub fn get_hash_target(cli: &Cli) -> PathBuf {

    let target: PathBuf = match &cli.input {
        Some(value) => value.clone(),
        None => query_hash_target()
    };

    match validate_hash_target(&target) {
        Err(e) => error_exit(Some(format!("Invalid target path provided: {e:?}"))),
        _ => {}
    }

    target
}

pub fn get_hash_function(cli: &Cli) -> HashFunctionType {

    let hashing_algorithm: String = match &cli.algorithm {
        Some(val) => format!("{:?}", val),
        None => query_hash_function()
    };

    match validate_hash_function(&hashing_algorithm) {
        Err(e) => error_exit(Some(format!("{e:?}"))),
        _ => {}
    }

    HashFunctionType::from_str(hashing_algorithm.as_str()).unwrap()
}
