//! Common cryptographic traits for Qylith
//!
//! This module defines the trait hierarchy for all cryptographic operations in Qylith.
//! It provides abstraction layers for:
//! - Digital signatures (signing and verification)
//! - Key encapsulation mechanisms (KEM)
//! - Hybrid signatures (combining PQC with classical algorithms)

use sp_core::{crypto::Pair as _, RuntimeDebug};
use sp_std::vec::Vec;

/// Result type for cryptographic operations
pub type CryptoResult<T> = Result<T, CryptoError>;

/// Errors that can occur during cryptographic operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CryptoError {
    /// Invalid key format or length
    InvalidKey,
    /// Invalid signature format or length
    InvalidSignature,
    /// Verification failed
    VerificationFailed,
    /// Key generation failed
    KeyGenerationFailed,
    /// Encryption failed
    EncryptionFailed,
    /// Decryption failed
    DecryptionFailed,
    /// Encapsulation failed
    EncapsulationFailed,
    /// Decapsulation failed
    DecapsulationFailed,
    /// Algorithm not supported
    AlgorithmNotSupported,
    /// Invalid parameters
    InvalidParameters,
    /// Random number generation failed
    RandomGenerationFailed,
}

impl sp_std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
        match self {
            CryptoError::InvalidKey => write!(f, "Invalid key format or length"),
            CryptoError::InvalidSignature => write!(f, "Invalid signature format or length"),
            CryptoError::VerificationFailed => write!(f, "Signature verification failed"),
            CryptoError::KeyGenerationFailed => write!(f, "Key generation failed"),
            CryptoError::EncryptionFailed => write!(f, "Encryption failed"),
            CryptoError::DecryptionFailed => write!(f, "Decryption failed"),
            CryptoError::EncapsulationFailed => write!(f, "Key encapsulation failed"),
            CryptoError::DecapsulationFailed => write!(f, "Key decapsulation failed"),
            CryptoError::AlgorithmNotSupported => write!(f, "Algorithm not supported"),
            CryptoError::InvalidParameters => write!(f, "Invalid cryptographic parameters"),
            CryptoError::RandomGenerationFailed => write!(f, "Random number generation failed"),
        }
    }
}

impl From<CryptoError> for &'static str {
    fn from(e: CryptoError) -> &'static str {
        match e {
            CryptoError::InvalidKey => "InvalidKey",
            CryptoError::InvalidSignature => "InvalidSignature",
            CryptoError::VerificationFailed => "VerificationFailed",
            CryptoError::KeyGenerationFailed => "KeyGenerationFailed",
            CryptoError::EncryptionFailed => "EncryptionFailed",
            CryptoError::DecryptionFailed => "DecryptionFailed",
            CryptoError::EncapsulationFailed => "EncapsulationFailed",
            CryptoError::DecapsulationFailed => "DecapsulationFailed",
            CryptoError::AlgorithmNotSupported => "AlgorithmNotSupported",
            CryptoError::InvalidParameters => "InvalidParameters",
            CryptoError::RandomGenerationFailed => "RandomGenerationFailed",
        }
    }
}

/// Trait for digital signature schemes
///
/// This trait defines the interface for all signature algorithms used in Qylith,
/// including post-quantum algorithms like FALCON-1024 and classical algorithms
/// like SR25519 for hybrid signatures.
pub trait SignatureScheme {
    /// The public key type for this signature scheme
    type PublicKey: Clone + PartialEq + RuntimeDebug + codec::Encode + codec::Decode + Default;
    
    /// The secret/private key type for this signature scheme
    type SecretKey: Clone + PartialEq + RuntimeDebug + codec::Encode + codec::Decode;
    
    /// The signature type produced by this scheme
    type Signature: Clone + PartialEq + RuntimeDebug + codec::Encode + codec::Decode + Default;
    
    /// The algorithm identifier for this scheme
    const ALGORITHM_ID: super::AlgorithmId;
    
    /// The security level of this signature scheme
    const SECURITY_LEVEL: super::SecurityLevel;
    
    /// Size of the public key in bytes
    const PUBLIC_KEY_SIZE: usize;
    
    /// Size of the secret key in bytes
    const SECRET_KEY_SIZE: usize;
    
    /// Size of the signature in bytes
    const SIGNATURE_SIZE: usize;
    
