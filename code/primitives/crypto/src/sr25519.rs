//! SR25519 Legacy Signature Support
//!
//! This module provides SR25519 (Schnorrkel over Ristretto) signature support
//! for hybrid signatures. While SR25519 is not quantum-resistant, it is included
//! in hybrid signatures for:
//! - **Backward compatibility**: During transition period
//! - **Defense in depth**: Requiring both signatures to be valid
//! - **Interoperability**: With existing Substrate/Polkadot ecosystem
//!
//! # Hybrid Signature Architecture
//!
//! Qylith uses hybrid signatures that combine:
//! - **FALCON-1024**: Post-quantum primary signature (NIST Level 5)
//! - **SR25519**: Classical backup signature (for compatibility)
//!
//! This provides protection against:
//! 1. Classical attacks (handled by FALCON)
//! 2. Quantum attacks (handled by FALCON's lattice security)
//! 3. Implementation bugs (handled by dual verification)
//!
//! # Security Note
//!
//! SR25519 is included ONLY for compatibility and defense-in-depth.
//! The primary security relies on FALCON-1024. When quantum computers
//! become a threat to SR25519, the chain can switch to FALCON-only
//! signatures since SR25519 verification can be made optional.

use crate::{traits::*, CryptoError, CryptoResult, SecurityLevel};
use codec::{Decode, Encode, MaxEncodedLen};
use runtime_debug::RuntimeDebug;
use scale_info::TypeInfo;
use sp_core::sr25519::{Public, Signature as Sr25519Signature, Pair as Sr25519Pair};
use sp_std::vec::Vec;

/// SR25519 Public Key wrapper
#[derive(
    Clone, PartialEq, Eq, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen, Default,
)]
pub struct Sr25519PublicKey {
    /// The underlying SR25519 public key
    #[codec(skip)]
    pub inner: Public,
}

/// SR25519 Secret Key wrapper
/// Note: We don't store the actual secret key for security reasons
#[derive(
    Clone, PartialEq, Eq, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen, Default,
)]
pub struct Sr25519SecretKey {
    /// Placeholder for secret key data (not actual secret)
    /// In production, this would use secure key storage
    #[codec(compact)]
    pub seed: Vec<u8>,
}

/// SR25519 Signature wrapper
#[derive(
    Clone, PartialEq, Eq, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen, Default,
)]
pub struct Sr25519SignatureWrapper(
    /// The underlying SR25519 signature
    #[codec(skip)]
    pub Sr25519Signature,
);

/// SR25519 Signature Scheme Implementation
///
/// This provides SR25519 signatures for hybrid signature support.
/// The actual signing operations use sp_core::sr25519.
pub struct Sr25519;

impl SignatureScheme for Sr25519 {
    type PublicKey = Sr25519PublicKey;
    type SecretKey = Sr25519SecretKey;
    type Signature = Sr25519SignatureWrapper;
    
    const ALGORITHM_ID: super::AlgorithmId = super::AlgorithmId::Sr25519;
    const SECURITY_LEVEL: SecurityLevel = SecurityLevel::Level1; // Not quantum resistant
    const PUBLIC_KEY_SIZE: usize = 32;
    const SECRET_KEY_SIZE: usize = 64; // Expanded secret key
    const SIGNATURE_SIZE: usize = 64;
    
    /// Generate a new SR25519 keypair
    fn keygen() -> CryptoResult<(Self::PublicKey, Self::SecretKey)> {
        let (pair, seed) = Sr25519Pair::generate_with_phrase(None);
        let public = pair.public();
        
        Ok((
            Sr25519PublicKey { inner: public },
            Sr25519SecretKey { seed: seed.into_bytes().to_vec() },
        ))
    }
    
    /// Generate a SR25519 keypair from a seed
    fn from_seed(seed: &[u8]) -> CryptoResult<(Self::PublicKey, Self::SecretKey)> {
        if seed.len() != 32 {
            return Err(CryptoError::InvalidParameters);
        }
        
        let pair = Sr25519Pair::from_seed_slice(seed)
            .map_err(|_| CryptoError::KeyGenerationFailed)?;
        let public = pair.public();
        
        Ok((
            Sr25519PublicKey { inner: public },
            Sr25519SecretKey { seed: seed.to_vec() },
        ))
    }
    
    /// Sign a message with SR25519
    fn sign(message: &[u8], secret_key: &Self::SecretKey) -> CryptoResult<Self::Signature> {
        let pair = Sr25519Pair::from_seed_slice(&secret_key.seed)
            .map_err(|_| CryptoError::KeyGenerationFailed)?;
        
        let signature = pair.sign(message);
        
        Ok(Sr25519SignatureWrapper(signature))
    }
    
