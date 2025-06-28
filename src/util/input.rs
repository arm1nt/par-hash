use std::io::{Error, ErrorKind, Write};
use std::path::PathBuf;
use std::process;
use std::str::FromStr;
use crate::models::HashFunctionType;
use crate::util::cli::Cli;

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
            format!("No file/directory found at path '{:?}'", target)
        ));
    }

    if !target.is_dir() && !target.is_file() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Path '{:?}' references neither a file nor a directory!", target)
        ));
    }

    Ok(())
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

pub fn get_hash_target(cli: &Cli) -> PathBuf {

    let target: PathBuf = if cli.input.is_some() {
        cli.input.clone().unwrap()
    } else {
        let prompt= "Enter the file or directory path: [Empty for CWD]\n> ";

        match query_cli_line(prompt) {
            Ok(target_string) => PathBuf::from(target_string),
            Err(e) => {
                eprintln!("An error occurred while querying the file/directory path: {e:?}");
                process::exit(1);
            }
        }
    };

    match validate_hash_target(&target) {
        Err(e) => {
            eprintln!("Invalid path provided: {e:?}");
            process::exit(1);
        },
        _ => {}
    }

    target
}

pub fn get_hash_function(cli: &Cli) -> HashFunctionType {

    let algorithm: String = if cli.algorithm.is_some() {
        format!("{:?}", cli.algorithm.clone().unwrap())
    } else {
        let prompt = format!("Choose one of the following supported hash functions: {}\n[Empty for MD5]\n> ", HashFunctionType::str_overview());

        let mut response = query_cli_line(prompt.as_str()).unwrap_or_else(|e| {
            eprintln!("An error occurred while querying the desired hash function: {e:?}");
            process::exit(1)
        });

        if response.is_empty() {
            response = String::from_str("md5").unwrap();
        }

        response
    };

    match validate_hash_function(&algorithm) {
        Err(e) => {
            eprintln!("{e:?}");
            process::exit(1);
        },
        _ => {}
    }

    HashFunctionType::from_str(algorithm.as_str()).unwrap()
}
