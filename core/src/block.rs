use std::fmt::Display;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use utils::{
    coder,
    coder::get_hash
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BlockHeader {
    pub time: i64,
    // 当前块数据唯一哈希
    pub tx_hash: String,
    // 上一个块的哈希
    pub pre_hash: String,
    // 挖矿难度
    difficulty: i32,
    // 块递增随机数
    nonce: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,
}

impl Display for BlockHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.tx_hash, self.nonce)
    }
}

impl Block {
    pub fn new_block(data: String, pre_hash: String, difficulty: i32) -> Block {
        let transactions = coder::my_serialize(&data);
        let tx_hash = get_hash(&transactions[..]);

        let time = Utc::now().timestamp();
        let mut block = Block {
            header: BlockHeader {
                time,
                tx_hash,
                pre_hash,
                difficulty,
                nonce: 0,
            },
            hash: "".to_string(),
            data,
        };
        block.set_hash();
        block.proof_of_work();
        block
    }

    // 设置hash
    fn set_hash(&mut self) {
        let header = coder::my_serialize(&(self.header));
        self.hash = get_hash(&header[..]);
    }

    // 工作量证明覆盖hash
    fn proof_of_work(&mut self) {
        if self.header.difficulty < 0 {
            return;
        }
        let target_prefix = "0".repeat(self.header.difficulty as usize);
        while !self.hash.starts_with(&target_prefix) {
            self.hash = self.calculate_hash();
            self.header.nonce += 1;
        }
    }

    fn calculate_hash(&self) -> String {
        let data = self.header.to_string();
        get_hash(data.as_bytes())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn should_test() {
        todo!()
    }
}