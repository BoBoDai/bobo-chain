use core::block_chain;

fn main() {
    let mut bc = block_chain::BlockChain::new_block_chain(4);

    bc.add_block("a -> b: 5btc".to_string());

    bc.add_block("c -> d: 5btc".to_string());

    for b in bc.blocks {
        println!("++++++++++++++++++++++++++++++++++++++++++++++++++++");
        println!("{:#?}", b);
        println!("");
    }
}
