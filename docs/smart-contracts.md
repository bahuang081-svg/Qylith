# 智能合约开发指南 | Smart Contract Development Guide

> 在 Qylith 上开发抗量子智能合约

## 📋 目录

1. [概述](#1-概述)
2. [ink! 智能合约开发](#2-ink-智能合约开发)
3. [Solidity 兼容层](#3-solidity-兼容层)
4. [PQC 预编译合约](#4-pqc-预编译合约)
5. [合约部署和测试](#5-合约部署和测试)

---

## 1. 概述

### 1.1 合约开发选项

| 选项 | 语言 | 特点 | 适用场景 |
|------|------|------|---------|
| **ink!** | Rust | 链原生，高效安全 | 核心逻辑，高安全需求 |
| **Solidity** | Solidity | Ethereum 兼容 | 迁移现有合约 |
| **Precompiles** | Rust | 内置 PQC 操作 | 签名验证，密码学操作 |

### 1.2 架构图

```
┌─────────────────────────────────────────────────────────────────┐
│                  Qylith Smart Contract Stack                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────────┐  ┌──────────────────┐  ┌────────────────┐  │
│  │   ink! Contracts │  │ Solidity (EVM)  │  │  Precompiles   │  │
│  │   (Rust)         │  │ (via Solang)   │  │  (Native)      │  │
│  └────────┬─────────┘  └────────┬─────────┘  └───────┬────────┘  │
│           │                      │                     │          │
│           ▼                      ▼                     ▼          │
│  ┌────────────────────────────────────────────────────────────────┐
│  │                    Contract Runtime                           │
│  │  ┌──────────────┐  ┌──────────────┐  ┌────────────────────┐   │
│  │  │ Contracts    │  │ EVM          │  │  PQC Precompile    │   │
│  │  │ Pallet       │  │ Pallet       │  │  Interface         │   │
│  │  └──────────────┘  └──────────────┘  └────────────────────┘   │
│  └────────────────────────────────────────────────────────────────┘
│                                                                 │
│  ┌────────────────────────────────────────────────────────────────┐
│  │                    Substrate Runtime                          │
│  └────────────────────────────────────────────────────────────────┘
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. ink! 智能合约开发

### 2.1 环境设置

```bash
# Install contract toolchain
cargo install cargo-contract --force
cargo install cargo-chef --locked

# Verify installation
cargo contract --version
# Output: cargo-contract 3.0.0
```

### 2.2 创建 ink! 项目

```bash
# Create new contract project
cargo contract new qtx_token

# Project structure
qtx_token/
├── lib.rs           # Contract logic
├── Cargo.toml       # Dependencies
└── tests/           # Integration tests
```

### 2.3 基础 ERC-20 合约示例

```rust
// contracts/qtx_token/lib.rs

#![cfg_attr(not(feature = "std"), no_std)]

use ink::{
    env::call::{build_call, ExecutionInput, Selector},
    prelude::string::String,
    prelude::vec::Vec,
    storage::Mapping,
    Environment,
};

#[ink::contract]
mod qtx_token {
    use super::*;

    /// Token decimals
    const DECIMALS: u8 = 12;

    #[derive(scale::Decode, scale::Encode, Clone, Debug, PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance,
        ZeroAddress,
        Overflow,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[ink(storage)]
    pub struct QtxToken {
        /// Total token supply
        total_supply: Balance,
        /// Token balances
        balances: Mapping<AccountId, Balance>,
        /// Allowance mapping
        allowances: Mapping<(AccountId, AccountId), Balance>,
        /// Token name
        name: String,
        /// Token symbol
        symbol: String,
    }

    impl QtxToken {
        /// Constructor - initialize contract with initial supply
        #[ink(constructor)]
        pub fn new(initial_supply: Balance, name: String, symbol: String) -> Self {
            let caller = Self::env().caller();
            let mut balances = Mapping::new();
            balances.insert(caller, &initial_supply);

            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: initial_supply,
            });

            Self {
                total_supply: initial_supply,
                balances,
                allowances: Mapping::default(),
                name,
                symbol,
            }
        }

        /// Get token name
        #[ink(message)]
        pub fn name(&self) -> String {
            self.name.clone()
        }

        /// Get token symbol
        #[ink(message)]
        pub fn symbol(&self) -> String {
            self.symbol.clone()
        }

        /// Get token decimals
        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            DECIMALS
        }

        /// Get total supply
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Get balance of an account
        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> Balance {
            self.balances.get(&account).unwrap_or(0)
        }

        /// Get allowance
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get(&(owner, spender)).unwrap_or(0)
        }

        /// Transfer tokens to another account
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), Error> {
            self.transfer_impl(self.env().caller(), to, value)
        }

        /// Transfer tokens with direct implementation
        fn transfer_impl(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<(), Error> {
            if from == to {
                return Ok(());
            }

            let from_balance = self.balances.get(&from).unwrap_or(0);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            // Update balances
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balances.get(&to).unwrap_or(0);
            self.balances.insert(to, &(to_balance + value));

            // Emit event
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });

            Ok(())
        }

        /// Approve spender to use tokens
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), Error> {
            let owner = self.env().caller();
            self.allowances.insert(&(owner, spender), &value);

            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });

            Ok(())
        }

        /// Transfer from using allowance
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            let allowance = self.allowances.get(&(from, caller)).unwrap_or(0);

            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }

            self.transfer_impl(from, to, value)?;
            self.allowances.insert(&(from, caller), &(allowance - value));

            Ok(())
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test;

        #[ink::test]
        fn new_works() {
            let token = QtxToken::new(1000, "QTX".into(), "QTX".into());
            assert_eq!(token.total_supply(), 1000);
            assert_eq!(token.balance_of(AccountId::from([0x1; 32])), 1000);
        }

        #[ink::test]
        fn transfer_works() {
            let mut token = QtxToken::new(1000, "QTX".into(), "QTX".into());
            let alice = AccountId::from([0x1; 32]);
            let bob = AccountId::from([0x2; 32]);

            set_caller(alice);
            token.transfer(bob, 100).unwrap();

            assert_eq!(token.balance_of(alice), 900);
            assert_eq!(token.balance_of(bob), 100);
        }

        #[ink::test]
        fn transfer_fails_insufficient_balance() {
            let mut token = QtxToken::new(100, "QTX".into(), "QTX".into());
            let alice = AccountId::from([0x1; 32]);
            let bob = AccountId::from([0x2; 32]);

            set_caller(alice);
            let result = token.transfer(bob, 200);
            assert!(result.is_err());
        }

        fn set_caller(account: AccountId) {
            ink::env::test::set_caller(account);
        }
    }
}
```

### 2.4 编译合约

```bash
# Build contract
cargo contract build

