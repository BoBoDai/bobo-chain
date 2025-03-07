use crate::block::Block;

pub struct BlockChain {
    pub blocks: Vec<Block>,
    // 工作量证明难度
    difficulty: i32
}

impl BlockChain {
    pub fn add_block(&mut self, data: String) {
        let pre_block = &self.blocks.last().unwrap();
        let new_block = Block::new_block(data, pre_block.hash.clone(), self.difficulty);
        self.blocks.push(new_block);
    }

    pub fn new_genesis_block() -> Block {
        Block::new_block("Genesis block".to_string(), String::from(""), -1)
    }

    pub fn new_block_chain(difficulty: i32) -> BlockChain {
        BlockChain {
            blocks: vec![BlockChain::new_genesis_block()],
            difficulty
        }
    }
}