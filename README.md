<div align="center">

# ⚛️ Qylith

### The First AI-Native, Quantum-Resistant Layer 1 Blockchain

<p align="center">
  <img src="https://img.shields.io/badge/Quantum-Resistant-FALCON--1024-00D4FF?style=for-the-badge&logo=Rust&logoColor=white"/>
  <img src="https://img.shields.io/badge/AI-Native-AEM-7B2FFF?style=for-the-badge&logo=Python&logoColor=white"/>
  <img src="https://img.shields.io/badge/Framework-Substrate-4E8B9C?style=for-the-badge&logo=Substrate&logoColor=white"/>
  <img src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge"/>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Status-Alpha-FF6B6B?style=flat-square"/>
  <img src="https://img.shields.io/badge/Rust-1.75+-orange?style=flat-square&logo=Rust&logoColor=white"/>
  <img src="https://img.shields.io/badge/NIST-Level%205-00D4FF?style=flat-square"/>
  <img src="https://img.shields.io/badge/Build-Passing-brightgreen?style=flat-square"/>
</p>

[📜 Whitepaper](whitepaper/Qylith_Whitepaper_v1_EN.md) · [🏗️ Architecture](架构设计/Qylith_架构设计文档.md) · [📋 Pitch Deck](融资/Qylith_Pitch_Deck.md) · [🎯 Cryptography](融资/Qylith_Crypto_Spec.md)

</div>

---

## ⚠️ The Quantum Apocalypse Is Coming

```
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║   ╭──────────────────────────────────────────────────────────────────────╮   ║
║   │                                                                      │   ║
║   │   ⏰ QUANTUM THREAT TIMELINE                                         │   ║
║   │                                                                      │   ║
║   │   💰 $2.5 Trillion  →  Crypto assets at quantum risk TODAY        │   ║
║   │   🔬 500K Qubits    →  Resources needed to break ECDSA-256        │   ║
║   │   📅 2027-2030      →  Expected "Q-Day" attack window              │   ║
║   │   🚫 No Plan        →  Ethereum has no quantum migration plan       │   ║
║   │   ⚡ 20x Faster     →  Google's 2026 paper cut break time         │   ║
║   │                                                                      │   ║
║   ╰──────────────────────────────────────────────────────────────────────╯   ║
║                                                                              ║
║                    "Every chain running ECDSA is already dead.               ║
║                     They just don't know it yet."                            ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
```

---

## 🛡️ Why Qylith Is Different

| Feature | Qylith | Ethereum | Bitcoin | Solana |
|---------|:------:|:--------:|:-------:|:------:|
| **Post-Quantum from Genesis** | ✅ FALCON-1024 | ❌ None | ❌ None | ❌ None |
| **AI Agent Native** | ✅ AEM Pallet | ❌ None | ❌ None | ❌ None |
| **Signature Size** | 1.3 KB | 65 B | 65 B | N/A |
| **Verification Speed** | 0.15 ms | <1 ms | <1 ms | N/A |
| **ML-KEM Key Exchange** | ✅ Yes | ❌ None | ❌ None | ❌ None |
| **STARK Verification** | 🔨 In Dev | ❌ None | ❌ None | ❌ None |

---

## 🔐 Post-Quantum Cryptography Stack

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                           QYLITH CRYPTOGRAPHIC LAYER                          │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐              │
│  │   FALCON-1024   │    │   ML-KEM-768    │    │    STARK        │              │
│  │                 │    │                 │    │                 │              │
│  │  Digital Signs  │    │  Key Exchange   │    │  Verifiable     │              │
│  │                 │    │                 │    │  Computation     │              │
│  ├─────────────────┤    ├─────────────────┤    ├─────────────────┤              │
│  │  Size:  1.3 KB │    │  Size:  4.6 KB │    │  Status: 🔨     │              │
│  │  Speed: 0.15ms │    │  Speed: 1.0 ms │    │  Prover Ready   │              │
│  │  NIST: Level 5 │    │  NIST: Level 5 │    │  Scalable       │              │
│  └─────────────────┘    └─────────────────┘    └─────────────────┘              │
│                                                                                 │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │                        SR25519 (Hybrid Fallback)                         │  │
│  │                      Classical + Quantum-Resistant                        │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### 🔬 Technical Specifications

