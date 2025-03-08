use core::block_chain::BlockChain;
use std::io;
pub struct CliHandler;

impl CliHandler {
    pub fn run(blockchain: &mut BlockChain) {
        loop {
            println!("🔹 请输入指令 (或输入 'exit' 退出): ");

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let data = input.trim().to_string();
            let data = data.splitn(2, " ").collect::<Vec<&str>>();

            if data[0] == "exit" {
                println!("👋  退出区块链...");
                break;
            }

            if data[0] == "add" {
                println!("⛏️ 正在创建区块");
                match blockchain.add_block(data[1].to_string()) {
                    Ok(_) => println!("💵 创建完成"),
                    Err(e) => println!("❌ 链被更改，无法新增加区块。"),
                }
                continue;
            }

            if data[0] == "list" {
                for b in &blockchain.blocks {
                    println!("++++++++++++++++++++++++++++++++++++++++++++++++++++");
                    println!("{:#?}", b);
                }
                continue;
            }
            println!("❌ 请输入 add 加交易信息添加区块，输入list查看当前链。");
        }
    }
}
