# FALCON 签名集成指南 | FALCON Signature Integration Guide

> FALCON-1024: 基于格密码学的抗量子数字签名

## 📋 目录

1. [FALCON 概述](#1-falcon-概述)
2. [密钥生成](#2-密钥生成)
3. [签名和验证流程](#3-签名和验证流程)
4. [从 ECDSA 迁移到 FALCON](#4-从-ecdsa-迁移到-falcon)
5. [混合签名使用场景](#5-混合签名使用场景)
6. [代码示例](#6-代码示例)

---

## 1. FALCON 概述

### 1.1 什么是 FALCON？

FALCON (Fast Fourier Lattice-based Compact Signatures over NTRU) 是一种基于格密码学的数字签名算法，具有以下特性：

| 特性 | 描述 |
|------|------|
| **量子安全** | 无法被量子计算机破解 |
| **签名紧凑** | ~666 字节签名（比 SPHINCS+ 更小） |
| **NIST 标准** | 后量子密码学标准化候选算法 |
| **高性能** | 签名生成和验证速度快 |

### 1.2 FALCON vs ECDSA

| 维度 | ECDSA (secp256k1) | FALCON-1024 |
|------|------------------|-------------|
| **安全性基础** | 椭圆曲线离散对数 | 格 (NTRU + FFT) |
| **量子攻击** | ❌ 易受攻击 | ✅ 抗量子 |
| **公钥大小** | 33 bytes | 1,797 bytes |
| **签名大小** | ~72 bytes | ~666 bytes |
| **密钥大小** | 32 bytes | ~2,048 bytes |

### 1.3 Qylith 中的 FALCON

```rust
// Qylith runtime pallet configuration
pub trait Config: frame_system::Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as Frame>::RuntimeEvent>;
    
    // FALCON signature scheme
    type FalconSignature: Verify<Signer = Self::AccountId>
        + SignedPayload<Signature>
        + transactions::Payload;
}
```

---

## 2. 密钥生成

### 2.1 Rust 中的密钥生成

```rust
// src/crypto/falcon.rs
use qylith_crypto::{FalconKeypair, SecretKey, PublicKey, Signature};

/// Generate a new FALCON-1024 keypair
/// 
/// # Security Notes
/// - Secret key MUST be stored securely
/// - Use hardware security modules (HSM) for production
/// - Never log or transmit secret keys in plaintext
pub fn generate_keypair() -> FalconKeypair {
    FalconKeypair::generate()
}

/// Generate keypair from entropy source
pub fn generate_keypair_with_entropy(entropy: &[u8]) -> FalconKeypair {
    FalconKeypair::generate_from_entropy(entropy)
}

/// Derive keypair from mnemonic phrase (BIP39 compatible)
pub fn keypair_from_mnemonic(mnemonic: &str) -> FalconKeypair {
    FalconKeypair::from_mnemonic(mnemonic)
}

/// Generate keypair from seed (raw bytes)
pub fn keypair_from_seed(seed: &[u8; 32]) -> FalconKeypair {
    FalconKeypair::from_seed(seed)
}
```

### 2.2 密钥存储格式

```rust
// src/crypto/keys.rs
use serde::{Deserialize, Serialize};

/// FALCON key storage format (JSON keystore)
#[derive(Serialize, Deserialize)]
pub struct Keystore {
    /// Keystore version
    pub version: u32,
    /// Crypto scheme identifier
    pub crypto: CryptoScheme,
    /// Secret key data (encrypted)
    pub secret: EncryptedKey,
    /// Public key (unencrypted)
    pub public: PublicKey,
}

#[derive(Serialize, Deserialize)]
pub struct CryptoScheme {
    /// Cipher: "aes-256-gcm" | "chacha20-poly1305"
    pub cipher: String,
    /// Key derivation: "pbkdf2" | "scrypt"
    pub kdf: KdfScheme,
    /// Cipher parameters
    pub cipherparams: CipherParams,
}

#[derive(Serialize, Deserialize)]
pub struct KdfScheme {
    pub function: String,       // "pbkdf2"
    pub params: Pbkdf2Params,
    pub salt: String,            // Hex encoded
}

#[derive(Serialize, Deserialize)]
pub struct Pbkdf2Params {
    pub c: u32,                  // Iteration count: 2048 minimum
    pub dklen: u32,              // Derived key length: 32
    pub prf: String,             // "hmac-sha512"
}
```

### 2.3 密钥导入/导出

```rust
// src/crypto/import_export.rs
use qylith_crypto::{FalconKeypair, SecretKey, PublicKey};

/// Export public key to SS58 address
pub fn public_key_to_ss58(public_key: &PublicKey, ss58_prefix: u16) -> String {
    public_key.to_ss58_address(ss58_prefix)
}

/// Import from hex-encoded public key
pub fn public_key_from_hex(hex: &str) -> Result<PublicKey, CryptoError> {
    PublicKey::from_hex(hex)
}

/// Export secret key (WARNING: handle with extreme care)
pub fn export_secret_key(keypair: &FalconKeypair) -> SecretKey {
    keypair.secret_key().clone()
}

/// Import from hex-encoded secret key
pub fn secret_key_from_hex(hex: &str) -> Result<SecretKey, CryptoError> {
    SecretKey::from_hex(hex)
}

/// Export to encrypted keystore (JSON)
pub fn export_keystore(
    keypair: &FalconKeypair,
    password: &str,
) -> Result<String, CryptoError> {
    let keystore = Keystore::new(keypair, password)?;
    serde_json::to_string_pretty(&keystore)
        .map_err(|e| CryptoError::Serialization(e.to_string()))
}
```

---

## 3. 签名和验证流程

### 3.1 基本签名流程

```rust
// src/crypto/sign.rs
use qylith_crypto::{FalconKeypair, Signature, Signer, Verifier};
use sp_core::bounded::BoundedVec;

/// Sign a message with FALCON
/// 
/// # Parameters
/// * `keypair` - The signer's FALCON keypair
/// * `message` - The message to sign (any length)
/// 
/// # Returns
/// * `Signature` - FALCON signature (66 bytes)
pub fn sign_message(keypair: &FalconKeypair, message: &[u8]) -> Signature {
    keypair.sign(message)
}

/// Sign structured data (recommended for transactions)
pub fn sign_payload<P: Encode>(keypair: &FalconKeypair, payload: &P) -> Signature {
    let encoded = payload.encode();
    keypair.sign(&encoded)
}
```

### 3.2 签名验证

```rust
// src/crypto/verify.rs
use qylith_crypto::{PublicKey, Signature, Verify};

/// Verify a signature
/// 
/// # Parameters
/// * `public_key` - The signer's public key
/// * `message` - The original message
/// * `signature` - The signature to verify
/// 
/// # Returns
/// * `bool` - true if valid, false otherwise
pub fn verify_signature(
    public_key: &PublicKey,
    message: &[u8],
    signature: &Signature,
) -> bool {
    Verify::verify(public_key, message, signature)
}

/// Verify multiple signatures (batch verification)
pub fn verify_batch(
    items: &[(PublicKey, Vec<u8>, Signature)],
) -> Vec<bool> {
    items.iter()
        .map(|(pk, msg, sig)| Verify::verify(pk, msg, sig))
        .collect()
}
```

### 3.3 签名流程图

```
┌─────────────────────────────────────────────────────────────────┐
│                      FALCON 签名流程                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  签名者 (Signer)                  验证者 (Verifier)           │
│  ─────────────                   ──────────────               │
│                                                                 │
│  ┌──────────────┐                  ┌──────────────┐            │
│  │ FALCON Keypair│                 │  Public Key  │            │
│  │ ┌──────────┐ │                  │  (from SS58) │            │
│  │ │Secret Key│ │                  └──────────────┘            │
│  │ └──────────┘ │                           ▲                   │
│  │ ┌──────────┐ │                           │                   │
│  │ │Public Key│ │◄─── derive ───────────────┘                   │
│  │ └──────────┘ │                                               │
│  └──────────────┘                                               │
│          │                                                      │
│          ▼                                                      │
│  ┌────────────────┐         Message + Signature                 │
│  │  Sign(Message) │────────────────────────────────────────►   │
│  │                │              (Verify)                       │
│  └────────────────┘                                             │
│          │                                                      │
│          ▼                                                      │
│  ┌────────────────┐                                             │
│  │   Signature    │  66 bytes (FALCON-1024)                     │
│  │   0x8a5d...    │                                             │
│  └────────────────┘                                             │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 4. 从 ECDSA 迁移到 FALCON

### 4.1 迁移策略

```
Phase 1: Dual Support (共存期)
┌────────────────────────────────────────────────────┐
│ • 新账户使用 FALCON                                 │
│ • 旧账户继续使用 ECDSA                              │
│ • 链上支持两种签名类型                               │
└────────────────────────────────────────────────────┘
                        ▼
Phase 2: Migration (迁移期)
┌────────────────────────────────────────────────────┐
│ • 提供 ECDSA → FALCON 迁移工具                      │
│ • 激励用户迁移                                      │
│ • 监控迁移进度                                      │
└────────────────────────────────────────────────────┘
                        ▼
Phase 3: FALCON Only (FALCON 优先)
┌────────────────────────────────────────────────────┐
│ • 新增账户强制使用 FALCON                            │
│ • 区块签名迁移到 FALCON                             │
│ • ECDSA 可选（向后兼容）                            │
└────────────────────────────────────────────────────┘
```

### 4.2 迁移工具

```rust
// src/migrations/ecdsa_to_falcon.rs
use qylith_crypto::{EcdsaKeypair, FalconKeypair, Migrator};

/// Migrate ECDSA account to FALCON
/// 
/// This creates a new FALCON account with the same underlying seed
/// as the ECDSA account, allowing seamless transition.
pub struct EcdsaToFalconMigrator;

impl EcdsaToFalconMigrator {
    /// Create migrator with source ECDSA keypair
    pub fn new(ecdsa_keypair: EcdsaKeypair) -> Self {
        Self { ecdsa_keypair }
    }

    /// Generate FALCON account with same seed
    pub fn migrate(&self) -> Result<FalconKeypair, MigrationError> {
        // Derive seed from ECDSA secret key
        let seed = self.ecdsa_keypair.secret_key().to_seed();
        
        // Generate FALCON keypair from same seed
        let falcon_keypair = FalconKeypair::from_seed(&seed);
        
        Ok(falcon_keypair)
    }

    /// Migrate with new key (different from ECDSA seed)
    pub fn migrate_with_new_key(&self) -> FalconKeypair {
        FalconKeypair::generate()
    }
}

/// Transfer balance and identity from ECDSA to FALCON account
pub async fn complete_migration(
    ecdsa_address: &AccountId,
    falcon_address: &AccountId,
    migrator: &EcdsaToFalconMigrator,
) -> Result<MigrationResult, MigrationError> {
    // 1. Verify ownership (sign message with ECDSA)
    let ownership_proof = migrator.ecdsa_keypair.sign(b"Qylith Migration");
    
    // 2. Submit migration transaction
    let tx = Migration::migrate_account(
        ecdsa_address.clone(),
        falcon_address.clone(),
        ownership_proof,
    );
    
    // 3. Wait for confirmation
    let result = submit_and_wait(tx).await?;
    
    Ok(MigrationResult {
        old_address: ecdsa_address.clone(),
        new_address: falcon_address.clone(),
        block: result.block_number,
    })
}
```

### 4.3 迁移配置

```rust
// src/pallets/migration/src/lib.rs
pub trait Config: frame_system::Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as Frame>::RuntimeEvent>;
    
    /// Migration deposit amount (prevent spam)
    type MigrationDeposit: Get<BalanceOf<Self>>;
    
    /// Maximum ECDSA accounts per FALCON account
    type MaxAccountsPerUser: Get<u32>;
}

/// Genesis configuration for migration pallet
#[pallet::genesis_config]
pub struct GenesisConfig {
    /// Pre-registered ECDSA accounts for migration
    pub pre_migrations: Vec<(AccountId, EcdsaPublic)>,
}
```

---

## 5. 混合签名使用场景

### 5.1 为什么需要混合签名？

| 场景 | 推荐签名 | 原因 |
|------|---------|------|
| **区块签名** | FALCON ✅ | 抗量子，保护链上安全 |
| **普通转账** | FALCON ✅ | 长期价值存储 |
| **即时交易** | ECDSA | 签名更小，更快验证 |
| **跨链桥** | 双签 | 既要安全又要兼容性 |

### 5.2 双签名验证器

```rust
// src/crypto/hybrid_signature.rs
use qylith_crypto::{EcdsaSignature, FalconSignature, DualSignature};

/// Hybrid signature that supports both ECDSA and FALCON
/// Used for backward compatibility and specific use cases
pub enum SignatureType {
    Ecdsa(EcdsaSignature),
    Falcon(FalconSignature),
}

/// Verify hybrid signature
pub fn verify_hybrid(
    public_keys: &(PublicKey, EcdsaPublic),
    message: &[u8],
    signature: &SignatureType,
) -> bool {
    match signature {
        SignatureType::Ecdsa(sig) => {
            // Verify with ECDSA key
            verify_ecdsa(&public_keys.1, message, sig)
        }
        SignatureType::Falcon(sig) => {
            // Verify with FALCON key
            verify_falcon(&public_keys.0, message, sig)
        }
    }
}

/// Multi-signature requiring both ECDSA and FALCON signatures
pub struct DualSigner {
    ecdsa_keypair: EcdsaKeypair,
    falcon_keypair: FalconKeypair,
}

impl DualSigner {
    /// Create dual signer from both keypairs
    pub fn new(ecdsa: EcdsaKeypair, falcon: FalconKeypair) -> Self {
        Self {
            ecdsa_keypair: ecdsa,
            falcon_keypair: falcon,
        }
    }

    /// Create dual signature (both ECDSA and FALCON)
    pub fn sign_dual(&self, message: &[u8]) -> DualSignature {
        DualSignature {
            ecdsa: self.ecdsa_keypair.sign(message),
            falcon: self.falcon_keypair.sign(message),
        }
    }
}

/// Verify dual signature (both must be valid)
pub fn verify_dual_signature(
    ecdsa_pk: &EcdsaPublic,
    falcon_pk: &PublicKey,
    message: &[u8],
    dual_sig: &DualSignature,
) -> bool {
    verify_ecdsa(ecdsa_pk, message, &dual_sig.ecdsa)
        && verify_falcon(falcon_pk, message, &dual_sig.falcon)
}
```

### 5.3 使用场景代码

```rust
// src/bridges/cross_chain.rs
use qylith_crypto::{DualSigner, DualSignature};

/// Cross-chain bridge transaction requiring dual signatures
/// 
/// Security: FALCON protects against quantum attack
/// Compatibility: ECDSA ensures compatibility with other chains
pub struct BridgeTransaction {
    pub from_chain: ChainId,
    pub to_chain: ChainId,
    pub amount: Balance,
    pub recipient: Vec<u8>,
    pub nonce: u64,
}

impl BridgeTransaction {
    /// Sign with dual signature scheme
    pub fn sign(self, signer: &DualSigner) -> SignedBridgeTransaction {
        let tx_hash = self.hash();
        
        SignedBridgeTransaction {
            transaction: self,
            signatures: DualSignature {
                ecdsa: signer.ecdsa_keypair.sign(&tx_hash),
                falcon: signer.falcon_keypair.sign(&tx_hash),
            },
            signature_type: SignatureType::Dual,
        }
    }
}
```

---

## 6. 代码示例

### 6.1 Rust 完整示例

```rust
// examples/falcon_full_example.rs
//! FALCON Signature Integration Example
//! 
//! This example demonstrates:
//! 1. Key generation
//! 2. Signing messages
//! 3. Verifying signatures
//! 4. Batch verification

use qylith_crypto::{
    FalconKeypair, 
    Signature, 
    PublicKey,
    Signer, 
    Verifier,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ============================================
    // Step 1: Generate FALCON Keypair
    // ============================================
    println!("=== FALCON Signature Demo ===\n");
    
    let keypair = FalconKeypair::generate();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();
    
    println!("Keypair Generated:");
    println!("  Secret Key (hex): {}...", 
        hex::encode(secret_key.as_ref()).chars().take(16).collect::<String>());
    println!("  Public Key (hex): {}...", 
        hex::encode(public_key.as_ref()).chars().take(16).collect::<String>());
    println!("  SS58 Address: {}", public_key.to_ss58_address(42));
    println!();

    // ============================================
    // Step 2: Sign Messages
    // ============================================
    let messages = vec![
        b"Hello, Qylith!".to_vec(),
        b"Transfer 100 QTX to Qtx123...".to_vec(),
        b"Deploy contract at 0x...".to_vec(),
    ];

    let signatures: Vec<Signature> = messages
        .iter()
        .map(|msg| keypair.sign(msg))
        .collect();

    println!("Messages Signed:");
    for (i, sig) in signatures.iter().enumerate() {
        println!("  Message {}: {:?}", i + 1, hex::encode(sig.as_ref()));
    }
    println!();

    // ============================================
    // Step 3: Verify Individual Signatures
    // ============================================
    println!("Verification Results:");
    for (i, (msg, sig)) in messages.iter().zip(signatures.iter()).enumerate() {
        let is_valid = Verifier::verify(public_key, msg, sig);
        println!("  Message {}: {}", i + 1, if is_valid { "✓ Valid" } else { "✗ Invalid" });
    }
    println!();

    // ============================================
    // Step 4: Batch Verification (Faster)
    // ============================================
    println!("Batch Verification:");
    let batch_result = signatures.iter()
        .zip(messages.iter())
        .map(|(sig, msg)| (public_key.clone(), msg.clone(), sig.clone()))
        .collect::<Vec<_>>();
    
    let all_valid = batch_result.iter()
        .all(|(pk, msg, sig)| Verifier::verify(pk, msg, sig));
    println!("  All signatures valid: {}", all_valid);
    println!();

    // ============================================
    // Step 5: Mnemonic Recovery
    // ============================================
    let mnemonic = keypair.to_mnemonic();
    println!("Mnemonic Phrase:");
    println!("  {}", mnemonic);
    
    // Recover from mnemonic
    let recovered_keypair = FalconKeypair::from_mnemonic(&mnemonic);
    let recovered_address = recovered_keypair.public_key().to_ss58_address(42);
    
    println!("\nRecovery Test:");
    println!("  Original:  {}", public_key.to_ss58_address(42));
    println!("  Recovered: {}", recovered_address);
    println!("  Match: {}", public_key.to_ss58_address(42) == recovered_address);

    Ok(())
}
```

### 6.2 TypeScript 示例

```typescript
// packages/qylith-sdk/src/falcon.ts
/**
 * FALCON Signature Integration for TypeScript
 */

import { 
  Keypair, 
  PublicKey, 
  SecretKey, 
  Signature,
  signMessage,
  verifySignature,
} from '@qylith/falcon-wasm';

export class FalconAccount {
  private keypair: Keypair;
  public publicKey: PublicKey;
  public address: string;

  /**
   * Generate a new FALCON account
   */
  static generate(): FalconAccount {
    const keypair = Keypair.generate();
    return new FalconAccount(keypair);
  }

  /**
   * Create account from mnemonic phrase
   */
  static fromMnemonic(mnemonic: string): FalconAccount {
    const keypair = Keypair.fromMnemonic(mnemonic);
    return new FalconAccount(keypair);
  }

  /**
   * Create account from seed (32 bytes)
   */
  static fromSeed(seed: Uint8Array): FalconAccount {
    if (seed.length !== 32) {
      throw new Error('Seed must be 32 bytes');
    }
    const keypair = Keypair.fromSeed(seed);
    return new FalconAccount(keypair);
  }

  private constructor(keypair: Keypair) {
    this.keypair = keypair;
    this.publicKey = keypair.publicKey;
    // Convert to SS58 address (Qylith prefix: 42)
    this.address = this.publicKey.toSS58Address(42);
  }

  /**
   * Sign a message
   * @param message - Bytes to sign
   * @returns FALCON signature (66 bytes)
   */
  sign(message: Uint8Array): Signature {
    return this.keypair.sign(message);
  }

  /**
   * Sign a UTF-8 string message
   */
  signMessage(message: string): Signature {
    const encoder = new TextEncoder();
    return this.sign(encoder.encode(message));
  }

  /**
   * Get the mnemonic phrase for this account
   */
  getMnemonic(): string {
    return this.keypair.toMnemonic();
  }

  /**
   * Export secret key (use with caution!)
   */
  exportSecretKey(): Uint8Array {
    return this.keypair.secretKeyBytes();
  }
}

/**
 * Verify a FALCON signature
 */
export function verify(
  publicKey: PublicKey,
  message: Uint8Array,
  signature: Signature
): boolean {
  return verifySignature(publicKey, message, signature);
}

/**
 * Batch verify multiple signatures (more efficient)
 */
export function verifyBatch(
  items: Array<{ publicKey: PublicKey; message: Uint8Array; signature: Signature }>
): boolean[] {
  return items.map(({ publicKey, message, signature }) => 
    verify(publicKey, message, signature)
  );
}

// ============================================
// Usage Examples
// ============================================

async function examples() {
  // 1. Generate new account
  const alice = FalconAccount.generate();
  console.log('Address:', alice.address);
  console.log('Mnemonic:', alice.getMnemonic());

  // 2. Recover from mnemonic
  const mnemonic = alice.getMnemonic();
  const bob = FalconAccount.fromMnemonic(mnemonic);
  console.log('Recovered:', bob.address);
  console.log('Match:', alice.address === bob.address);

  // 3. Sign and verify
  const message = new TextEncoder().encode('Transfer 100 QTX');
  const signature = alice.sign(message);
  const isValid = verify(alice.publicKey, message, signature);
  console.log('Signature valid:', isValid);

  // 4. Send transaction (integrated with SDK)
  const { QylithSDK } = require('./sdk');
  const sdk = new QylithSDK({ provider: 'ws://localhost:9944' });
  
  await sdk.connect();
  
  const tx = await sdk.tx.balances.transfer({
    dest: 'Qtx5XYZ...123',
    value: 100_000_000_000n, // 100 QTX
  });
  
  // Sign with FALCON keypair
  const signedTx = await tx.signAsync(alice);
  const hash = await signedTx.submit();
  
  console.log('Transaction hash:', hash.toHex());
  console.log('Status: Pending...');
  
  await hash.waitForFinalized();
  console.log('Status: Finalized!');
  
  await sdk.disconnect();
}

examples().catch(console.error);
```

### 6.3 运行示例

```bash
# Rust example
cargo run --example falcon_full_example

# TypeScript example (requires WASM build)
cd packages/qylith-sdk
npm run build
npm run example falcon
```

---

## 📚 相关资源

- [FALCON 官方规范](https://falcon-signature.info/)
- [NIST PQC 标准化](https://csrc.nist.gov/projects/post-quantum-cryptography)
- [Qylith SDK](../sdk-guide.md)
- [API 参考](../api-reference.md)
