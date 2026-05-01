# Qylith - AI-Native Post-Quantum L1 Blockchain

<p align="center">
  <img src="https://img.shields.io/badge/Quantum-Resistant-FALCON--1024-blue" alt="Post-Quantum">
  <img src="https://img.shields.io/badge/AI-Native-AEM-green" alt="AI Native">
  <img src="https://img.shields.io/badge/Consensus-NPoS-orange" alt="NPoS">
  <img src="https://img.shields.io/badge/Language-Rust-orange" alt="Rust">
</p>

## Overview

**Qylith** is the world's first AI-native post-quantum Layer 1 blockchain, built on Substrate. It combines:

- **FALCON-1024 Signatures**: NIST Level 5 post-quantum digital signatures
- **ML-KEM-768 Encryption**: Post-quantum key encapsulation for P2P communication
- **AI Agent Execution Module (AEM)**: Native support for AI agents on-chain
- **NPoS Consensus**: Secure and decentralized proof-of-stake

## Key Features

### 🔐 Post-Quantum Cryptography
- **FALCON-1024**: 1,280 byte signatures, 0.15ms verification, NIST Level 5
- **ML-KEM-768**: Secure key encapsulation for network encryption
- **Crypto-Agility**: Architecture supports future algorithm upgrades

### 🤖 AI-Native Execution
- **Agent Registry**: On-chain identity for AI agents with FALCON key binding
- **Reputation System**: Chain-based reputation with slash mechanisms
- **Multi-Agent Coordination**: Native support for agent-to-agent communication
- **STARK Verification**: Verify AI inference proofs on-chain

### ⚡ High Performance
- **6-second block time**: Fast transaction finality
- **10,000+ TPS**: Parallel execution optimization
- **GRANDPA finality**: Immediate transaction finality

## Architecture

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
│  └──────────────────────────────────────────────────────────┘   │
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
└─────────────────────────────────────────────────────────────────┘
```

## Project Structure

```
qylith/
├── primitives/
│   └── crypto/              # Post-quantum cryptography primitives
│       └── src/
│           ├── lib.rs       # Main module
│           ├── falcon.rs     # FALCON-1024 signature implementation
│           ├── ml_kem.rs     # ML-KEM-768 KEM implementation
│           ├── sr25519.rs    # SR25519 for hybrid signatures
│           └── traits.rs     # Crypto traits and registry
├── runtime/
│   └── src/
│       ├── lib.rs           # Runtime configuration
│       ├── constants.rs     # Runtime constants
│       └── pallets/
│           └── aem/          # AI Agent Execution Module
│               └── src/
│                   ├── lib.rs           # Pallet definition
│                   ├── types.rs         # Type definitions
│                   ├── storage.rs       # Storage definitions
│                   ├── events.rs        # Event definitions
│                   ├── errors.rs        # Error types
│                   ├── dispatchables.rs # Extrinsics
│                   ├── weights.rs       # Weight info
│                   └── benchmarking.rs  # Benchmarks
├── node/
│   └── src/
│       ├── main.rs          # Node entry point
│       ├── chain_spec.rs    # Chain specification
│       └── rpc.rs           # RPC configuration
├── scripts/                  # Build and utility scripts
├── Dockerfile
├── Makefile
├── justfile
└── README.md
```

## Building

### Prerequisites

- Rust (latest nightly)
- wasm32-unknown-unknown target
- Linux/macOS build tools

### Quick Start

```bash
# Clone the repository
git clone https://github.com/qylith/qylith.git
cd qylith

# Install prerequisites
make install-deps

# Build the node
make build

# Run in development mode
./target/release/qylith --dev --tmp
```

### Using Docker

```bash
# Build Docker image
make docker-build

# Run container
make docker-run
```

### Available Commands

```bash
make help              # Show all available commands
make build             # Build release binary
make check             # Check code without building
make test              # Run tests
make fmt               # Format code
make clippy            # Run linter
make lint              # Run all linting
make benchmark          # Run benchmarks
make docker-build      # Build Docker image
make dev               # Run in dev mode
make purge             # Purge chain data
```

## Development

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for specific pallet
cargo test -p pallet-aem

# Run with output
cargo test --workspace -- --nocapture
```

