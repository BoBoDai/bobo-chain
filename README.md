# bobo-chain （2025/03）

## 1 什么是bobo链

bobo链是一个Rust实现的区块链demo，为学习和深入了解区块链原理和Rust数据结构而生。

## 2 核心功能

- 区块生成：创链同时生成初始区块，初始区块不需要PoW（Proof of Work）证明。
- 挖矿机制：在创建块的时候通过nonce实现工作量证明
- 链验证：保证链上前一块hash匹配
- 存储：持久化当前链

## 3 实现方式

区块结构包含区块头，区块hash和区块数据

```
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,
}
```

其中区块头包含 数据唯一hash，上一个块的hash，挖矿难度和工作量证明

```
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
```

通过随机前n位为0的hash，实现工作量证明。

```
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
```

## 4 基本操作

1. 启动程序

```bash
  cargo run
```
会看到提示`请输入指令`

2. 退出程序

看到`请输入指令`提示后输入

```
exit
```

3. 添加新区块

看到`请输入指令`提示后输入
```
add {交易信息}
```
例如
```
add aid -> bid 500 sol
```

4. 查看当前链

```
list
```