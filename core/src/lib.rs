pub mod block;
pub mod block_chain;

use std::fs;
use std::io::{Read, Write};
const FILE_NAME: &str = "block_chain";

struct Db;

impl Db {

    pub fn is_chain_exist() -> bool {
        fs::exists(FILE_NAME).unwrap()
    }

    pub fn save_chain(block_chain: &mut block_chain::BlockChain) {
        let json = serde_json::to_string(block_chain).unwrap();
        let mut file = fs::File::create(FILE_NAME).unwrap();
        file.write(json.as_bytes()).unwrap();
    }

    pub fn load_chain() -> block_chain::BlockChain {
        let mut file = fs::File::open(FILE_NAME).unwrap();
        let data: block_chain::BlockChain = serde_json::from_reader(&mut file).unwrap();
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block_chain::BlockChain;

    #[test]
    fn should_save_chain_in_file_then_load_chain_by_file() {
        // given
        let mut except = BlockChain::new_block_chain(1);
        Db::save_chain(&mut except);
        //when
        let is_file_exist = Db::is_chain_exist();
        let result = Db::load_chain();
        //then
        assert!(is_file_exist);
        assert_eq!(except, result);
        fs::remove_file(FILE_NAME).unwrap();
    }
}