# Output:
#   Building  contract qtx_token
#   Generating metadata ...
#   Assembling WASM binary ...
#   
#  => qtx_token.contract  (canonical)
#  => qtx_token.wasm      (optimized)
#  => metadata.json       (ABIs)
```

---

## 3. Solidity 兼容层

### 3.1 概述

Qylith 通过 [Solang](https://github.com/hyperledger-labs/solang) 编译器支持 Solidity 合约：

- 语法完全兼容 Solidity 0.8.x
- EVM 指令支持
- 转换为 WASM 运行在 Substrate Contracts Pallet

### 3.2 Solidity 合约示例

```solidity
// contracts/QTXToken.sol
// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

/// @title QTXToken - Example Solidity token on Qylith
contract QTXToken {
    string public name = "QTX Token";
    string public symbol = "QTX";
    uint8 public decimals = 12;
    uint256 public totalSupply;
    
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    
    constructor(uint256 _initialSupply) {
        totalSupply = _initialSupply * 10 ** uint256(decimals);
        balanceOf[msg.sender] = totalSupply;
    }
    
    function transfer(address _to, uint256 _value) public returns (bool success) {
        require(balanceOf[msg.sender] >= _value, "Insufficient balance");
        
        balanceOf[msg.sender] -= _value;
        balanceOf[_to] += _value;
        
        emit Transfer(msg.sender, _to, _value);
        return true;
    }
    
    function approve(address _spender, uint256 _value) public returns (bool success) {
        allowance[msg.sender][_spender] = _value;
        emit Approval(msg.sender, _spender, _value);
        return true;
    }
    
    function transferFrom(address _from, address _to, uint256 _value) public returns (bool success) {
        require(balanceOf[_from] >= _value, "Insufficient balance");
        require(allowance[_from][msg.sender] >= _value, "Insufficient allowance");
        
        balanceOf[_from] -= _value;
        balanceOf[_to] += _value;
        allowance[_from][msg.sender] -= _value;
        
        emit Transfer(_from, _to, _value);
        return true;
    }
}
```

### 3.3 编译 Solidity

```bash
# Install Solang
cargo install solang

