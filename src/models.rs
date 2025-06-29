use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::models::HashFunctionType::{MD5, SHA1, SHA2_256, SHA2_512, SHA3_256, SHA3_512};

#[derive(Debug)]
pub struct InternalState {
    nr_of_subdirs: usize,
    nr_of_files: usize,
    total_size_to_process: usize, // in bytes
    nr_of_processed_files: usize,
    nr_of_processed_subdirs: usize,
    processed_size: usize, // in bytes
}

#[derive(Debug)]
pub struct InternalStateUpdate {
    pub test: String
    // processed file or directory
    // bytes processed
    // time it took to processes this file/folder
    // perhaps computed hash value
    // etc.
}

pub struct HashingConfig {
    pub split_threshold: Option<u64>, // in bytes
    pub chunk_size: Option<u64>, // in bytes
}

/// Hash functions supported by par-hash
#[derive(Debug, PartialEq, Clone, Copy, EnumIter)]
pub enum HashFunctionType {
    MD5,
    SHA1,
    SHA2_256,
    SHA2_512,
    SHA3_256,
    SHA3_512,
}

impl std::str::FromStr for HashFunctionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "md5" => Ok(MD5),
            "sha1" => Ok(SHA1),
            "sha2_256" => Ok(SHA2_256),
            "sha2_512" => Ok(SHA2_512),
            "sha3_256" => Ok(SHA3_256),
            "sha3_512" => Ok(SHA3_512),
            _ => Err(format!("'{s}' is not a supported hashing algorithm!"))
        }
    }
}

impl HashFunctionType {

    pub fn str_overview() -> String {
        let mut supported_types: Vec<String> = vec![];

        for supported_type in HashFunctionType::iter() {
            supported_types.push(format!("{:?}", supported_type));
        }

        supported_types.join(", ")
    }
}
