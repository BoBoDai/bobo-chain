use core::block_chain;
use main::CliHandler;

fn main() {
    // 新建链，难度系数过大会算的很慢，为 -1 不进行工作量证明
    let mut bc = block_chain::BlockChain::new_block_chain(4);
    CliHandler::run(&mut bc);
}
