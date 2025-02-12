//!
//! This is the block module.<br>
//! The bock struct is used to create a block.
//!

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::Write;

/// This is the block struct.<br>
/// It includes index, timestamp, <br>
/// hash previous hash and data field.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub previous_hash: String,
    pub hash: String,
    pub data: String,
}

///
/// Functions for Struct Block.
///
impl Block {
    ///
    /// Function to create a new block struct.
    ///
    pub fn new(index: u64, timestamp: u64, previous_hash: String, data: String) -> Block {
        let mut block = Block {
            index,
            timestamp,
            previous_hash,
            hash: String::new(),
            data,
        };
        block.hash = block.calculate_hash();
        block
    }

    ///
    /// This function calculates the hash for the block.<br>
    /// The hash is SHA256.
    ///
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_string());
        hasher.update(self.timestamp.to_string());
        hasher.update(&self.previous_hash);
        hasher.update(&self.data);
        let result = hasher.finalize();
        let mut hash = String::new();
        for byte in result {
            write!(&mut hash, "{:02x}", byte).unwrap();
        }
        hash
    }
}