# Compile Solidity to WASM
solang compile --target substrate contracts/QTXToken.sol -o output/

# Output:
#   info: Contract QTXToken compiled successfully
#   => QTXToken.contract
#   => QTXToken.json (metadata)
```

---

## 4. PQC 预编译合约

### 4.1 预编译合约列表

| 地址 | 合约 | 功能 |
|------|------|------|
| `0x00000000000000000000000000000001` | FALCON | FALCON-1024 签名验证 |
| `0x00000000000000000000000000000002` | FALCON_KEYGEN | FALCON 密钥生成 |
| `0x00000000000000000000000000000003` | HYBRID_SIGN | 混合签名验证 |

### 4.2 FALCON 预编译接口

```rust
// src/precompiles/falcon.rs

/// FALCON signature verification precompile
/// 
/// Input format (ABI encoded):
///   - bytes: public_key (FALCON public key, 1797 bytes)
///   - bytes: message (any length)
///   - bytes: signature (66 bytes)
/// 
/// Returns: bool (true if valid)
pub fn verify_falcon_signature(
    public_key: &[u8],
    message: &[u8],
    signature: &[u8],
) -> Result<bool, PrecompileError> {
    // Validate public key length
    if public_key.len() != 1797 {
        return Err(PrecompileError::InputTooShort);
    }
    
    // Validate signature length
    if signature.len() != 66 {
        return Err(PrecompileError::InputTooShort);
    }
    
    // Verify signature
    falcon::verify(public_key, message, signature)
}

/// Generate FALCON keypair on-chain (for specific use cases)
/// 
/// Returns: (public_key, signature_of_challenge)
pub fn generate_falcon_keypair(
    challenge: &[u8],
) -> Result<(Vec<u8>, Vec<u8>), PrecompileError> {
    // This is expensive - use sparingly
    let keypair = falcon::Keypair::generate();
    let signature = keypair.sign(challenge);
    
    Ok((keypair.public_key().to_bytes(), signature.to_bytes()))
}
```

### 4.3 在合约中使用 PQC

```rust
// contracts/pqc_verifier/lib.rs

#![cfg_attr(not(feature = "std"), no_std)]

use ink::prelude::vec::Vec;
use ink::env::call::build_call;

#[ink::contract]
mod pqc_verifier {
    /// Precompile addresses
    const FALCON_PRECOMPILE: [u8; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ];

    #[ink(storage)]
    pub struct PqcVerifier {
        /// Registered public keys (key hash -> public key)
        registered_keys: Mapping<[u8; 32], Vec<u8>>,
    }

    impl PqcVerifier {
        /// Register a FALCON public key
        #[ink(message)]
        pub fn register_key(&mut self, public_key: Vec<u8>, key_hash: [u8; 32]) {
            // Validate key length
            assert_eq!(public_key.len(), 1797, "Invalid FALCON public key length");
            
            self.registered_keys.insert(key_hash, &public_key);
        }

        /// Verify a FALCON signature using precompile
        #[ink(message)]
        pub fn verify_signature(
            &self,
            key_hash: [u8; 32],
            message: Vec<u8>,
            signature: Vec<u8>,
        ) -> bool {
            // Get registered public key
            let public_key = match self.registered_keys.get(&key_hash) {
                Some(pk) => pk,
                None => return false,
            };

            // Call FALCON precompile
            let result = build_call::<Environment>()
                .call(FALCON_PRECOMPILE.into())
                .gas_limit(50000)
                .exec_input(ExecutionInput::new(Selector::new("verify"))
                    .push_arg(public_key)
                    .push_arg(message)
                    .push_arg(signature)
                )
                .returns::<bool>()
                .invoke();

            result.unwrap_or(false)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test;

        #[ink::test]
        fn verify_works() {
            // This would require proper FALCON setup in test environment
        }
    }
}
```

---

## 5. 合约部署和测试

### 5.1 使用 Polkadot.js UI 部署

1. 打开 https://polkadot.js.org/apps/
2. 进入 **Developer** → **Contracts**
3. 点击 **Upload & Deploy Code**
4. 上传 `.contract` 文件
5. 设置初始参数
6. 点击 **Deploy**

### 5.2 使用 Rust SDK 部署

```rust
// examples/deploy_contract.rs
use qylith_contracts::{CodeHash, Contract};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdk = Sdk::new("ws://localhost:9944").await?;
    
