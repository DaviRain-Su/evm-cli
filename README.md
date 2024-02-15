# Evm cli

现在是一个evm cli工具 主要用于和evm的chian进行交互

支持evm chain
- [Bera chain](https://docs.berachain.com/developers/index)

## Bera Chain

### Network info

- Network: Berachain Artio
- ChainId: 80085
- Currency: Bera
- Block Explorer URL:
    - [Offical](https://artio.beratrail.io/)
    - [Lore Explorer](https://berascan.ai/)
    - [DexGuru](https://bera-test.dex.guru/)
- Third RPC
    - [offical](https://artio.rpc.berachain.com/)
    - [Ankr](https://rpc.ankr.com/berachain_testnet)



```bash
evm-cli 0.1.0

USAGE:
    evm-cli <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    auto         auto generate config.toml file to ~/.config/evm-cli/config.toml
    balance      Display single or multi Wallet Balance
    bear         bera module
    deploy-sc    Now support deploy incrementer smart contract
    generator    generate new keypair, support single or multi keypair generate and load single keypair
    help         Prints this message or the help of the given subcommand(s)
    transfer     Support batch transfer
```

```bash
evm-cli-bera 0.1.0
bera module

USAGE:
    evm-cli bera <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    bank      Bera Chain Bank module
    dex       Bera Chain Dex module
    epoch     Bera Chain epoch module
    help      Prints this message or the help of the given subcommand(s)
    honey     Bera chain Honey module
    nft       Bera chain batch buy nft
    w-bera    Bera chain Wbera module
```

## 生成账户

## 生成单个账户

```bash
cargo run generator single generator
```

## 导入单个账户

```bash
cargo run generator single load your-private-key
```

## 打印账户资产

### 打印多账户的资产

```bash
cargo run balance multi --chain-id 80085 --file-name your-multi-wallet
```

### 打印单个账户

```bash
cargo run balance single 80085
```

## 批量部署合约

```bash
cargo run deploy-sc --chain-id 80085 --file-name your-file-name
```

## 批量转账


### 从一个账户到多个账户

```bash
cargo run transfer --is-one-to-more --amount 0.1 --chain-id 80085 --file-name davirain-10000
```

## 交互bera


## 批量转移honey

```bash
cargo run bear honey --chain-id 80085 --file-name davirain-10000
```
