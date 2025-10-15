use crate::block::Block;
use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut chain = Vec::new();
        let tx1 = Transaction::new("address1".into(), 100);
        let genesis = Block::new(0, vec![tx1], "0".into());
        chain.push(genesis);
        Blockchain { chain }
    }

    pub fn add_block(&mut self, data: Vec<Transaction>) {
        let prev_block = self.chain.last().unwrap();
        let new_block = Block::new(prev_block.index + 1, data, prev_block.hash.clone());
        self.chain.push(new_block);
    }

    pub fn get_latest_block (&mut self) -> Option<Block> {
        let last_block = self.chain.pop();
        return last_block;
    }
}