    // Load contract artifacts
    let contract_code = std::fs::read("path/to/qtx_token.contract")?;
    let metadata = std::fs::read("path/to/metadata.json")?;
    
    // Upload contract code
    println!("Uploading contract code...");
    let upload_tx = sdk.api.tx.contracts.upload_code(contract_code);
    let code_stored = upload_tx.sign_and_submit(&alice).await?;
    let code_hash = code_stored.code_hash;
    
    println!("Contract code uploaded: {:?}", code_hash);
    
    // Deploy contract
    println!("Instantiating contract...");
    let instantiate_tx = sdk.api.tx.contracts.instantiate(
        code_hash,                           // code hash
        0u128,                               // endowment
        500_000_000_000u64,                  // gas limit
        None,                                // storage deposit limit
        QtxToken::new(1_000_000_000_000u128, // constructor args
            "QTX Token".into(), 
            "QTX".into()
        ).encode(),
    );
    
    let contract_deployed = instantiate_tx.sign_and_submit(&alice).await?;
    let contract_address = contract_deployed.account_id;
    
    println!("Contract deployed at: {:?}", contract_address);
    
    // Interact with contract
    let result = sdk.api.query.contracts.contract_info(contract_address).await?;
    println!("Contract info: {:?}", result);
    
    Ok(())
}
```

### 5.3 使用 JavaScript SDK 部署

```typescript
// examples/deploy_contract.js
const { QylithSDK } = require('qylith-sdk');
const { readFileSync } = require('fs');

async function main() {
  const sdk = new QylithSDK({
    provider: 'ws://localhost:9944',
  });

  await sdk.connect();
  sdk.setAccount(sdk.createAccount(mnemonic));

  // Read contract artifacts
  const contractFile = JSON.parse(
    readFileSync('./qtx_token.contract', 'utf8')
  );

  console.log('Uploading contract code...');
  const codeTx = sdk.api.tx.contracts.uploadCode(
    contractFile.source.wasm
  );
  await codeTx.signAndSend(sdk.account);
  
  // Wait for code to be stored
  const codeHash = codeTx.hash;
  console.log('Code hash:', codeHash.toHex());

  console.log('Instantiating contract...');
  
  // Encode constructor arguments
  const constructorArgs = sdk.api.tx.contracts.constructor(
    'new',                    // constructor name
    1_000_000_000_000n,       // initial supply
    'QTX Token',              // name
    'QTX'                     // symbol
  );

  const instantiateTx = sdk.api.tx.contracts.instantiateWithCode(
    0n,                       // endowment
    500_000_000_000n,        // gas limit
    null,                     // storage deposit limit
    contractFile.source.wasm, // contract code
    constructorArgs           // constructor input
  );

  const result = await instantiateTx.signAndSend(sdk.account);
  
  if (result.status.isInBlock) {
    // Extract contract address from events
    const event = result.events.find(e => 
      e.event.method === 'Instantiated'
    );
    
    const contractAddress = event.event.data[0];
    console.log('Contract deployed at:', contractAddress.toString());
  }

  await sdk.disconnect();
}

main().catch(console.error);
```

### 5.4 单元测试

```bash
# Run ink! contract tests
cargo contract test

# Output:
#    Running 4 tests for qtx_token
#    test new_works ... ok
#    test transfer_works ... ok
#    test transfer_fails_insufficient_balance ... ok
#    test approve_works ... ok
```

### 5.5 端到端测试

```bash
# Start local node (in background)
cargo run --release -- --dev --rpc-port 9933 &
sleep 30

# Run integration tests
cargo contract runtime-storage-ring
cargo contract test --manifest-path integration-tests/Cargo.toml

# Stop node
pkill -f qylith-node
```

---

## 📚 相关资源

- [ink! 官方文档](https://use.ink/)
- [Solang 编译器](https://github.com/hyperledger-labs/solang)
- [Polkadot.js Contracts](https://polkadot.js.org/apps/#/contracts)
- [快速开始](../quick-start.md)
