use crate::block_chain::BlockChain;

pub mod block;
pub mod block_chain;

pub trait DbTrait {
    fn save_chain(&self, chain: &mut BlockChain);
    fn load_chain(&self) -> BlockChain;
    fn is_chain_exist(&self) -> bool;
}
