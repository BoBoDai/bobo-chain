use chrono::prelude::*;
use serde::Serialize;
use utils::coder;
use utils::coder::get_hash;

#[derive(Debug, Serialize)]
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

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,
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
        let date = Utc::now().timestamp().to_string();
        let data = self.data.to_string();
        let nance = self.header.nonce;
        let data = date + data.as_str() + nance.to_string().as_str();
        get_hash(data.as_bytes())
    }
}
