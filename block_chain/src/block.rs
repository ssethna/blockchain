//!
//! This is the block module.<br>
//! The bock struct is used to create a block.
//!

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::cmp::PartialEq;
use std::fmt::Write;

/// This is the block struct.<br>
/// It includes index, timestamp, <br>
/// hash previous hash and data field.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub block_index: u64,
    pub timestamp: u64,
    pub previous_hash: String,
    pub hash: String,
    pub data: Value,
}

///
/// Adding Trait PartialEq.
///
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.block_index == other.block_index
            && self.timestamp == other.timestamp
            && self.previous_hash == other.previous_hash
            && self.hash == other.hash
            && self.data == other.data
    }
}

///
/// Functions for Struct Block.
///
impl Block {
    ///
    /// Function to create a new block struct.
    ///
    pub fn new(block_index: u64, timestamp: u64, previous_hash: String, data: Value) -> Block {
        let mut block = Block {
            block_index,
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
        hasher.update(self.block_index.to_string());
        hasher.update(self.timestamp.to_string());
        hasher.update(&self.previous_hash);
        hasher.update(&self.data.to_string());
        let result = hasher.finalize();
        let mut hash = String::new();
        for byte in result {
            write!(&mut hash, "{:02x}", byte).unwrap();
        }
        hash
    }
}
