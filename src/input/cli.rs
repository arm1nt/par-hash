use std::path::PathBuf;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "par-hash")]
#[command(version = env!["CARGO_PKG_VERSION"])]
#[command(propagate_version = true)]
#[command(about = "Tool to efficiently compute the hash for files and folders")]
#[command(author = "arm1nt")]
#[command(next_line_help = true)]
pub struct Cli {

    /// Path to input file or directory whose hash should be computed. If not specified as cli
    /// argument, the path is queried interactively during runtime.
    #[arg(short, long, value_name = "FILE|FOLDER PATH", required = false)]
    pub input: Option<PathBuf>,

    /// Hashing function to be used for computing the file/folder hash. If not specified as cli
    /// argument, the desired hash function to be used is queried interactively during runtime.
    #[arg(value_enum, short, long, required = false)]
    pub algorithm: Option<HashFunctionType>,

    /// File size threshold (in bytes) at which a file should be split into chunks to parallelize
    /// the computation of its hash value
    #[arg(short, long, value_name = "SPLIT THRESHOLD", required = false)]
    pub split_size: Option<u64>,

    /// When a file is split into fixed size chunks, this option specifies the chunk size. If the
    /// file size is not a multiple of the chosen chunk size, the last chunk will be smaller.
    #[arg(short, long, value_name = "CHUNK SIZE")]
    pub chunk_size: Option<u64>,

    /// Specify whether progress information should be displayed
    #[arg(short, long, required = false)]
    pub progress: bool,

    /// Print extra detailed information
    #[arg(short, long, required = false)]
    pub verbose: bool,
}

#[derive(ValueEnum, Clone, PartialEq, Debug)]
pub enum HashFunctionType {
    /// Usage not recommended as practical attacks against MD5 exist, but due to e.g. the small
    /// digest size and fast computation, there are still cases where its usage is appropriate.
    MD5,
    /// Usage not recommended as attacks against SHA1 are known, but there are still cases where
    /// its usage is appropriate.
    SHA1,
    /// SHA2 with 256-bit hash size
    SHA2_256,
    /// SHA2 with 512-bit hash size
    SHA2_512,
    /// SHA3 with 256-bit hash size
    SHA3_256,
    /// SHA3 with 512-bit hash size
    SHA3_512
}

pub fn parse_cli_arguments() -> Cli {
    Cli::parse()
}