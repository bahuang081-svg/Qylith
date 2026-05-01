# 快速上手指南 | Quick Start Guide

> 让有 Rust 基础的开发者在 **30 分钟内**启动 Qylith 节点

## 📋 目录

1. [环境准备](#1-环境准备)
2. [编译和运行本地节点](#2-编译和运行本地节点)
3. [连接到 Qylith 测试网](#3-连接到qylith测试网)
4. [创建第一个 FALCON 签名账户](#4-创建第一个falcon签名账户)
5. [发送第一笔交易](#5-发送第一笔交易)

---

## 1. 环境准备 | Environment Setup

### 1.1 安装 Rust

Qylith 基于 Substrate，需要 Rust 工具链。

```bash
# Install Rust via rustup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the prompts, select "1) Proceed with standard installation"
source ~/.cargo/env

# Verify installation
rustc --version
# Output: rustc 1.75.0 (version >= 1.70 required)

cargo --version
# Output: cargo 1.75.0
```

### 1.2 配置 Rust 工具链

```bash
# Add WebAssembly target for Substrate
rustup target add wasm32-unknown-unknown --toolchain stable

# Add nightly toolchain for Substrate development
rustup toolchain install nightly-2024-01-01 --target wasm32-unknown-unknown
rustup default nightly-2024-01-01
```

### 1.3 安装依赖 (Ubuntu/Debian)

```bash
# Install required packages
sudo apt update
sudo apt install -y \
    build-essential \
    git \
    clang \
    curl \
    libssl-dev \
    llvm \
    libudev-dev \
    make \
    protobuf-compiler \
    pkg-config
```

### 1.4 安装 Node.js (可选，用于前端开发)

```bash
# Using nvm (recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
source ~/.bashrc

nvm install 20
nvm use 20

node --version
# Output: v20.x.x
```

### 1.5 验证环境

```bash
# Run this script to verify all dependencies
rustc --version && cargo --version && rustup show
```

---

## 2. 编译和运行本地节点

### 2.1 克隆 Qylith 代码库

```bash
# Clone the Qylith node template
git clone https://github.com/qylith-chain/qylith-node.git
cd qylith-node

# Or use the Substrate node template as base
git clone https://github.com/substrate-developer-hub/substrate-node-template.git qylith-node
cd qylith-node
```

### 2.2 编译节点

```bash
# First-time build (takes 15-30 minutes)
cargo build --release

# Subsequent builds (incremental, faster)
cargo build

# Build with specific features
cargo build --release --features falcon-signatures,aem
```

### 2.3 运行本地开发节点

```bash
# Start a local development node (single node, no peers)
cargo run --release -- \
    --dev \
    --tmp \
    --rpc-port 9933 \
    --ws-port 9944

# Or with custom chain spec
cargo run --release -- \
    --dev \
    --chain local \
    --alice \
    --rpc-port 9933 \
    --ws-port 9944
```

### 2.4 验证节点运行

打开第二个终端：

```bash
# Check if node is running via HTTP RPC
curl -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"system_chain","params":[],"id":1}' \
    http://localhost:9933

# Expected response:
# {"jsonrpc":"2.0","result":"Qylith Dev","id":1}
```

或者使用 Polkadot.js Apps 连接：

1. 打开浏览器访问 https://polkadot.js.org/apps/
2. 点击左上角设置，选择 "Local Node"
3. 输入 `ws://localhost:9944`
4. 点击切换

---

## 3. 连接到 Qylith 测试网

### 3.1 配置测试网节点

```bash
# Start a node connected to testnet
cargo run --release -- \
    --chain qylith-testnet \
    --bootnodes /ip4/ Bootnode IP /tcp/30333/p2p/ PeerId \
    --rpc-port 9933 \
    --ws-port 9944 \
    --name "My Testnet Node"
```

### 3.2 测试网配置

```toml
# qylith-testnet-chainspec.json structure
{
  "name": "Qylith Testnet",
  "id": "qylith_testnet",
  "chainType": "Live",
  "bootNodes": [
    "/ip4/1.2.3.4/tcp/30333/p2p/QmTestPeerId..."
  ],
  "telemetryEndpoints": [
    "/dns/telemetry.qylith.io/tcp/443/x-parity-ws/..."
  ],
  "protocolId": "qylith",
  "properties": {
    "ss58Format": 42,
    "tokenDecimals": 12,
    "tokenSymbol": "QTX",
    "isFalconEnabled": true,
    "isAemEnabled": true
  }
}
```

### 3.3 验证网络连接

```bash
# Check connected peers
curl -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"system_peers","params":[],"id":1}' \
    http://localhost:9933

# Check network health
curl -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
    http://localhost:9933
```

---

## 4. 创建第一个 FALCON 签名账户

### 4.1 使用 Polkadot.js 创建账户

1. 打开 https://polkadot.js.org/apps/
2. 进入 **Accounts** 页面
3. 点击 **Add Account**
4. 选择 **FALCON** 作为密钥类型
5. 保存种子短语和 keystore 文件

### 4.2 使用 Rust SDK 创建账户

```rust
// examples/create_falcon_account.rs
use qylith_crypto::{FalconKeypair, SecretKey};

// Generate a new FALCON keypair
let keypair = FalconKeypair::generate();
let secret_key = keypair.secret_key();
let public_key = keypair.public_key();

// Display account address (using SS58 format)
let ss58_address = public_key.to_ss58_address(42); // 42 = Qylith SS58 prefix
println!("FALCON Address: {}", ss58_address);

// Save secret key securely (DO NOT commit to git!)
let secret_phrase = secret_key.to_mnemonic();
println!("Secret Phrase: {}", secret_phrase);
```

### 4.3 密钥格式说明

| 格式 | 描述 | 示例 |
|------|------|------|
| **Hex** | 原始公钥 | `0x8a5d2f9c...` |
| **SS58** | Qylith 地址格式 | `Qtx123...abc` |
| **Mnemonic** | 助记词 (24 words) | `abandon...zoo` |

### 4.4 从种子恢复账户

```rust
// Restore account from mnemonic
let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
let keypair = FalconKeypair::from_mnemonic(mnemonic);
let address = keypair.public_key().to_ss58_address(42);
```

---

## 5. 发送第一笔交易

### 5.1 使用 Polkadot.js UI

1. 连接节点后，进入 **Accounts**
2. 选择发送账户，点击 **Send**
3. 输入接收地址和金额
4. 点击 **Make Transfer**
5. 签名并提交交易

### 5.2 使用 JavaScript SDK

```javascript
// examples/send_transaction.js
const { QylithSDK } = require('qylith-sdk-js');

async function main() {
  const sdk = new QylithSDK({
    provider: 'ws://localhost:9944',
    // For testnet: provider: 'wss://testnet.qylith.io'
  });

  // Connect to node
  await sdk.connect();

  // Create account from mnemonic
  const account = sdk.createAccount(
    'abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about'
  );

  // Get balance before transfer
  const balanceBefore = await sdk.getBalance(account.address);
  console.log('Balance before:', balanceBefore.toString());

  // Send transfer transaction
  const txHash = await sdk.transfer(
    account,                    // Sender
    'Qtx5XYZ...123',            // Recipient
    BigInt(1_000_000_000_000)   // 1 QTX (12 decimals)
  );

  console.log('Transaction hash:', txHash);

  // Wait for finalization
  const result = await txHash.waitForFinalized();
  console.log('Block:', result.block.toString());
  console.log('Status:', result.status);

  // Get balance after transfer
  const balanceAfter = await sdk.getBalance(account.address);
  console.log('Balance after:', balanceAfter.toString());

  await sdk.disconnect();
}

main().catch(console.error);
```

### 5.3 使用 JSON-RPC API

```bash
# Prepare transaction (using Polkadot.js API format)
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc": "2.0",
        "method": "author_submitExtrinsic",
        "params": ["0x..."],
        "id": 1
    }' \
    http://localhost:9933
```

### 5.4 使用 Rust SDK

```rust
// examples/send_transfer.rs
use qylith_sdk::{Sdk, AccountId, Balance};
use sp_keyring::AccountKeyring;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdk = Sdk::new("ws://localhost:9944").await?;

    // Alice is pre-funded in dev mode
    let alice = AccountKeyring::Alice.pair();
    let bob = AccountKeyring::Bob.to_account_id();

    // Current balance
    let balance = sdk.get_balance(&alice.public_key()).await?;
    println!("Alice balance: {:?}", balance);

    // Build transfer extrinsic
    let transfer = qylith_sdk::balances::Transfer {
        dest: bob.clone(),
        value: 1_000_000_000_000u128, // 1 QTX
    };

    // Sign and submit
    let hash = sdk.submit extrinsic(transfer, &alice).await?;
    println!("Transaction hash: {:?}", hash);

    // Wait for block inclusion
    let result = sdk.wait_for_finalized(hash).await?;
    println!("Included in block: {:?}", result.block_hash);

    Ok(())
}
```

---

## 🎯 下一步

恭喜完成快速上手！现在你可以：

- 📖 深入学习 [FALCON 签名集成](./falcon-integration.md)
- 🤖 探索 [AEM 开发指南](./aem-development.md)
- 📜 开发 [智能合约](./smart-contracts.md)
- 🔌 查阅 [API 参考](./api-reference.md)

## 🆘 常见问题

### Q: 编译失败，显示 "linker cc not found"
```bash
sudo apt install build-essential
```

### Q: `wasm32-unknown-unknown` target 错误
```bash
rustup target add wasm32-unknown-unknown --toolchain nightly
```

### Q: 节点启动后无区块产出
确认使用 `--dev` 模式或正确配置了测试网 bootnodes。

### Q: 如何查看节点日志？
```bash
# Enable detailed logging
RUST_LOG=debug cargo run --release -- --dev -lruntime=debug
```
