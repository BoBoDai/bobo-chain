use chrono::prelude::*;
use serde::Serialize;
use utils::coder;
use utils::coder::get_hash;

#[derive(Debug, Serialize)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String,
    pub pre_hash: String,
    difficulty: i32,
    nonce: u64,
}

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,
}

impl Block {
    fn set_hash(&mut self) {
        let header = coder::my_serialize(&(self.header));
        self.hash = get_hash(&header[..]);
    }

    pub fn new_block(data: String, pre_hash: String, difficulty: i32) -> Block {
        let transactions = coder::my_serialize(&data);
        let tx_hash = coder::get_hash(&transactions[..]);

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
        let data = self.data.clone();
        let nance = self.header.nonce;
        let data = data + nance.to_string().as_str();
        let hasher = get_hash(data.as_bytes());
        hasher
    }
}
