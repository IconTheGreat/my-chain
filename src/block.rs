use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::transaction::Transaction;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Block {
    pub index: u64,
    pub data: Vec<Transaction>,
    pub height: usize,
    pub timestamp: u64,
    pub prev_blockhash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, data: Vec<Transaction>, prev_blockhash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut hasher = Sha256::new();
        hasher.update(data.iter().map(|tx| format!("{:?}", tx)).collect::<String>().as_bytes());
        hasher.update(prev_blockhash.as_bytes());
        let result = hasher.finalize();
        let hash = format!("{:x}", result);
        Block {
            index: 0,
            data,
            height: 0,
            timestamp,
            prev_blockhash,
            hash,
            nonce: 0,
        }
    }
}