### Running in Development Mode

```bash
# Start with temporary database
./target/release/qylith --dev --tmp

# Start with persistent database
./target/release/qylith --dev

# Enable detailed logging
RUST_LOG=debug ./target/release/qylith --dev
```

### Generating Chain Spec

```bash
# Generate development spec
./target/release/qylith build-spec --chain development > chain-spec.json

# Convert to raw format
./target/release/qylith build-spec --chain development --raw > chain-spec-raw.json
```

## Cryptography

### FALCON-1024

FALCON (Fast Fourier Lattice-based Compact Signatures over NTRU) is the primary signature algorithm:

| Parameter | Value |
|-----------|-------|
| Security Level | NIST Level 5 |
| Signature Size | 1,280 bytes |
| Public Key Size | 1,793 bytes |
| Verification Time | ~0.15ms |
| Classical Security | ~2^128 |
| Quantum Security | ~2^85 |

### ML-KEM-768

ML-KEM (Module-Lattice Key Encapsulation Mechanism) is used for P2P encryption:

| Parameter | Value |
|-----------|-------|
| Security Level | NIST Level 3 |
| Public Key Size | 1,184 bytes |
| Ciphertext Size | 1,088 bytes |
| Shared Secret | 32 bytes |

## AEM (AI Agent Execution Module)

The AEM provides native support for AI agents:

### Agent Registry

```rust
// Register an AI agent
pallet_aem::register_agent(
    signing_key: FalconPublicKey,      // FALCON-1024 identity key
    encryption_key: MlKemPublicKey,   // ML-KEM-768 communication key
    metadata_uri: Vec<u8>,            // IPFS CID for metadata
    permission_level: PermissionLevel, // Execution permissions
)
```

### Permission Levels

- `ReadOnly`: Only read chain data
- `TradeOnly(TokenId)`: Only trade specific tokens
- `Unilateral`: Execute any transaction autonomously
- `Managed`: Requires owner confirmation

### Task Management

```rust
// Create a task
pallet_aem::create_task(
    input_uri: Vec<u8>,           // Task input data
    priority: TaskPriority,       // Low/Normal/High/Critical
    max_fee: Balance,            // Maximum fee
    deadline: BlockNumber,        // Task deadline
)

// Complete a task
pallet_aem::complete_task(
    task_id: TaskId,
    output_uri: Vec<u8>,          // Task output
)
```

## Roadmap

| Phase | Description | Status |
|-------|-------------|--------|
| 1 | Project skeleton, FALCON/ML-KEM primitives | ✅ Complete |
| 2 | AEM pallet implementation | 🚧 In Progress |
| 3 | Runtime integration, NPoS configuration | 📋 Planned |
| 4 | Testnet launch | 📋 Planned |
| 5 | Mainnet launch | 📋 Planned |
| 6 | STARK verification integration | 📋 Planned |
| 7 | Cross-chain bridges | 📋 Planned |

## Contributing

We welcome contributions! Please see our contributing guidelines:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linting
5. Submit a pull request

## Security

### Responsible Disclosure

If you discover any security vulnerabilities, please report them to security@qylith.io. We appreciate responsible disclosure and will work with you to address any issues.

### Cryptographic Considerations

- FALCON-1024 provides NIST Level 5 security
- ML-KEM-768 provides NIST Level 3 security
- Hybrid signatures combine PQC with classical for defense-in-depth
- Regular security audits are planned

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## References

- [FALCON Website](https://falcon-sign.info/)
- [ML-KEM Specification](https://pq-crystals.org/kyber/spec.shtml)
- [Substrate Documentation](https://docs.substrate.io/)
- [NIST Post-Quantum Cryptography](https://csrc.nist.gov/projects/post-quantum-cryptography)

## Contact

- Website: https://qylith.io
- Twitter: @QylithChain
- Discord: [Join](https://discord.gg/qylith)
- Email: contact@qylith.io

---

<p align="center">
  Built with ❤️ by the Qylith Team
</p>
