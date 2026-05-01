//! AI Agent Execution Module (AEM)
//!
//! This pallet provides the core infrastructure for AI-native blockchain operations
//! in Qylith. It enables:
//!
//! - **Agent Registry**: Register AI agents with on-chain identity
//! - **Permission Management**: Define agent execution permissions
//! - **Reputation System**: Track and reward agent behavior
//! - **Task Execution**: Coordinate agent task execution
//! - **Multi-Agent Coordination**: Enable agent-to-agent communication
//!
//! # Architecture
//!
//! ```
//! ┌─────────────────────────────────────────────────────────────┐
//! │                   AEM Pallet                                │
//! ├─────────────────────────────────────────────────────────────┤
//! │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
//! │  │Agent Registry │  │  Reputation  │  │   Task       │    │
//! │  │              │  │   System     │  │  Scheduler   │    │
//! │  └──────────────┘  └──────────────┘  └──────────────┘    │
//! │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
//! │  │  Permission  │  │  Multi-Agent│  │   STARK      │    │
//! │  │  Manager     │  │  Coordinator │  │  Verifier    │    │
//! │  └──────────────┘  └──────────────┘  └──────────────┘    │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Security Model
//!
//! - Agents must stake QYL to register
//! - Malicious behavior results in stake slashing
//! - Reputation affects task allocation
//! - Permission levels control execution scope
//!
//! # References
//!
//! See the architecture design document for full specifications.

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "512"]
// Edit this file to define custom pallets, if not needed.
// Delete all the unused code that's no longer needed.
pub mod benchmarking;
pub mod weights;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

// --- Substrate/Polkadot imports ---
pub use frame_support::{construct_runtime, pallet_prelude::*, parameter_types};
pub use frame_system::pallet_prelude::*;
pub use sp_runtime::{traits::*, RuntimeDebug};
pub use sp_std::prelude::*;

// --- Qylith imports ---
pub use qylith_primitives::{falcon::FalconPublicKey, ml_kem::MlKemPublicKey};

// --- Pallet internal modules ---
pub mod types;
pub mod dispatchables;
pub mod storage;
pub mod events;
pub mod errors;

pub use types::*;
pub use dispatchables::*;
pub use storage::*;
pub use events::*;
pub use errors::*;

/// Minimum stake required to register an agent
pub const MIN_AGENT_STAKE: u128 = 1_000 * 10u128.pow(18); // 1000 QYL

/// Minimum reputation score
pub const MIN_REPUTATION: i32 = 0;

/// Maximum reputation score
pub const MAX_REPUTATION: i32 = 10000;

/// Reputation points per successful task
pub const REPUTATION_REWARD_SUCCESS: i32 = 10;

/// Reputation penalty for failed task
pub const REPUTATION_PENALTY_FAILED: i32 = 5;

/// Reputation penalty for malicious behavior
pub const REPUTATION_PENALTY_MALICIOUS: i32 = 100;

/// Slash percentage for agent misbehavior (in basis points)
pub const SLASH_BASIS_POINTS: u32 = 1000; // 10%

/// Maximum tasks an agent can have pending
pub const MAX_PENDING_TASKS: u32 = 100;

/// Configure the AEM pallet in a runtime.
pub trait Config: frame_system::Config {
    /// The runtime event type.
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    
    /// The currency mechanism (for staking)
    type Currency: ReservableCurrency<Self::AccountId>;
    
    /// Weight information for extrinsic calls
    type WeightInfo: WeightInfo;
    
    /// Maximum permission levels
    type MaxPermissionLevel: Get<u32>;
    
    /// Maximum metadata size
    type MaxMetadataSize: Get<u32>;
}
