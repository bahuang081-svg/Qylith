<div align="center">

# ⚛️ Qylith

### The First AI-Native, Quantum-Resistant Layer 1

**Quantum-proof from genesis. AI-native by design.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Substrate](https://img.shields.io/badge/Substrate-Frame-green.svg)](https://substrate.io/)

[Whitepaper](./白皮书/Qylith_Whitepaper_v1_EN.md) · [Architecture](./架构设计/Qylith_架构设计文档.md) · [Pitch Deck](./融资/Qylith_Pitch_Deck.md) · [Discord](https://discord.gg/qylith)

</div>

---

## ⚠️ The Quantum Clock Is Ticking

Google's March 2026 paper cut the estimated resources to break ECDSA by **20x**. 

| Threat | Timeline |
|--------|----------|
| 500K physical qubits can break ECDSA | 2027-2030 |
| Bitcoin BIP-360 still in discussion | 2026 |
| Ethereum has no quantum migration plan | 2026 |
| **$2.5 trillion in crypto at risk** | **Now** |

**Every major blockchain — Bitcoin, Ethereum, Solana — runs on cryptography that quantum computers will break.**

Qylith is built differently. From the very first block.

---

## 🔬 Why Qylith?

| Feature | Qylith | Other "Quantum-Resistant" Chains |
|---------|--------|----------------------------------|
| Quantum-resistant from genesis | ✅ FALCON-1024 (NIST Level 5) | ❌ Retrofitting Dilithium on top |
| AI Agent execution environment | ✅ Native AEM Pallet | ❌ None |
| Signature size | 1.3 KB (FALCON-1024) | 4.6 KB (ML-DSA-87) |
| Verification speed | 0.15 ms | 1 ms |
| Post-quantum KEM | ✅ ML-KEM-768 | ❌ Classical key exchange |
| Framework | Substrate (Polkadot ecosystem) | Custom / Cosmos SDK |

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────┐
│              Qylith Network                  │
├─────────────┬──────────────┬────────────────┤
│  AI Agent   │  Consensus   │  Cross-Chain   │
│  Execution  │  Layer       │  Bridge        │
│  Module     │  (NPoS)      │  (QuantumShield)│
├─────────────┼──────────────┼────────────────┤
│  FALCON-1024 Signatures  │  ML-KEM-768 KEM │
├───────────────────────────┴─────────────────┤
│           Substrate Runtime Core             │
└─────────────────────────────────────────────┘
```

### Core Components

- **FALCON-1024 Signatures** — NIST Level 5 post-quantum signatures, 1.3KB per sig, 0.15ms verification
- **ML-KEM-768 Key Encapsulation** — Post-quantum encrypted P2P communication
- **AEM (AI Agent Execution Module)** — Native pallet for AI agent registration, scheduling, and reputation
- **QuantumShield Bridge** — Quantum-resistant cross-chain bridge to Ethereum
- **SR25519 Hybrid Signatures** — Backward-compatible classical signatures for transition period

---

## 🚀 Quick Start

### Prerequisites
- Rust 1.75+
- Substrate development environment

### Build
```bash
git clone https://github.com/qylith-network/qylith.git
cd qylith/code
cargo build --release
```

### Run Dev Node
```bash
./target/release/qylith-node --dev
```

---

## 📊 Project Structure

```
Qylith/
├── code/                          # Substrate chain implementation
│   ├── primitives/crypto/         # Post-quantum cryptography
│   │   ├── falcon.rs              # FALCON-1024 signatures
│   │   ├── ml_kem.rs              # ML-KEM-768 key encapsulation
│   │   └── sr25519.rs             # Hybrid classical signatures
│   ├── runtime/                   # Chain runtime
│   │   └── pallets/aem/          # AI Agent Execution Module
│   └── node/                      # Node implementation
├── quantum-shield-bridge/         # Cross-chain bridge MVP
│   ├── ethereum/                  # Ethereum contracts
│   ├── qylith/                    # Qylith-side minting
│   ├── relayer/                   # Bridge relayer
│   └── frontend/                  # Bridge UI (demo.html)
├── 白皮书/                        # Whitepapers (CN + EN)
├── 竞品分析/                      # Competitive analysis
├── 代币经济/                      # Tokenomics
├── 应用生态/                      # Application ecosystem
├── 融资/                          # Pitch deck
└── 品牌/                          # Brand assets
```

---

## 🛣️ Roadmap

| Phase | Timeline | Milestone |
|-------|----------|-----------|
| **Phase 0** | Q2 2026 | Whitepaper, Architecture, Competitive Analysis |
| **Phase 1** | Q3 2026 | Testnet v1 with FALCON-1024 + AEM |
| **Phase 2** | Q4 2026 | QuantumShield Bridge mainnet, AI Agent SDK |
| **Phase 3** | Q1 2027 | Mainnet launch, Staking, Governance |
| **Phase 4** | Q2 2027 | Ecosystem expansion, Hackathons, Grants |

---

## 💰 Token: QYL

- **Total Supply**: 1,000,000,000 QYL
- **Seed Round**: $2M @ $10M FDV
- **Utility**: Staking, Governance, AI Agent execution fees, Bridge fees

See [Tokenomics](./代币经济/Qylith_代币经济学.md) for full details.

---

## 🤝 Contributing

Qylith is open source and we welcome contributions!

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🔗 Links

- 📄 [Whitepaper (English)](./白皮书/Qylith_Whitepaper_v1_EN.md)
- 📄 [Whitepaper (Chinese)](./白皮书/Qylith_Whitepaper_v1.md)
- 📊 [Competitive Analysis](./竞品分析/抗量子公链竞品深度分析.md)
- 💰 [Pitch Deck](./融资/Qylith_Pitch_Deck.md)
- 🏗️ [Architecture](./架构设计/Qylith_架构设计文档.md)

---

<div align="center">

**The quantum era doesn't wait. Neither do we.** ⚛️

</div>
