use std::cmp::min;
use std::fs::{DirEntry, File, Metadata};
use std::path::PathBuf;
use std::fs;
use std::io::{BufReader, Read};
use std::sync::Arc;
use std::sync::mpsc::Sender;
use crate::models::{HashFunctionType, HashingConfig, InternalStateUpdate};
use rayon::prelude::*;
use crate::hasher::{Hasher, HasherFactory};
use crate::merkle_tree::MerkleTree;
use crate::models::TargetType::{DIRECTORY, FILE};
use crate::util::error_exit;
use crate::util::fs::{get_dir_entry, get_file, get_metadata, is_supported_filetype, read_chunk};
use crate::util::math::{gb_to_bytes, mb_to_bytes};

pub struct HashComputer {
    config: HashingConfig,
    hash_algorithm: HashFunctionType,
    progress_tx: Option<Sender<InternalStateUpdate>>, // Sender to the progress tracker thread
}

impl HashComputer {

    /// Initialize a new HashComputer instance that contains all metadata/config needed to compute
    /// the target's hash value
    pub fn new(
        config: HashingConfig,
        algorithm: HashFunctionType,
        progress_tx: Option<Sender<InternalStateUpdate>>
    ) -> Arc<Self> {
        Arc::new(HashComputer {
            config,
            hash_algorithm: algorithm,
            progress_tx
        })
    }

    pub fn compute_hash(&self, target: PathBuf) -> Vec<u8> {

        if target.is_file() {
            self.abstract_compute_file_hash(target)
        } else if target.is_dir() {
            self.compute_directory_hash(target)
        } else {
            error_exit(Some(format!("Path {:?} references neither a file nor a directory!", target)));
        }
    }

    fn compute_directory_hash(&self, path: PathBuf) -> Vec<u8> {

        let directory_entries: Vec<DirEntry> = fs::read_dir(&path)
            .unwrap_or_else(|e| {
                error_exit(Some(format!("Unable to unwrap entry of directory at path '{:?}': {e:?}", path)));
            })
            .map(|entry| get_dir_entry(&path, entry))
            .filter(|entry| is_supported_filetype(&entry.path()))
            .collect();

        // Concurrently compute the hash value of each directory entry
        let mut dir_entry_hashes: Vec<Vec<u8>> = directory_entries
            .par_iter() // Maintains the order of the entries => hash value reproducible
            .map(|directory_entry| self.compute_hash(directory_entry.path()))
            .collect();

        // Add hashed directory metadata that should be considered when computing the directories final hash
        let mut hasher_name: Box<dyn Hasher> = HasherFactory::get_instance(&self.hash_algorithm);
        dir_entry_hashes.push(hasher_name.compute_hash(&mut path.to_string_lossy().as_bytes().to_vec()));

        // From all obtained hash values, compute a merkle tree and get the hash value of its root node
        let mut tree: MerkleTree = MerkleTree::new(&self.hash_algorithm);
        tree.initialize_from_vector(&dir_entry_hashes);

        self.send_internal_state_update(InternalStateUpdate {
            target_type: DIRECTORY,
            processed_bytes: None
        });

        tree.get_root_hash()
    }

    fn abstract_compute_file_hash(&self, path: PathBuf) -> Vec<u8> {

        let file_metadata = get_metadata(&path);

        if self.config.split_threshold.is_none() {
            let res = self.compute_simple_file_hash(path);

            self.send_file_update(file_metadata.len());
            return res;
        }

        let res = if file_metadata.len() >= self.config.split_threshold.unwrap() {
            self.compute_chunked_file_hash(path)
        } else {
            self.compute_simple_file_hash(path)
        };

        self.send_file_update(file_metadata.len());
        res
    }

    fn compute_simple_file_hash(&self, path: PathBuf) -> Vec<u8> {
        let file: File = get_file(&path);
        let mut reader: BufReader<File> = BufReader::new(file);

        let mut hasher: Box<dyn Hasher> = HasherFactory::get_instance(&self.hash_algorithm);

        // To not waste memory, we do not load the entire file into memory at once but read in chunks
        let mut buffer= [0u8; 8192];

        loop {
            let n = reader.read(&mut buffer).unwrap_or_else(|e| {
                error_exit(Some(format!("Unable to read from file '{:?}': {e:?}", path)));
            });

            if n == 0 {
                break;
            }

            hasher.update(&mut buffer);
        }

        hasher.finalize()
    }

    fn compute_chunked_file_hash(&self, path: PathBuf) -> Vec<u8> {

        // Compute chunk ranges to prevent having to read the whole file into memory at once
        let metadata: Metadata = get_metadata(&path);
        let chunk_size = self.get_chunk_size(&metadata);
        let mut chunk_ranges: Vec<(u64, u64)> = vec![];

        for i in (0.. metadata.len()).step_by(chunk_size+1) {
            let end = min(metadata.len(), i + chunk_size as u64);
            chunk_ranges.push((i, end));
        }

        let chunk_hashes: Vec<Vec<u8>> = self.process_chunks(&path, chunk_ranges);
        let mut tree: MerkleTree = MerkleTree::new(&self.hash_algorithm);
        tree.initialize_from_vector(&chunk_hashes);

        tree.get_root_hash()
    }

    fn process_chunks(&self, path: &PathBuf, chunk_ranges: Vec<(u64, u64)>) -> Vec<Vec<u8>> {
        chunk_ranges
            .par_iter()
            .map(|range| {
                self.compute_file_chunk_hash(path, range)
            })
            .collect()
    }

    fn compute_file_chunk_hash(&self, path: &PathBuf, range: &(u64, u64)) -> Vec<u8> {
        let mut chunk = read_chunk(path, range.0, range.1);
        HasherFactory::get_instance(&self.hash_algorithm).compute_hash(&mut chunk)
    }

    fn get_chunk_size(&self, metadata: &Metadata) -> usize {
        if self.config.chunk_size.is_some() {
            return self.config.chunk_size.unwrap() as usize;
        }

        let file_size = metadata.len();

        // between 100 MB and 1 GB => 16 MB chunk size
        if file_size >= mb_to_bytes(100) && file_size <= gb_to_bytes(1) {
            return mb_to_bytes(16) as usize;
        }

        // between 1 GB and 10 GB => 64 MB chunk size
        if file_size > gb_to_bytes(1) && file_size <= gb_to_bytes(10) {
            return mb_to_bytes(64) as usize;
        }

        // everything bigger than 10 GB => 256 MB chunk size
        mb_to_bytes(256) as usize
    }

    fn send_file_update(&self, size: u64) {
        self.send_internal_state_update(InternalStateUpdate {
            target_type: FILE,
            processed_bytes: Some(size)
        });
    }

    fn send_internal_state_update(&self, update: InternalStateUpdate) {
        let tx: Option<Sender<InternalStateUpdate>> = self.progress_tx.clone();

        match tx {
            Some(tx) => tx.send(update).unwrap_or_else(|_e| {}),
            None => {}
        }
    }

}
