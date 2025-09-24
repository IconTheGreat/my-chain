use crate::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut chain = Vec::new();
        let genesis = Block::new(0, "Genesis Block".into(), "0".into());
        chain.push(genesis);
        Blockchain { chain }
    }

    pub fn add_block(&mut self, data: String) {
        let prev_block = self.chain.last().unwrap();
        let new_block = Block::new(prev_block.index + 1, data, prev_block.hash.clone());
        self.chain.push(new_block);
    }

    pub fn get_latest_block (&mut self) -> Option<Block> {
        let last_block = self.chain.pop();
        return last_block;
    }
}