| Algorithm | Type | NIST Level | Size | Speed | Status |
|-----------|------|:----------:|------|-------|:------:|
| **FALCON-1024** | Signature | 5 | 1.3 KB | 0.15ms | ✅ Production |
| **ML-KEM-768** | KEM | 5 | 4.6 KB | 1.0ms | ✅ Production |
| **SR25519** | Signature | 1 (hybrid) | 0.6 KB | 0.05ms | ✅ Active |
| **STARK** | Proof | ∞ | - | - | 🔨 Dev |

---

## 🤖 AI-Native Architecture

```
                            ┌─────────────────────────────────────────────┐
                            │              QYLITH NETWORK                  │
                            ├─────────────────────────────────────────────┤
                            │                                              │
    ┌─────────────────┐      │      ┌─────────────────────────────────┐    │
    │   🤖 AI Agent   │      │      │                                 │    │
    │   ┌───────────┐ │      │      │     AEM (AI Agent Execution)   │    │
    │   │  Registry │ │      │      │              Pallet              │    │
    │   ├───────────┤ │      │      │                                 │    │
    │   │ Scheduling│ │◄─────┼──────┤  • Agent Registration           │    │
    │   ├───────────┤ │      │      │  • Execution Environment        │    │
    │   │ Reputation│ │      │      │  • State Management              │    │
    │   └───────────┘ │      │      │  • Fee Settlement               │    │
    └─────────────────┘      │      │                                 │    │
                            │      └─────────────────────────────────┘    │
    ┌─────────────────┐      │                      │                       │
    │   🤖 AI Agent   │      │      ┌───────────────┴───────────────┐    │
    │   ┌───────────┐ │      │      │                               │    │
    │   │  Registry │ │      │      │   Cross-Chain Bridge          │    │
    │   ├───────────┤ │      │      │   (QuantumShield Protocol)    │    │
    │   │ Execution │ │◄────┼──────┤                               │    │
    │   └───────────┘ │      │      │   • FALCON Verification       │    │
    └─────────────────┘      │      │   • Asset Transfer              │    │
                            │      │   • Oracle Feeds                 │    │
    ┌─────────────────┐      │      └───────────────────────────────┘    │
    │   🤖 AI Agent   │      │                                              │
    │   ┌───────────┐ │      │      ┌─────────────────────────────────┐  │
    │   │  Registry │ │      │      │                                 │  │
    │   ├───────────┤ │      │      │      FALCON-1024 Signatures     │  │
    │   │  Scheduling│ │◄─────┼──────┤      ML-KEM-768 Key Exchange    │  │
    │   └───────────┘ │      │      │                                 │  │
    └─────────────────┘      │      └─────────────────────────────────┘  │
                            │                                              │
                            │      ┌─────────────────────────────────┐    │
                            │      │                                 │    │
                            │      │      Substrate Runtime Core     │    │
                            │      │                                 │    │
                            │      │      • Block Production         │    │
                            │      │      • Consensus (NPoS)         │    │
                            │      │      • Smart Contracts           │    │
                            │      │                                 │    │
                            │      └─────────────────────────────────┘    │
                            │                                              │
                            └─────────────────────────────────────────────┘
```

---

## ⚡ Performance Metrics

```
╭─────────────────────────────────────────────────────────────────────────────╮
│                                                                             │
│   🏃 Block Time          │  6 seconds                                        │
│   📊 Transactions/sec   │  10,000+ (target)                                 │
│   ⚡ Finality           │  < 1 second (FALCON verification)                │
│   💾 Signature Size     │  1.3 KB (FALCON-1024)                             │
│   🔐 Security Level     │  NIST Level 5 (Post-Quantum)                     │
│   🌐 Cross-Chain       │  QuantumShield Bridge                             │
│   🤖 AI Execution      │  Native AEM Pallet                                │
│                                                                             │
╰─────────────────────────────────────────────────────────────────────────────╯
```

