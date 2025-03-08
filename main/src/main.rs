use core::block_chain;
use std::io;

fn main() {
    // 新建链，难度系数为2算的会比较快，过大会算的很慢，为 -1 不进行工作量证明
    let mut bc = block_chain::BlockChain::new_block_chain(4);

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
            bc.add_block(data[1].to_string());
            println!("⛏️ 创建完成");
            continue;
        }

        if data[0] == "list" {
            for b in &bc.blocks {
                println!("++++++++++++++++++++++++++++++++++++++++++++++++++++");
                println!("{:#?}", b);
            }
            continue;
        }
        println!("❌ 请输入 add 加交易信息添加区块，输入list查看当前链。");
    }
}
