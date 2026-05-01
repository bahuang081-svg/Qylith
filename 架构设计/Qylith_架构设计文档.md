# Qylith Technical Architecture Document

## 技术架构设计文档 v1.1

**项目代号**: Qylith (Quantum + Cortex)
**文档类型**: Technical Architecture Specification
**版本**: 1.1（基于竞品分析更新）
**日期**: 2026年

---

## 目录

1. [架构设计原则](#1-架构设计原则)
2. [整体架构概览](#2-整体架构概览)
3. [密码学层详细设计](#3-密码学层详细设计)
4. [共识层详细设计](#4-共识层详细设计)
5. [执行层详细设计](#5-执行层详细设计)
6. [数据层详细设计](#6-数据层详细设计)
7. [网络层详细设计](#7-网络层详细设计)
8. [AI Agent执行模块(AEM)详细设计](#8-ai-agent执行模块aem详细设计)
9. [竞品对比与差异化分析](#9-竞品对比与差异化分析)
10. [安全模型分析](#10-安全模型分析)
11. [性能基准与目标](#11-性能基准与目标)

---

## 1. 架构设计原则

### 1.1 核心设计原则

Qylith架构遵循以下核心原则：

| 原则 | 描述 | 在Qylith中的体现 |
|------|------|-----------------|
| **Security First** | 安全性不可妥协 | 原生PQC、AI原生身份系统 |
| **Modularity** | 模块化设计 | 各层松耦合，可独立演进 |
| **Crypto-Agility** | 密码学可升级 | 算法抽象层，支持未来升级 |
| **Performance** | 高性能执行 | 并行执行引擎、Layer 2支持 |
| **True AI-Native** | AI第一公民 | 协议级AEM，非应用附加 |

### 1.2 设计决策的权衡

**性能 vs 安全性**：
- 选择**FALCON-1024**作为主签名：1.3KB签名大小，0.15ms验证延迟，性能卓越
- 选择ML-DSA-87作为混合签名备选：提供双保险

**去中心化 vs 效率**：
- NPoS支持大量验证人（1000+）
- 采用GRANDPA-like最终性 gadget

---

## 2. 整体架构概览

### 2.1 分层架构图

```
┌─────────────────────────────────────────────────────────────────┐
│                      APPLICATION LAYER                           │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐        │
│  │  DeFi    │  │  AI      │  │ 跨链桥   │  │  隐私    │        │
│  │Protocols │  │Marketplace│  │ Bridges │  │Protocols │        │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘        │
├─────────────────────────────────────────────────────────────────┤
│                       EXECUTION LAYER                            │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │              WASM Runtime (ink! + Solidity)              │   │
│  └──────────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │          AI Agent Execution Module (AEM)                  │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │   │
│  │  │Agent ID  │  │Inference │  │Autonomous│  │Multi-Agent│ │   │
│  │  │Registry  │  │Verifier  │  │Executor  │  │Orchestrator│ │   │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘  │   │
│  └──────────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │           Parallel Execution Engine (PEE)                  │   │
│  └──────────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│                       CONSENSUS LAYER                           │
│  ┌────────────┐  ┌────────────┐  ┌────────────────────────┐   │
│  │    NPoS    │  │  Block     │  │   Finality Gadget       │   │
│  │  Election  │  │ Production │  │   (GRANDPA-like)        │   │
│  └────────────┘  └────────────┘  └────────────────────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│                     CRYPTOGRAPHY LAYER                           │
│  ┌──────────────┐  ┌──────────────┐  ┌────────────────────┐     │
│  │ FALCON-1024  │  │   ML-KEM-768 │  │ Hybrid Signature   │     │
│  │   Signing    │  │   KEM        │  │    Manager         │     │
│  └──────────────┘  └──────────────┘  └────────────────────┘     │
├─────────────────────────────────────────────────────────────────┤
│                        NETWORK LAYER                             │
│  ┌──────────────┐  ┌──────────────┐  ┌────────────────────┐     │
│  │   libp2p     │  │   PQC-TLS    │  │  Cross-Chain      │     │
│  │   Base       │  │   Handshake  │  │  Message Relayer   │     │
│  └──────────────┘  └──────────────┘  └────────────────────┘     │
├─────────────────────────────────────────────────────────────────┤
│                        DATA LAYER                                │
│  ┌──────────────┐  ┌──────────────┐  ┌────────────────────┐     │
│  │  State MMR   │  │  State Expiry│  │  Transaction Log  │     │
│  │   Storage    │  │   Manager    │  │   (Events)        │     │
│  └──────────────┘  └──────────────┘  └────────────────────┘     │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 组件交互流程

```
用户交易生命周期：

1. Client签署交易（FALCON-1024签名，1.3KB）
       ↓
2. P2P网络传播（ML-KEM-768加密会话密钥）
       ↓
3. Validator验证签名（0.15ms延迟）
       ↓
4. Mempool按依赖关系排序
       ↓
5. Block Producer打包区块
       ↓
6. 验证人并行验证交易
       ↓
7. GRANDPA最终性确认
       ↓
8. 状态更新 → MMR状态根更新
```

---

## 3. 密码学层详细设计

### 3.1 后量子密码学标准选择依据

#### 3.1.1 签名算法对比分析

| 算法 | 签名大小 | 验证延迟 | NIST等级 | 量子安全 | Qylith应用 |
|------|---------|---------|---------|---------|-----------|
| **FALCON-1024** | **1,280 bytes** | **0.15ms** | **Level 5** | ✅ | **主签名** |
| ML-DSA-87 | 2,420 bytes | ~1ms | Level 5 | ✅ | 混合签名备选 |
| ECDSA secp256k1 | 64 bytes | ~0.1ms | - | ❌ | 过渡期兼容 |
| Dilithium-3 | 3,293 bytes | ~0.5ms | Level 3 | ✅ | 备选 |

**选择FALCON-1024的理由**：
- **最小签名**：1.3KB，比ML-DSA-87小46%，大幅降低存储与带宽成本
- **极速验证**：0.15ms，比ML-DSA-87快6-7倍，提升TPS
- **最高安全**：NIST Level 5，等效128-bit经典安全
- **实用成熟**：基于NTRU格，理论扎实，实现优化成熟

#### 3.1.2 NIST后量子密码学标准化进程

| 算法类型 | NIST标准 | Qylith采用 |
|---------|---------|-----------|
| 数字签名 | FALCON (FIPS 206草案) | **FALCON-1024** |
| 数字签名备选 | ML-DSA (FIPS 206) | ML-DSA-87 |
| 密钥封装 | ML-KEM (FIPS 203) | ML-KEM-768 |
| 哈希函数 | SHA-3, SHAKE | SHA-3/256, Poseidon |

### 3.2 FALCON-1024 实现规格

#### 3.2.1 参数定义

```rust
// FALCON-1024 参数结构
pub struct FALCON_1024_PARAMS {
    // 多项式阶数
    n: u32 = 1024,
    
    // 模数
    q: u32 = 12289,
    
    // 高斯噪声标准差
    sigma: f64 = 1.61684067,
    
    // 标准化衰落
    beta: f64 = 8.0 / 3.14159265,
    
    // 签名大小（bytes）
    signature_size: usize = 1280,
    
    // 公钥大小（bytes）
    public_key_size: usize = 1793,
    
    // 安全等级
    security_level: SecurityLevel = SecurityLevel::Level5,
}
```

#### 3.2.2 密钥生成

```rust
// FALCON基于NTRU格，密钥生成包含：
// 1. 生成私钥多项式 f, g, F, G
// 2. 计算公钥 h = (g + qG) / f mod q
// 核心困难问题：NTRU Shortest Vector Problem

pub struct FalconKeyPair {
    pub public_key: FalconPublicKey,   // h: 1793 bytes
    pub secret_key: FalconSecretKey,   // f, F: ~2.3KB
}

pub struct FalconSecretKey {
    f: Polynomial,  // 私钥小多项式
    F: Polynomial,   // 辅助小多项式
    g: Polynomial,   // 用于签名
    G: Polynomial,   // 用于签名
}

impl FalconKeyPair {
    pub fn keygen() -> Self {
        // 1. 采样私钥多项式 f, g (小范数)
        let f = sample_ternary_lattice(sigma);
        let g = sample_ternomial_lattice(sigma);
        
        // 2. 计算逆元 f^(-1) mod q
        let f_inv = f.inverse_mod(q).unwrap();
        
        // 3. 计算公钥 h = g * f^(-1) mod q
        let h = g.mul(&f_inv).mod(q);
        
        // 4. 生成G, F满足 fG - gF = q
        let (F, G) = compute_FG(&f, &g, q);
        
        FalconKeyPair {
            public_key: FalconPublicKey { h },
            secret_key: FalconSecretKey { f, F, g, G },
        }
    }
}
```

#### 3.2.3 签名生成（Fast Fourier Sampling）

```rust
impl FalconSigning {
    pub fn sign(message: &[u8], sk: &FalconSecretKey) -> FalconSignature {
        // 1. Hash-to-tree: 构建签名树
        let tree = FalconTree::build(message);
        
        // 2. 迭代采样
        let (s1, s2) = self.fast_fourier_sampling(&tree, &sk);
        
        // 3. 验证签名范数
        let norm = s1.norm() + s2.norm();
        if norm > THRESHOLD {
            return self.sign(message, sk); // 重新采样
        }
        
        // 4. 计算最终签名
        let signature = s1 + s2 * sk.h; // mod q
        
        FalconSignature { v: signature }
    }
}
```

### 3.3 ML-DSA-87 作为混合签名备选

```rust
// 混合签名结构（作为过渡期方案）
pub struct HybridSignature {
    // 主签名：FALCON-1024
    falcon_sig: FalconSignature,
    falcon_pubkey: FalconPublicKey,
    
    // 备选签名：ML-DSA-87
    mldsa_sig: Option<MLDSASignature>,
    mldsa_pubkey: Option<MLDSAPublicKey>,
    
    // 签名时间戳
    timestamp: u64,
}

impl HybridSignature {
    pub fn verify(&self, message: &[u8]) -> bool {
        // FALCON主验证
        let falcon_valid = self.falcon_sig.verify(message, &self.falcon_pubkey);
        
        // 如果提供ML-DSA签名，也验证
        let mldsa_valid = match (&self.mldsa_sig, &self.mldsa_pubkey) {
            (Some(sig), Some(pk)) => sig.verify(message, pk),
            _ => true, // 非必需
        };
        
        // 时间戳检查
        let timestamp_valid = self.check_timestamp();
        
        // 所有检查通过
        falcon_valid && mldsa_valid && timestamp_valid
    }
}
```

### 3.4 ML-KEM-768 实现规格

```rust
pub struct ML_KEM_768_PARAMS {
    k: u32 = 3,                  // Module rank
    n: u32 = 256,                // Polynomial degree
    q: u32 = 3329,               // 模数
    public_key_size: usize = 1184, // 公钥大小
    cipher_text_size: usize = 1088, // 密文大小
    shared_secret_size: usize = 32,
}
```

**应用场景**：
- 节点间TLS-like加密通道（替代TLS 1.3）
- 跨链桥的密钥协商
- 秘密共享与阈值签名

### 3.5 Crypto-Agility 实现

```rust
pub struct CryptoRegistry {
    // 已注册的签名算法
    signing_algorithms: HashMap<AlgorithmId, Box<dyn SignatureScheme>>,
    
    // 已注册的KEM算法
    kem_algorithms: HashMap<AlgorithmId, Box<dyn KEMScheme>>,
    
    // 默认算法
    default_signing: AlgorithmId = AlgorithmId::FALCON_1024,
    default_kem: AlgorithmId = AlgorithmId::ML_KEM_768,
}

pub enum AlgorithmId {
    // 主签名
    FALCON_1024,
    FALCON_512,
    
    // 备选签名
    ML_DSA_87,
    ML_DSA_65,
    
    // 研究方向（未来集成）
    ORBIT_SIGNATURE,  // CEH-Orbit: 567字节签名
    DILITHIUM_5,
    
    // KEM
    ML_KEM_768,
    ML_KEM_1024,
}
```

### 3.6 未来研究方向：CEH-Orbit 轨道密码学

**轨道密码学（Orbit Cryptography）**是值得关注的新兴PQC方向：

| 特性 | 传统格密码 | Orbit签名 |
|------|----------|----------|
| 签名大小 | 1-3KB | **567 bytes** |
| 安全基础 | Module-Lattice | 编码理论+格 |
| 审计状态 | 成熟(NIST标准化) | 早期(论文阶段) |
| 风险 | 低 | 中高(未广泛审计) |

**Qylith立场**：
- **当前**：不采用Orbit签名，安全性优先
- **跟踪**：密切跟进Orbit密码学的学术进展与审计结果
- **储备**：Crypto-Agility架构支持未来无缝集成
- **目标**：一旦Orbit通过充分审计且安全等级达标，可作为FALCON的替代升级

---

## 4. 共识层详细设计

### 4.1 Substrate深度定制架构

**为什么选择Substrate**：
- **Quantus Network验证**：Balaji Srinivasan投资，2026 Q1已主网上线，证明Substrate抗量子链可行
- **Polkadot生态互操作**：XCMP消息传递，共享安全性
- **Rust生态优势**：密码学库丰富，内存安全
- **快速出活**：成熟框架，开发效率高

```rust
// Qylith Runtime定制结构
pub mod qylith_runtime {
    use frame_system::Config as SysConfig;
    use pallet_balances::Config as BalancesConfig;
    use pallet_staking::Config as StakingConfig;
    
    // Qylith核心模块
    mod pallet_pqc_crypto {
        // 原生FALCON-1024签名验证
        // ML-KEM-768密钥封装
        // 混合签名支持
    }
    
    mod pallet_aem {
        // AI Agent执行模块
        // Agent注册与身份
        // 声誉系统
    }
    
    mod pallet_parallel_exec {
        // 并行执行引擎
        // 依赖图分析
        // 乐观并发控制
    }
    
    mod pallet_state_mmr {
        // Merkle Mountain Range
        // 状态过期机制
        // 量子安全承诺
    }
}
```

### 4.2 Nominated Proof of Stake (NPoS)

**设计目标**：

| 目标 | 实现方式 |
|------|---------|
| 高安全性 | 2/3+诚实验证人假设 |
| 去中心化 | 大量验证人(1000+)，提名人参与 |
| 经济激励 | 合理的质押收益与惩罚机制 |
| 抗审查 | 无许可的验证人加入 |

**参考Quantus Network的NPoS实现**：
```rust
// 基于Substrate Staking Pallet定制
pub struct NPoSConfig {
    // 验证人参数
    pub min_validator_bond: Balance = 100_000 * QYL,
    pub max_validators: u32 = 1000,
    pub target_validators: u32 = 500,
    
    // 提名人参数
    pub min_nominator_bond: Balance = 1_000 * QYL,
    pub max_nominations: u32 = 16,
    
    // 收益参数
    pub validator_commission: Perbill = Perbill::from_percent(20),
    pub reward_payout_frequency: BlockNumber = 600, // 1小时
}
```

### 4.3 出块与最终性

**时间参数**：

```rust
pub struct ChainParameters {
    // Slot时长
    pub slot_duration: Duration = Duration::from_secs(6),
    
    // 每个Epoch的Slot数
    pub slots_per_epoch: u64 = 300,
    
    // Epoch时长
    pub epoch_duration: Duration = Duration::from_secs(1800), // 30分钟
    
    // 每个Era的Epoch数
    pub epochs_per_era: u64 = 12,
    
    // Era时长
    pub era_duration: Duration = Duration::from_secs(21600), // 6小时
}
```

**区块结构**：

```rust
pub struct Block {
    header: QylithBlockHeader,
    body: QylithBlockBody,
}

pub struct QylithBlockHeader {
    parent_hash: Hash,
    number: BlockNumber,
    state_root: MMRRoot,           // MMR状态根
    transactions_root: Hash,
    receipts_root: Hash,
    logs: Vec<Log>,
    validator_set: ValidatorSetId,
    signature: VersionedSignature, // FALCON-1024签名
}

pub struct QylithBlockBody {
    transactions: Vec<SignedTransaction>,
    epoch_transition: Option<EpochTransition>,
}
```

---

## 5. 执行层详细设计

### 5.1 WASM运行时架构

```
┌─────────────────────────────────────────────────────┐
│                 Qylith Runtime                       │
├─────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────┐   │
│  │            VM Executor (WASM)               │   │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐      │   │
│  │  │  ink!   │  │Solidity │  │  Move   │      │   │
│  │  │Runtime  │  │ Adapter │  │(Future) │      │   │
│  │  └─────────┘  └─────────┘  └─────────┘      │   │
│  └─────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────┐   │
│  │            Host Functions                    │   │
│  │  - Balance Read/Write                       │   │
│  │  - Storage Read/Write                       │   │
│  │  - Call other contracts                     │   │
│  │  - Call AEM primitives                      │   │
│  └─────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────┐   │
│  │            Gas Metering                      │   │
│  │  - Operation costing                        │   │
│  │  - AI execution gas                         │   │
│  └─────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────┘
```

### 5.2 并行执行引擎 (PEE)

**灵感来源**：MegaETH的并行EVM思路

```rust
pub struct ParallelExecutor {
    workers: WorkerPool,
    cache: TransactionCache,
}

impl ParallelExecutor {
    pub async fn execute_block(&self, block: Block) -> Result<BlockResult> {
        // 1. 分析依赖图
        let deps = DependencyGraph::analyze(&block.transactions);
        
        // 2. 获取第一层可并行交易
        let mut ready = deps.parallelizable_set();
        let mut results = Vec::new();
        
        while !ready.is_empty() {
            // 3. 并行执行所有就绪交易
            let batch_results = self.workers.execute_batch(ready.clone()).await;
            
            // 4. 收集结果并检查冲突
            for (tx_id, result) in batch_results.into_iter() {
                match result {
                    Ok(receipt) => {
                        results.push((tx_id, receipt));
                        self.cache.confirm_writes(tx_id);
                    }
                    Err(Conflict) => {
                        // 回滚并标记为串行重试
                        self.retry_serial(tx_id).await;
                    }
                }
            }
            
            // 5. 更新依赖图，获取下一层
            ready = deps.next_layer();
        }
        
        Ok(BlockResult { results })
    }
}
```

---

## 6. 数据层详细设计

### 6.1 Merkle Mountain Range (MMR)

```rust
pub struct MMR {
    peaks: Vec<MMRPeak>,
    nodes: HashMap<Hash, MMRNode>,
    size: u64,
}

impl MMR {
    pub fn push(&mut self, leaf_hash: Hash) {
        let pos = self.size;
        self.size += 1;
        
        let leaf = MMRNode::Leaf { position: pos, hash: leaf_hash };
        self.nodes.insert(hash_of(&leaf), leaf);
        
        // 向上合并
        let mut current = hash_of(&leaf);
        let mut height = 0;
        
        while pos % 2 == 1 && height < self.peaks.len() as u32 {
            let left = self.peaks[height as usize].hash;
            current = hash_nodes(left, current, height);
            self.peaks[height as usize] = MMRPeak { height, hash: current };
            height += 1;
        }
        
        if height >= self.peaks.len() as u32 {
            self.peaks.push(MMRPeak { height, hash: current });
        }
    }
    
    pub fn root(&self) -> MMRRoot {
        hash_peaks(&self.peaks)
    }
}
```

### 6.2 状态过期机制

```rust
pub struct StateExpiryManager {
    // 活跃状态（最近2个epoch）
    active_state: Arc<RwLock<HashMap<StorageKey, StorageValue>>>,
    
    // 历史状态归档
    historical_archive: Arc<RwLock<ArchiveStore>>,
    
    current_epoch: EpochIndex,
}

impl StateExpiryManager {
    pub fn on_epoch_change(&mut self, new_epoch: EpochIndex) {
        let expiry_epoch = if new_epoch > 2 { new_epoch - 2 } else { 0 };
        
        // 移动过期状态到归档
        let to_archive: Vec<_> = self.key_epochs.iter()
            .filter(|(_, meta)| meta.epoch < expiry_epoch)
            .map(|(k, _)| k.clone())
            .collect();
        
        for key in to_archive {
            if let Some(value) = self.active_state.write().unwrap().remove(&key) {
                self.historical_archive.write().unwrap()
                    .archive(key, value, expiry_epoch);
            }
        }
        
        self.current_epoch = new_epoch;
    }
}
```

### 6.3 Quantum-Secure State Commitment

**使用Poseidon哈希**（zkSNARK友好，量子抵抗）：

```rust
pub struct PoseidonHasher {
    full_rounds: usize = 8,
    partial_rounds: usize = 22,
}

impl PoseidonHasher {
    pub fn hash(&self, inputs: &[FieldElement]) -> FieldElement {
        let mut state = inputs.to_vec();
        state.resize(3, FieldElement::zero());
        
        // 完整轮次 + 部分轮次 + 最终轮次
        // ... (详见FRI/Rescue哈希设计)
        
        state[0]
    }
}
```

---

## 7. 网络层详细设计

### 7.1 libp2p + Substrate网络栈

**参考Quantus Network的实现**：

```rust
pub struct QylithNetworkConfig {
    listen_addresses: Vec<Multiaddr>,
    max_connections: u32 = 5000,
    
    // PQC加密握手
    handshake_protocol: HandshakeProtocol = HandshakeProtocol::PQNoise,
    
    // gossipsub配置
    gossip_config: GossipParams = GossipParams::default(),
}

pub fn build_transport(config: &QylithNetworkConfig) -> Transport {
    Transport::new(
        tcp::TcpConfig::new()
            .nodelay(true)
    )
    .upgrade(Version::V1)
    .authenticate(PQNoiseAuthenticator::new(
        MLKEM768::new(),
        FALCON_SIGNER::new(),
    ))
    .multiplex(yamux::YamuxConfig::default())
    .boxed()
}
```

### 7.2 PQC加密的握手流程

```rust
pub struct PQCHandshake {
    pub async fn client_handshake(
        stream: NoiseStream,
        client_static_key: FALCONPublicKey,
    ) -> Result<(NoiseStream, SharedSecret)> {
        // 1. 发送FALCON公钥
        stream.write_all(&client_static_key.serialize()).await?;
        
        // 2. 接收服务器FALCON公钥
        let server_key_bytes = stream.read_fixed(1793).await?;
        let server_falcon_key = FALCONPublicKey::deserialize(server_key_bytes)?;
        
        // 3. ML-KEM密钥封装
        let (ciphertext, client_shared) = MLKEM768::encapsulate(&server_falcon_key)?;
        stream.write_all(&ciphertext).await?;
        
        // 4. 接收服务器KEM响应
        let server_ct_bytes = stream.read_fixed(1088).await?;
        let server_shared = MLKEM768::decapsulate(&client_static_key, &server_ct_bytes)?;
        
        // 5. 导出会话密钥
        let session_key = kdf(b"qylith-pqc" || client_shared || server_shared);
        
        Ok((stream, session_key))
    }
}
```

---

## 8. AI Agent执行模块(AEM)详细设计

### 8.1 AEM是Qylith的核心创新

**核心理念**：AEM不是"支持AI"，而是"为AI Agent而生"。

**与其它链的"AI支持"对比**：

| 特性 | 其它链 | Qylith |
|------|--------|--------|
| AI推理 | 链下执行，无法验证 | **链上STARK验证** |
| Agent身份 | 借用钱包，事后授权 | **原生身份注册** |
| 执行权限 | 人类签名授权 | **协议级自主执行** |
| 声誉系统 | 无 | **链上声誉+Slash惩罚** |
| 多Agent协调 | 无 | **原生协调协议** |

### 8.2 AEM架构

```
┌─────────────────────────────────────────────────────────────────┐
│                 AI Agent Execution Module (AEM)                 │
├─────────────────────────────────────────────────────────────────┤
│  ┌───────────────────────────────────────────────────────────┐   │
│  │                    Agent Registry                        │   │
│  │  - Agent身份注册（FALCON公钥绑定）                       │   │
│  │  - 权限级别管理                                           │   │
│  │  - 质押与惩罚机制                                         │   │
│  └───────────────────────────────────────────────────────────┘   │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │                 Inference Verifier                       │   │
│  │  - STARK proof verification (<100ms)                    │   │
│  │  - ZK-SNARK proof verification                           │   │
│  │  - 模型哈希验证                                           │   │
│  └───────────────────────────────────────────────────────────┘   │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │                Autonomous Executor                        │   │
│  │  - 协议级权限管理                                         │   │
│  │  - 自主交易签名                                           │   │
│  │  - 预算控制                                               │   │
│  └───────────────────────────────────────────────────────────┘   │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │              Multi-Agent Orchestrator                     │   │
│  │  - Agent-to-Agent消息                                     │   │
│  │  - 原子性多跳交易                                         │   │
│  │  - 争议解决                                               │   │
│  └───────────────────────────────────────────────────────────┘   │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │                  Reputation System                        │   │
│  │  - 链上声誉分数                                           │   │
│  │  - 信任图谱                                               │   │
│  │  - Slash条件                                              │   │
│  └───────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

### 8.3 Agent Registry

```rust
pub struct AgentRegistration {
    pub metadata_uri: CID,
    pub owner: AccountId,
    
    // FALCON-1024签名密钥（Agent身份绑定）
    pub signing_key: FALCONPublicKey,
    
    // ML-KEM加密密钥（通信加密）
    pub encryption_key: MLKEMPublicKey,
    
    pub permission_level: PermissionLevel,
    pub stake: Balance,
}

pub enum PermissionLevel {
    ReadOnly,           // 仅读取链上数据
    TradeOnly(TokenId), // 仅与特定代币交互
    Unilateral,         // 自主执行任何交易
    Managed,            // 需要所有者确认
}

impl AgentRegistry {
    pub fn register(&self, registration: AgentRegistration) -> Result<AgentId> {
        assert!(registration.stake >= MIN_AGENT_STAKE);
        
        // 验证FALCON签名公钥
        assert!(registration.signing_key.is_valid());
        
        // 生成Agent ID（绑定FALCON公钥）
        let agent_id = AgentId::from(sha3_256(&registration.signing_key));
        
        self.agents.insert(agent_id, registration);
        self.emit_event(Event::AgentRegistered { agent_id });
        
        Ok(agent_id)
    }
}
```

### 8.4 Inference Verifier（链上推理验证）

**核心创新**：STARK证明验证，让不确定的AI推理在确定性链上可验证。

```rust
pub struct InferenceVerifier {
    stark_verifier: StarkVerifier,
    allowed_models: HashMap<ModelId, ModelSpec>,
}

pub struct ModelSpec {
    pub model_hash: Hash,           // 权重哈希
    pub input_schema: Schema,
    pub output_schema: Schema,
    pub execution_limits: Limits,
}

impl InferenceVerifier {
    pub fn verify(&self, proof: &InferenceProof) -> Result<InferenceOutput> {
        // 1. 验证STARK证明
        assert!(self.stark_verifier.verify(&proof.stark_proof)?);
        
        // 2. 验证模型哈希
        let spec = self.allowed_models.get(&proof.model_id)?;
        assert_eq!(spec.model_hash, proof.model_hash);
        
        // 3. 验证输入/输出
        assert!(spec.input_schema.validate(&proof.input)?);
        assert!(spec.output_schema.validate(&proof.output)?);
        
        // 4. 验证时间戳（防止重放）
        assert!(proof.timestamp > self.last_verified_timestamp);
        
        Ok(InferenceOutput {
            verified: true,
            output: proof.output.clone(),
            confidence: proof.stark_proof.proof_quality(),
        })
    }
}
```

### 8.5 Autonomous Executor（自主执行器）

```rust
pub struct AutonomousExecutor {
    registry: AgentRegistry,
    agent_state: AgentStateStore,
}

impl AutonomousExecutor {
    pub async fn execute_transaction(
        &self,
        agent_id: AgentId,
        tx: UnsignedTransaction,
    ) -> Result<SignedTransaction> {
        let agent = self.registry.get_agent(agent_id)?;
        let auth = self.agent_state.get_auth(agent_id)?;
        
        // 验证权限级别
        match (agent.permission_level, &tx) {
            (PermissionLevel::ReadOnly, _) => {
                return Err(Unauthorized("Read-only agent".into()));
            }
            (PermissionLevel::TradeOnly(token_id), tx) => {
                assert_eq!(tx.token_id, token_id);
            }
            (PermissionLevel::Unilateral, _) => {}
            (PermissionLevel::Managed, _) => {
                return Err(NeedsConfirmation("Owner approval required".into()));
            }
        }
        
        // 验证预算
        assert!(tx.value + tx.fee <= auth.budget);
        
        // 验证时间窗口
        assert!(self.current_slot < auth.valid_until);
        
        // Agent签名交易
        let signed = self.sign_as_agent(agent_id, &tx)?;
        
        self.agent_state.deduct_budget(agent_id, tx.value + tx.fee);
        
        Ok(signed)
    }
}
```

### 8.6 Multi-Agent Orchestrator

```rust
pub struct MultiAgentOrchestrator {
    message_store: AgentMessageStore,
    registry: AgentRegistry,
}

pub struct AgentMessage {
    pub from: AgentId,
    pub to: AgentId,
    pub message_type: MessageType,
    pub payload: Vec<u8>,
    pub nonce: u64,
    pub signature: FALCONSignature,  // Agent用自己的FALCON密钥签名
}

impl MultiAgentOrchestrator {
    // 原子性多跳交易
    pub async fn atomic_multihop(
        &self,
        steps: Vec<ExecutionStep>,
    ) -> Result<AtomicResult> {
        let mut receipts = Vec::new();
        
        for step in steps {
            let receipt = self.execute_step(step).await?;
            
            if !receipt.success {
                // 回滚所有之前的步骤
                for prev_receipt in receipts.iter().rev() {
                    self.rollback(prev_receipt).await?;
                }
                return Ok(AtomicResult { success: false });
            }
            
            receipts.push(receipt);
        }
        
        Ok(AtomicResult { success: true, receipts })
    }
}
```

---

## 9. 竞品对比与差异化分析

### 9.1 竞品全景

| 项目 | 抗量子 | AI支持 | 主网状态 | 技术栈 | 融资 |
|------|--------|--------|---------|--------|------|
| **Qylith** | ✅ 原生 | ✅ 原生 | 规划中 | Substrate | 待定 |
| QoreChain | ✅ | ✅ | ❌ 8年未上线 | 三虚拟机 | 无 |
| Quantus Network | ✅ | ❌ | ✅ 2026 Q1上线 | Substrate | Balaji投资 |
| Ethereum | ❌ | ❌ | ✅ | 自研 | - |
| Solana | ❌ | ❌ | ✅ | 自研 | - |
| Algorand | ⚠️ 规划中 | ❌ | ✅ | 自研 | - |
| QANplatform | ⚠️ | ❌ | ✅ | 自研 | 有 |

### 9.2 直接竞品：QoreChain分析

**声称特性**：
- AI + 抗量子 + 三虚拟机（EVM + WASM + Move）
- "下一代Layer1"

**现实**：
- **8年无融资**：没有外部资本认可
- **主网未上线**：多年开发无产出
- **技术存疑**：声称三虚拟机，但无代码验证

**Qylith vs QoreChain**：
```
QoreChain: 声称三虚拟机 → 8年无果
Qylith:   Substrate单框架 → 快速出活 + 验证方案

QoreChain: 无融资 → 无法持续开发
Qylith:   目标融资 → 可持续开发

QoreChain: 主网未上 → 空谈
Qylith:   路线图清晰 → 可执行
```

### 9.3 参考案例：Quantus Network

**为何参考Quantus**：
- 2026 Q1已主网上线（验证可行性）
- Balaji Srinivasan投资（顶级投资人背书）
- Substrate架构（技术路线可参考）
- 纯抗量子（无AI）——留下差异化空间

**Qylith的差异化**：
```
Quantus:  抗量子 ✅  AI原生 ❌
Qylith:  抗量子 ✅  AI原生 ✅ ← 真正差异化

Quantus:  借鉴其Substrate定制方式
Qylith:   在其基础上增加AI原生层
```

### 9.4 真正的蓝海

**市场空白**：没有任何一条链同时做到：
1. ✅ 原生抗量子（从创世区块）
2. ✅ AI原生执行层（协议级，非应用附加）
3. ✅ 可验证的链上AI推理

**Qylith是第一个**：第一且唯一"AI原生 + 原生抗量子"的Layer 1。

### 9.5 技术选型确认：Substrate深度定制

**理由**：

| 考量 | Substrate | 自研 |
|------|-----------|------|
| **可行性** | ✅ Quantus已验证 | ❌ 无先例 |
| **开发速度** | ✅ 成熟框架 | ❌ 需从零构建 |
| **互操作性** | ✅ Polkadot生态 | ❌ 需自建跨链 |
| **密码学开发** | ✅ Rust生态丰富 | ✅ 可控但耗时 |
| **社区支持** | ✅ Parity + 生态 | ❌ 需自建 |

**结论**：Substrate深度定制是最佳选择——站在巨人肩膀上，专注差异化创新。

---

## 10. 安全模型分析

### 10.1 威胁模型

| 威胁类型 | 描述 | Qylith防御 |
|---------|------|-----------|
| 量子攻击 | Shor算法攻破ECDSA | **FALCON-1024原生** |
| 51%攻击 | 恶意验证人控制 | NPoS经济激励 |
| 重放攻击 | 消息被重复使用 | Nonce + 时间戳 |
| Sybil攻击 | 女巫攻击伪造身份 | 质押门槛 |
| AI恶意行为 | Agent执行恶意操作 | **声誉Slash + 质押惩罚** |
| 签名伪造 | 伪造Agent签名 | **FALCON密钥绑定** |

### 10.2 密码学安全假设

```
FALCON-1024 安全分析：
- 基础困难问题：NTRU Shortest Vector Problem
- NIST安全等级：Level 5（2^128经典安全）
- 量子安全：Grover加速后约2^85安全性
- 签名大小：1,280字节（极紧凑）
- 验证速度：0.15ms（极快）

ML-KEM-768 安全分析：
- 基础困难问题：Module-LWE
- NIST安全等级：Level 3（2^192经典安全）
- 用于密钥封装（不直接暴露于签名攻击）
```

---

## 11. 性能基准与目标

### 11.1 性能指标

| 指标 | 目标值 | 说明 |
|------|-------|------|
| TPS | 10,000+ | 并行执行优化 |
| 出块时间 | 6秒 | 与Polkadot相当 |
| 最终确认 | 12-18秒 | GRANDPA最终性 |
| 最大验证人 | 1,000+ | NPoS支持 |
| 签名验证 | 0.15ms | FALCON优化 |
| STARK验证 | <100ms | 链上验证 |

### 11.2 Gas成本

| 操作类型 | Gas | QYL成本估算 |
|---------|-----|-------------|
| 转账 | 21,000 | $0.002 |
| 简单合约调用 | 50,000 | $0.005 |
| 复杂合约调用 | 200,000 | $0.02 |
| AI Agent注册 | 1,000,000 | $0.10 |
| STARK验证 | 500,000 | $0.05 |

---

## 附录

### A. 参考项目

1. **Quantus Network** - Substrate抗量子链参考
2. **Polkadot** - NPoS共识参考
3. **MegaETH** - 并行执行引擎参考
4. **FALCON** - 签名算法参考
5. **liboqs** - PQC实现库

### B. 术语表

| 术语 | 定义 |
|------|------|
| PQC | Post-Quantum Cryptography，后量子密码学 |
| FALCON | Fast Fourier Lattice-based Compact Signatures over NTRU |
| AEM | AI Agent Execution Module，AI代理执行模块 |
| STARK | Scalable Transparent Arguments of Knowledge |
| MMR | Merkle Mountain Range，默克尔山岭 |
| Crypto-Agility | 密码学敏捷性 |

---

*文档版本: v1.1*
*最后更新: 2026年*
*维护者: Qylith Technical Team*