    /// Generate a new keypair from randomness
    fn keygen() -> CryptoResult<(Self::PublicKey, Self::SecretKey)>;
    
    /// Generate a keypair from a seed
    fn from_seed(seed: &[u8]) -> CryptoResult<(Self::PublicKey, Self::SecretKey)>;
    
    /// Sign a message with the secret key
    fn sign(message: &[u8], secret_key: &Self::SecretKey) -> CryptoResult<Self::Signature>;
    
    /// Verify a signature against a message and public key
    fn verify(
        message: &[u8],
        signature: &Self::Signature,
        public_key: &Self::PublicKey,
    ) -> CryptoResult<bool>;
    
    /// Get the public key from a secret key
    fn public_from_secret(secret_key: &Self::SecretKey) -> CryptoResult<Self::PublicKey>;
    
    /// Serialize a public key to bytes
    fn serialize_public(public_key: &Self::PublicKey) -> Vec<u8>;
    
    /// Deserialize a public key from bytes
    fn deserialize_public(bytes: &[u8]) -> CryptoResult<Self::PublicKey>;
    
    /// Serialize a secret key to bytes (should be used carefully)
    fn serialize_secret(secret_key: &Self::SecretKey) -> Vec<u8>;
    
    /// Deserialize a secret key from bytes
    fn deserialize_secret(bytes: &[u8]) -> CryptoResult<Self::SecretKey>;
    
    /// Serialize a signature to bytes
    fn serialize_signature(signature: &Self::Signature) -> Vec<u8>;
    
    /// Deserialize a signature from bytes
    fn deserialize_signature(bytes: &[u8]) -> CryptoResult<Self::Signature>;
}

/// Trait for Key Encapsulation Mechanisms (KEM)
///
/// KEMs are used for key exchange and P2P encryption in Qylith.
/// ML-KEM-768 is the primary KEM algorithm.
pub trait KeyEncapsulationScheme {
    /// The public key type for this KEM
    type PublicKey: Clone + PartialEq + RuntimeDebug + codec::Encode + codec::Decode + Default;
    
    /// The secret key type for this KEM
    type SecretKey: Clone + PartialEq + RuntimeDebug + codec::Encode + codec::Decode;
    
    /// The ciphertext type produced by encapsulation
    type Ciphertext: Clone + PartialEq + RuntimeDebug + codec::Encode + codec::Decode + Default;
    
    /// The shared secret produced by encapsulation/decapsulation
    type SharedSecret: Clone + PartialEq + RuntimeDebug + AsRef<[u8]>;
    
    /// The algorithm identifier for this KEM
    const ALGORITHM_ID: super::AlgorithmId;
    
    /// Size of the public key in bytes
    const PUBLIC_KEY_SIZE: usize;
    
    /// Size of the secret key in bytes
    const SECRET_KEY_SIZE: usize;
    
    /// Size of the ciphertext in bytes
    const CIPHERTEXT_SIZE: usize;
    
    /// Size of the shared secret in bytes
    const SHARED_SECRET_SIZE: usize;
    
    /// Generate a new keypair for encapsulation
    fn keygen() -> CryptoResult<(Self::PublicKey, Self::SecretKey)>;
    
    /// Encapsulate a shared secret for the given public key
    /// Returns (ciphertext, shared_secret)
    fn encapsulate(public_key: &Self::PublicKey) -> CryptoResult<(Self::Ciphertext, Self::SharedSecret)>;
    
    /// Decapsulate a ciphertext to recover the shared secret
    fn decapsulate(ciphertext: &Self::Ciphertext, secret_key: &Self::SecretKey) -> CryptoResult<Self::SharedSecret>;
    
    /// Serialize a public key to bytes
    fn serialize_public(public_key: &Self::PublicKey) -> Vec<u8>;
    
    /// Deserialize a public key from bytes
    fn deserialize_public(bytes: &[u8]) -> CryptoResult<Self::PublicKey>;
    
    /// Serialize a secret key to bytes
    fn serialize_secret(secret_key: &Self::SecretKey) -> Vec<u8>;
    
    /// Deserialize a secret key from bytes
    fn deserialize_secret(bytes: &[u8]) -> CryptoResult<Self::SecretKey>;
    
    /// Serialize a ciphertext to bytes
    fn serialize_ciphertext(ciphertext: &Self::Ciphertext) -> Vec<u8>;
    
