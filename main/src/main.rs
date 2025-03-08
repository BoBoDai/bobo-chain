use core::block_chain;

fn main() {
    // 新建链，难度系数为2算的会比较快，过大会算的很慢，为 -1 不进行工作量证明
    let mut bc = block_chain::BlockChain::new_block_chain(4);

    bc.add_block("a -> b: 5btc".to_string());

    bc.add_block("c -> d: 5btc".to_string());

    for b in bc.blocks {
        println!("++++++++++++++++++++++++++++++++++++++++++++++++++++");
        println!("{:#?}", b);
        println!("");
    }
}
