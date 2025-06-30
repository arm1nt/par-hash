use std::fs::{DirEntry, File, Metadata};
use std::io::{Error, Read, Seek, SeekFrom};
use std::path::PathBuf;
use crate::util::error_exit;

pub fn read_chunk(path: &PathBuf, start: u64, end: u64) -> Vec<u8> {
    let mut file: File = get_file(path);

    file.seek(SeekFrom::Start(start)).unwrap_or_else(|e| {
        error_exit(Some(format!("Error seeking chunk ({start}, {end}) in file '{:?}': {e:?} ", path)));
    });

    let mut buffer: Vec<u8> = vec![0u8; (end-start) as usize];

    let bytes_read = file.read(&mut buffer).unwrap_or_else(|e| {
        error_exit(Some(format!("Unable to read chunk ({start}, {end}) from file '{:?}': {e:?}", path)));
    });
    buffer.truncate(bytes_read);

    buffer
}

pub fn get_file(path: &PathBuf) -> File {
    File::open(path).unwrap_or_else(|e| {
        error_exit(Some(format!("Unable to open file at path '{:?}': {e:?}", path)));
    })
}

pub fn get_metadata(path: &PathBuf) -> Metadata {
    path.metadata().unwrap_or_else(|e| {
        error_exit(Some(format!("Unable to get metadata for '{:?}: {e:?}", path)));
    })
}

pub fn get_dir_entry(path: &PathBuf, entry_res: Result<DirEntry, Error>) -> DirEntry {
    match entry_res {
        Ok(dir_entry) => dir_entry,
        Err(e) => {
            error_exit(Some(format!("Unable to unwrap entry in directory '{:?}': {e:?}", path)))
        }
    }
}

pub fn is_supported_filetype(path: &PathBuf) -> bool {
    path.is_dir() || path.is_file()
}
