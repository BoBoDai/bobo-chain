use std::fs;
use std::io::Write;
use core::block_chain::BlockChain;
use core::DbTrait;

const FILE_NAME: &str = "block_chain";

pub struct Db;

impl DbTrait for Db {
    fn save_chain(&self, block_chain: &mut BlockChain) {
        let json = serde_json::to_string(block_chain).unwrap();
        let mut file = fs::File::create(FILE_NAME).unwrap();
        file.write(json.as_bytes()).unwrap();
    }

    fn load_chain(&self) -> BlockChain {
        let mut file = fs::File::open(FILE_NAME).unwrap();
        let data: BlockChain = serde_json::from_reader(&mut file).unwrap();
        data
    }

    fn is_chain_exist(&self) -> bool {
        fs::exists(FILE_NAME).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_save_chain_in_file_then_load_chain_by_file() {
        // given
        let db = Db;
        let mut except = BlockChain::new_block_chain(1, &db);
        db.save_chain(&mut except);
        //when
        let is_file_exist = db.is_chain_exist();
        let result = db.load_chain();
        //then
        assert!(is_file_exist);
        assert_eq!(except, result);
        fs::remove_file(FILE_NAME).unwrap();
    }
}
