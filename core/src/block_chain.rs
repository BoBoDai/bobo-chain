use serde::{Deserialize, Serialize};
use crate::block::Block;
use crate::Db;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BlockChain {
    //链的载体
    pub blocks: Vec<Block>,
    // 工作量证明的难度
    difficulty: i32,
}

impl BlockChain {
    //增加新区块
    pub fn add_block(&mut self, data: String) -> Result<(), String> {
        let pre_block = &self.blocks.last().unwrap();
        let new_block = Block::new_block(data, pre_block.hash.clone(), self.difficulty);
        if self.block_chain_is_valid() {
            self.blocks.push(new_block);
            Db::save_chain(self);
            Ok(())
        } else {
            Err("valid failed".to_string())
        }
    }

    // 新建创世块，工作量难度为-1表示不需要工作量证明
    fn new_genesis_block() -> Block {
        Block::new_block("Genesis block".to_string(), String::from(""), -1)
    }

    //新建链
    pub fn new_block_chain(difficulty: i32) -> BlockChain {
        if Db::is_chain_exist() {
            return Db::load_chain();
        }
        let mut chain = BlockChain {
            blocks: vec![BlockChain::new_genesis_block()],
            difficulty,
        };
        Db::save_chain(&mut chain);
        chain
    }

    // 验证当前链没有被篡改
    fn block_chain_is_valid(&self) -> bool {
        for i in 0..self.blocks.len() - 1 {
            let block = &self.blocks[i];
            let pre_block = &self.blocks[i + 1];
            if block.header.pre_hash != pre_block.hash {
                return false;
            }
        }
        true
    }
}
