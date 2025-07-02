# par-hash

`par-hash` is a multi-threaded command-line utility to efficiently compute a hash value for large files and complex folder hierachies, enabling you to e.g. quickly and reliably verify their integrity.

## How it works

The final hash value of a target (either a file or a directory) is computed based on an underlying hash function such as MD5, SHA-1, SHA-2, SHA-3, or even a custom implementation. The concrete hashing strategy differs slightly depending on the target, but always uses a Merkle-tree-based approach.

### File Hashing

To compute a file's hash, `par-hash` uses a threshold-based and Merkle-tree backed strategy:

- If no threshold is defined, or the file's size is below the threshold, the hash is computed by simply applying the underlying hash function directly to the entire file content.

- If a size threshold is defined and the file's size exceeds the threshold
  - The file is split into fixed-size chunks
  - The hash of each chunk is computed concurrently
  - A Merkle tree is built from the chunk hashes
  - The root hash of the Merkle tree represents the final hash of the file

### Directory Hashing

- For directories, `par-hash` performs recursive and concurrent hashing with metadata inclusion:

  - Recursively and concurrently compute the hash of each directory entry (files and subdirectories)
  - Compute the hash values of selected directory metadata
  - Construct a Merkle tree from all the resulting hashes
  - The root hash of the Merkle tree represents the final hash of the directory.
