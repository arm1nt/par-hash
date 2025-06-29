use crate::hasher::{Hasher, HasherFactory};
use crate::models::HashFunctionType;

pub struct MerkleTree {
    algorithm: HashFunctionType,
    root_node: Option<Box<MerkleNode>>
}

#[derive(Clone)]
struct MerkleNode {
    hash: Vec<u8>,
    left_child: Option<Box<MerkleNode>>,
    right_child: Option<Box<MerkleNode>>
}

impl MerkleTree {

    /// Create a new uninitialized merkle tree instance
    pub fn new(algorithm: &HashFunctionType) -> Self {
        MerkleTree {
            algorithm: algorithm.clone(),
            root_node: None
        }
    }

    pub fn get_root_hash(&self) -> Vec<u8> {
        if self.root_node.is_none() {
            panic!("Attempted to get root hash without initializing the merkle tree!");
        }

        self.root_node.clone().unwrap().hash
    }

    pub fn initialize_from_vector(&mut self, entries: &Vec<Vec<u8>>) {
        self.root_node = self.private_initialize_from_vector(entries, 0, entries.len() - 1);
    }

    fn private_initialize_from_vector(&mut self, entries: &Vec<Vec<u8>>, start: usize, end: usize) -> Option<Box<MerkleNode>> {

        // Leaf node
        if start == end {
            return Some(Box::new(MerkleNode {
                hash: entries[start].clone(),
                left_child: None,
                right_child: None
            }));
        }


        let left_child = self.private_initialize_from_vector(
            entries,
            start,
            start + (end-start)/2
        );

        let right_child = self.private_initialize_from_vector(
            entries,
            start + (end-start)/2 + 1,
            end
        );

        let hash = HasherFactory::get_instance(&self.algorithm).compute_hash(
            &mut concat_hashes(
                left_child.clone(),
                right_child.clone()
            )
        );

        Some(Box::new(MerkleNode { hash, left_child, right_child }))
    }

}

fn concat_hashes(left: Option<Box<MerkleNode>>, right: Option<Box<MerkleNode>>) -> Vec<u8> {

    match (left, right) {
        (None, None) => {
            panic!("Encountered illegal state: Both left and right children of non-leaf merkle node are none")
        },
        (Some(node), None) | (None, Some(node)) => {
            node.hash.clone()
        },
        (Some(left_node), Some(right_node)) => {
            [left_node.hash, right_node.hash].concat()
        }
    }
}
