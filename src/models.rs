use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::models::HashFunctionType::{MD5, SHA1, SHA2_256, SHA2_512, SHA3_256, SHA3_512};

#[derive(Debug, Default)]
pub struct InternalState {
    pub nr_of_sub_dirs: u64,
    pub nr_of_files: u64,
    pub total_size_to_process: u64, // in bytes

    pub nr_of_processed_sub_dirs: u64,
    pub nr_of_processed_files: u64,
    pub processed_size: u64, // in bytes
}

#[derive(Debug)]
pub struct InternalStateUpdate {
    pub target_type: TargetType,
    pub processed_bytes: u64
}

#[derive(Debug, PartialEq)]
pub enum TargetType {
    FILE,
    DIRECTORY,
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
