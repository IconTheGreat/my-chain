use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Block {
    pub index: u64,
    pub data: String,
    pub height: usize,
    pub timestamp: u64,
    pub prev_blockhash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, data: String, prev_blockhash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hasher.update(prev_blockhash.as_bytes());
        let result = hasher.finalize();
        let hash = format!("{:x}", result);
        Block {
            index,
            data,
            height: 0,
            timestamp,
            prev_blockhash,
            hash,
            nonce: 0,
        }
    }
}
