//! Qylith Cryptography Primitives
//!
//! This module provides the foundational cryptographic operations for the Qylith blockchain:
//! - **FALCON-1024**: Post-quantum digital signatures (main signature algorithm)
//! - **ML-KEM-768**: Post-quantum key encapsulation mechanism (for P2P encryption)
//!
//! # Architecture
//!
//! The crypto layer is designed with "Crypto-Agility" in mind, allowing for future
//! algorithm upgrades without breaking changes. All algorithms implement common traits
//! for uniform access.
//!
//! # Security
//!
//! - FALCON-1024: NIST Level 5, ~2^128 classical security, ~2^85 quantum security
//! - ML-KEM-768: NIST Level 3, used for key encapsulation (not signatures)
//!
//! # References
//!
//! - [FALCON: Fast Fourier Lattice-based Compact Signatures over NTRU](https://falcon-sign.info/)
//! - [ML-KEM: Module-Lattice Key Encapsulation Mechanism](https://pq-crystals.org/kyber/)
//! - [NIST Post-Quantum Cryptography Standards](https://csrc.nist.gov/projects/post-quantum-cryptography)

#![cfg_attr(not(feature = "std"), no_std)]

pub mod falcon;
pub mod ml_kem;
pub mod traits;
pub mod sr25519; // Legacy support with hybrid signatures

pub use falcon::*;
pub use ml_kem::*;
pub use traits::*;

/// Algorithm identifiers for the crypto registry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlgorithmId {
    /// FALCON-1024 - Main post-quantum signature (NIST Level 5)
    Falcon1024,
    /// FALCON-512 - Lightweight post-quantum signature (NIST Level 1)
    Falcon512,
    /// ML-DSA-87 - Backup post-quantum signature (NIST Level 5)
    MlDsa87,
    /// ML-KEM-768 - Post-quantum key encapsulation (NIST Level 3)
    MlKem768,
    /// SR25519 - Legacy schnorrkel signature (non-PQC, for compatibility)
    Sr25519,
}

impl Default for AlgorithmId {
    fn default() -> Self {
        // Default to FALCON-1024 as the main signature algorithm
        AlgorithmId::Falcon1024
    }
}

/// Security level enumeration matching NIST PQC standards
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    /// NIST Level 1: ~2^80 classical security
    Level1,
    /// NIST Level 3: ~2^96 classical security
    Level3,
    /// NIST Level 5: ~2^128 classical security
    Level5,
}

impl SecurityLevel {
    /// Returns true if this security level is quantum-resistant
    pub fn is_quantum_resistant(&self) -> bool {
        matches!(self, Self::Level1 | Self::Level3 | Self::Level5)
    }
}

/// Type alias for account IDs using FALCON public keys
pub type AccountId = sp_core::crypto::AccountId32;

/// Type alias for block number
pub type BlockNumber = u32;

/// Type alias for index (nonce)
pub type Index = u32;

/// Type alias for balance
pub type Balance = u128;

/// Hash type for the blockchain
pub type Hash = sp_core::H256;

/// Balance type for runtime
pub type BalanceOf<T> = <T as frame_system::Config>::Currency;

/// Account ID type for runtime
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

// Re-export commonly used types
pub use sp_core::{ed25519, sr25519, ecdsa};
