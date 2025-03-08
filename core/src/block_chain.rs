use crate::block::Block;

pub struct BlockChain {
    //链的载体
    pub blocks: Vec<Block>,
    // 工作量证明的难度
    difficulty: i32
}

impl BlockChain {
    //增加新区块
    pub fn add_block(&mut self, data: String) {
        let pre_block = &self.blocks.last().unwrap();
        let new_block = Block::new_block(data, pre_block.hash.clone(), self.difficulty);
        self.blocks.push(new_block);
    }

    // 新建创世块，工作量难度为-1表示不需要工作量证明
    pub fn new_genesis_block() -> Block {
        Block::new_block("Genesis block".to_string(), String::from(""), -1)
    }

    //新建链
    pub fn new_block_chain(difficulty: i32) -> BlockChain {
        BlockChain {
            blocks: vec![BlockChain::new_genesis_block()],
            difficulty
        }
    }
}