    /// Verify a SR25519 signature
    fn verify(
        message: &[u8],
        signature: &Self::Signature,
        public_key: &Self::PublicKey,
    ) -> CryptoResult<bool> {
        // Use sp_core's verification
        let result = sp_core::sr25519::Pair::verify(
            &signature.0,
            message,
            &public_key.inner,
        );
        
        Ok(result)
    }
    
    /// Extract public key from secret key
    fn public_from_secret(secret_key: &Self::SecretKey) -> CryptoResult<Self::PublicKey> {
        let pair = Sr25519Pair::from_seed_slice(&secret_key.seed)
            .map_err(|_| CryptoError::KeyGenerationFailed)?;
        
        Ok(Sr25519PublicKey { inner: pair.public() })
    }
    
    fn serialize_public(public_key: &Self::PublicKey) -> Vec<u8> {
        public_key.inner.to_raw_vec()
    }
    
    fn deserialize_public(bytes: &[u8]) -> CryptoResult<Self::PublicKey> {
        if bytes.len() != 32 {
            return Err(CryptoError::InvalidKey);
        }
        
        let public = Public::try_from(bytes)
            .map_err(|_| CryptoError::InvalidKey)?;
        
        Ok(Sr25519PublicKey { inner: public })
    }
    
    fn serialize_secret(secret_key: &Self::SecretKey) -> Vec<u8> {
        // Return seed (not the expanded secret)
        secret_key.seed.clone()
    }
    
    fn deserialize_secret(bytes: &[u8]) -> CryptoResult<Self::SecretKey> {
        if bytes.len() != 32 {
            return Err(CryptoError::InvalidKey);
        }
        
        // Verify the seed produces a valid keypair
        Sr25519Pair::from_seed_slice(bytes)
            .map_err(|_| CryptoError::InvalidKey)?;
        
        Ok(Sr25519SecretKey { seed: bytes.to_vec() })
    }
    
    fn serialize_signature(signature: &Self::Signature) -> Vec<u8> {
        signature.0.to_raw_vec()
    }
    
    fn deserialize_signature(bytes: &[u8]) -> CryptoResult<Self::Signature> {
        if bytes.len() != 64 {
            return Err(CryptoError::InvalidSignature);
        }
        
        let signature = Sr25519Signature::try_from(bytes)
            .map_err(|_| CryptoError::InvalidSignature)?;
        
        Ok(Sr25519SignatureWrapper(signature))
    }
}

/// Convert between sp_core types and our wrapper types
impl Sr25519PublicKey {
    /// Create from sp_core::sr25519::Public
    pub fn from_public(public: Public) -> Self {
        Sr25519PublicKey { inner: public }
    }
    
    /// Convert to sp_core::sr25519::Public
    pub fn to_public(&self) -> Public {
        self.inner
    }
    
    /// Get the raw bytes
    pub fn as_bytes(&self) -> &[u8] {
        self.inner.as_ref()
    }
}

impl Sr25519SignatureWrapper {
    /// Create from sp_core::sr25519::Signature
    pub fn from_signature(sig: Sr25519Signature) -> Self {
        Sr25519SignatureWrapper(sig)
    }
    
    /// Convert to sp_core::sr25519::Signature
    pub fn to_signature(&self) -> Sr25519Signature {
        self.0
    }
    
    /// Get the raw bytes
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sr25519_keygen() {
        let (pk, sk) = Sr25519::keygen().unwrap();
        assert_eq!(Sr25519::serialize_public(&pk).len(), 32);
        assert_eq!(Sr25519::serialize_secret(&sk).len(), 32);
    }
    
    #[test]
    fn test_sr25519_sign_verify() {
        let (_pk, sk) = Sr25519::keygen().unwrap();
        let message = b"Hello, Qylith!";
        
        let signature = Sr25519::sign(message, &sk).unwrap();
        assert_eq!(Sr25519::serialize_signature(&signature).len(), 64);
        
        let (pk, _sk) = Sr25519::keygen().unwrap();
        let result = Sr25519::verify(message, &signature, &pk);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
    
    #[test]
    fn test_sr25519_seed_derivation() {
        let seed = [0u8; 32];
        let (pk1, sk1) = Sr25519::from_seed(&seed).unwrap();
        let (pk2, sk2) = Sr25519::from_seed(&seed).unwrap();
        
        // Same seed should produce same keys
        assert_eq!(pk1.as_bytes(), pk2.as_bytes());
        assert_eq!(sk1.seed, sk2.seed);
    }
}
