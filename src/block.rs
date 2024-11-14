use std::fmt::{ self, Debug, Formatter };
use super::*;

pub struct Block {
    pub index: u32, // Index of where the block is located in the chain
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
}

impl Debug for Block {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {} at: {} with: {}", 
    &self.index, &hex::encode(&self.hash), &self.timestamp, &self.payload)
    }
}

impl Block {
    pub fn new(index: u32, timestamp: u128, prev_block_hash: BlockHash, 
        nonce: u64, payload: String) -> Self {
            Block {
                index,
                timestamp,
                hash: vec![0; 32],
                prev_block_hash,
                nonce,
                payload,
            }
        }
}