    /// Deserialize a ciphertext from bytes
    fn deserialize_ciphertext(bytes: &[u8]) -> CryptoResult<Self::Ciphertext>;
}

/// Hybrid signature structure combining PQC and classical signatures
///
/// This provides defense-in-depth by requiring both FALCON-1024 and SR25519
/// signatures to be valid. This is especially important during the transition
/// period before full quantum resistance is critical.
#[derive(RuntimeDebug, Clone, PartialEq, Eq, codec::Encode, codec::Decode, Default)]
pub struct HybridSignature<PQC, Classical> {
    /// The post-quantum signature (FALCON-1024)
    pub pqc_signature: PQC,
    /// The classical signature (SR25519) for compatibility
    pub classical_signature: Classical,
    /// Timestamp when the signature was created
    pub timestamp: u64,
}

impl<PQC: Clone, Classical: Clone> HybridSignature<PQC, Classical> {
    /// Create a new hybrid signature
    pub fn new(pqc_signature: PQC, classical_signature: Classical, timestamp: u64) -> Self {
        Self {
            pqc_signature,
            classical_signature,
            timestamp,
        }
    }
}

/// Crypto registry for managing multiple algorithms
///
/// This enables "crypto-agility" - the ability to switch between algorithms
/// or support multiple algorithms simultaneously.
pub struct CryptoRegistry<SS: SignatureScheme, KEM: KeyEncapsulationScheme> {
    /// Registered signature schemes
    signing_algorithms: sp_std::collections::btree_map::BTreeMap<
        super::AlgorithmId,
        Box<dyn Fn() -> Result<(), CryptoError>>,
    >,
    /// Registered KEM schemes
    kem_algorithms: sp_std::collections::btree_map::BTreeMap<
        super::AlgorithmId,
        Box<dyn Fn() -> Result<(), CryptoError>>,
    >,
    /// Default signature algorithm
    default_signing: super::AlgorithmId,
    /// Default KEM algorithm
    default_kem: super::AlgorithmId,
    _phantom: sp_std::marker::PhantomData<(SS, KEM)>,
}

impl<SS: SignatureScheme + 'static, KEM: KeyEncapsulationScheme + 'static> CryptoRegistry<SS, KEM> {
    /// Create a new crypto registry with default algorithms
    pub fn new() -> Self {
        let mut registry = Self {
            signing_algorithms: sp_std::collections::btree_map::BTreeMap::new(),
            kem_algorithms: sp_std::collections::btree_map::BTreeMap::new(),
            default_signing: super::AlgorithmId::Falcon1024,
            default_kem: super::AlgorithmId::MlKem768,
            _phantom: sp_std::marker::PhantomData,
        };
        
        // Register default algorithms
        registry.register_signing_algorithm(super::AlgorithmId::Falcon1024);
        registry.register_signing_algorithm(super::AlgorithmId::Sr25519);
        registry.register_kem_algorithm(super::AlgorithmId::MlKem768);
        
        registry
    }
    
    /// Register a signature algorithm
    pub fn register_signing_algorithm(&mut self, algorithm_id: super::AlgorithmId) {
        self.signing_algorithms.insert(
            algorithm_id,
            Box::new(|| Ok(())),
        );
    }
    
    /// Register a KEM algorithm
    pub fn register_kem_algorithm(&mut self, algorithm_id: super::AlgorithmId) {
        self.kem_algorithms.insert(
            algorithm_id,
            Box::new(|| Ok(())),
        );
    }
    
    /// Check if a signing algorithm is registered
    pub fn is_signing_algorithm_registered(&self, algorithm_id: super::AlgorithmId) -> bool {
        self.signing_algorithms.contains_key(&algorithm_id)
    }
    
    /// Check if a KEM algorithm is registered
    pub fn is_kem_algorithm_registered(&self, algorithm_id: super::AlgorithmId) -> bool {
        self.kem_algorithms.contains_key(&algorithm_id)
    }
    
    /// Get the default signing algorithm
    pub fn default_signing_algorithm(&self) -> super::AlgorithmId {
        self.default_signing
    }
    
    /// Get the default KEM algorithm
    pub fn default_kem_algorithm(&self) -> super::AlgorithmId {
        self.default_kem
    }
}

impl<SS: SignatureScheme, KEM: KeyEncapsulationScheme> Default for CryptoRegistry<SS, KEM> {
    fn default() -> Self {
        Self::new()
    }
}