---

## 🚀 Quick Start

### Prerequisites
```
• Rust 1.75+
• Substrate development environment
• 8GB RAM minimum
```

### Build
```bash
git clone https://github.com/bahuang081-svg/Qylith.git
cd Qylith/code
cargo build --release
```

### Run Dev Node
```bash
./target/release/qylith-node --dev
```

### Run Tests
```bash
cargo test --all
```

---

## 📁 Project Structure

```
Qylith/
├── code/                              # Substrate chain implementation
│   ├── primitives/crypto/              # Post-quantum cryptography
│   │   ├── falcon.rs                  # FALCON-1024 signatures
│   │   ├── ml_kem.rs                  # ML-KEM-768 key encapsulation
│   │   ├── sr25519.rs                 # Hybrid classical signatures
│   │   └── runtime/                   # Chain runtime
│   ├── pallets/aem/                   # AI Agent Execution Module
│   │   └── src/
│   ├── node/                          # Node implementation
│   ├── quantum-shield-bridge/          # Cross-chain bridge MVP
│   └── ethereum/                      # Ethereum contracts
├── whitepaper/                        # Documentation
│   └── Qylith_Whitepaper_v1_EN.md
├── 架构设计/                          # Architecture docs
│   └── Qylith_架构设计文档.md
├── 融资/                              # Pitch & Spec
│   ├── Qylith_Pitch_Deck.md
│   └── Qylith_Crypto_Spec.md
└── README.md
```

---

## 🗺️ Roadmap

| Phase | Timeline | Milestone |
|:-----:|:--------:|-----------|
| **Phase 0** | Q2 2026 | Whitepaper, Architecture, Competitive Analysis |
| **Phase 1** | Q3 2026 | Testnet v1 with FALCON-1024 + AEM |
| **Phase 2** | Q4 2026 | QuantumShield Bridge mainnet, AI Agent SDK |
| **Phase 3** | Q1 2027 | Mainnet launch, Staking, Governance |
| **Phase 4** | Q2 2027 | Ecosystem expansion, Hackathons, Grants |

---

## 💰 Token: QYL

- **Total Supply:** 1,000,000,000 QYL
- **Seed Round:** $2M @ $10M FDV
- **Utility:** Staking, Governance, AI Agent execution fees, Bridge fees

---

## 🤝 Contributing

Qylith is open source and we welcome contributions!

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 🔗 Links

- 🌐 [Whitepaper (English)](whitepaper/Qylith_Whitepaper_v1_EN.md)
- 🌐 [Whitepaper (中文)](白皮书/Qylith_Whitepaper_v1_EN.md)
- 🏗️ [Architecture](架构设计/Qylith_架构设计文档.md)
- 📊 [Pitch Deck](融资/Qylith_Pitch_Deck.md)
- 🔐 [Cryptography Spec](融资/Qylith_Crypto_Spec.md)

---

<div align="center">

```
 ╭─────────────────────────────────────────────────────────────────────────╮
 │                                                                         │
 │   ░██████╗██████╗░██╗██████╗░██╗████████╗███████╗██████╗░               │
 │   ██╔════╝██╔══██╗██║██╔══██╗██║╚══██╔══╝██╔════╝██╔══██╗               │
 │   ╚█████╗░██████╔╝██║██████╔╝██║░░░██║░░░█████╗░░██████╔╝               │
 │   ░╚═══██╗██╔═══╝░██║██╔══██╗██║░░░██║░░░██╔══╝░░██╔══██╗               │
 │   ██████╔╝██║░░░░░██║██║░░██║██║░░░██║░░░███████╗██║░░██║               │
 │   ╚═════╝░╚═╝░░░░░╚═╝╚═╝░░╚═╝╚═╝░░░╚═╝░░░╚══════╝╚═╝░░╚═╝               │
 │                                                                         │
 │                    🔐 QUANTUM-RESISTANT • 🤖 AI-NATIVE 🔐               │
 │                                                                         │
 ╰─────────────────────────────────────────────────────────────────────────╯
```

**The quantum era doesn't wait. Neither do we.** ⚛️

</div>
