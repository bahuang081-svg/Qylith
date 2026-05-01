//! AEM Type Definitions
//!
//! This module defines all the core types used by the AEM pallet,
//! including agent identities, permissions, tasks, and reputation.

use crate::*;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// Unique identifier for an AI agent
/// Derived from the agent's FALCON public key hash
#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Encode, Decode, TypeInfo, MaxEncodedLen,
    Default, RuntimeDebug,
)]
pub struct AgentId(pub [u8; 32]);

impl AgentId {
    /// Create a new AgentId from bytes
    pub fn new(id: [u8; 32]) -> Self {
        AgentId(id)
    }
    
    /// Get the underlying bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl From<sp_core::H256> for AgentId {
    fn from(hash: sp_core::H256) -> Self {
        AgentId(hash.into_inner())
    }
}

impl AsRef<[u8]> for AgentId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Unique identifier for a task
#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Encode, Decode, TypeInfo, MaxEncodedLen,
    Default, RuntimeDebug,
)]
pub struct TaskId(pub [u8; 32]);

impl TaskId {
    /// Create a new TaskId from bytes
    pub fn new(id: [u8; 32]) -> Self {
        TaskId(id)
    }
}

impl From<sp_core::H256> for TaskId {
    fn from(hash: sp_core::H256) -> Self {
        TaskId(hash.into_inner())
    }
}

/// Unique identifier for a model
/// Used for STARK verification of AI model inference
#[derive(
    Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, Default, RuntimeDebug,
)]
pub struct ModelId(pub [u8; 32]);

impl ModelId {
    /// Create a new ModelId from bytes
    pub fn new(id: [u8; 32]) -> Self {
        ModelId(id)
    }
    
    /// Get the underlying bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl From<sp_core::H256> for ModelId {
    fn from(hash: sp_core::H256) -> Self {
        ModelId(hash.into_inner())
    }
}

/// Agent permission levels
/// Defines what operations an agent is authorized to perform
#[derive(
    Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug,
)]
#[repr(u8)]
pub enum PermissionLevel {
    /// Agent can only read chain data, no write operations
    ReadOnly = 0,
    /// Agent can only interact with specific tokens
    TradeOnly = 1,
    /// Agent can execute any transaction autonomously
    Unilateral = 2,
    /// Agent requires owner confirmation for each operation
    Managed = 3,
}

impl Default for PermissionLevel {
    fn default() -> Self {
        PermissionLevel::ReadOnly
    }
}

/// Agent registration data
/// Stored on-chain when an agent registers
#[derive(
    Clone, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug,
)]
#[scale_info(skip_type_params(T))]
pub struct AgentRegistration<T: Config> {
    /// The owner account (human) who registered this agent
    pub owner: T::AccountId,
    
    /// FALCON-1024 public key for agent identity
    /// Used for signing agent transactions
    pub signing_key: FalconPublicKey,
    
    /// ML-KEM-768 public key for encrypted communications
    pub encryption_key: MlKemPublicKey,
    
    /// IPFS CID or similar reference to agent metadata
    /// Contains agent description, capabilities, etc.
    pub metadata_uri: BoundedVec<u8, T::MaxMetadataSize>,
    
    /// Permission level for this agent
    pub permission_level: PermissionLevel,
    
    /// For TradeOnly permission: allowed token IDs
    /// Empty for other permission levels
    pub allowed_tokens: BoundedVec<TokenId, T::MaxPermissionLevel>,
    
    /// Block number when agent was registered
    pub registered_at: BlockNumberOf<T>,
    
    /// Whether the agent is currently active
    pub is_active: bool,
}

/// Token identifier for TradeOnly permission
pub type TokenId = u32;

/// Agent reputation data
/// Tracks agent reliability and performance
#[derive(
    Clone, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug,
)]
pub struct ReputationData {
    /// Current reputation score (0 to MAX_REPUTATION)
    pub score: i32,
    
    /// Total number of tasks completed successfully
    pub tasks_completed: u64,
    
    /// Total number of tasks failed
    pub tasks_failed: u64,
    
    /// Number of times agent was slashed
    pub slash_count: u32,
    
    /// Block number of last task update
    pub last_update: BlockNumber,
}

impl Default for ReputationData {
    fn default() -> Self {
        ReputationData {
            score: 1000, // Start with neutral reputation
            tasks_completed: 0,
            tasks_failed: 0,
            slash_count: 0,
            last_update: 0,
        }
    }
}

impl ReputationData {
    /// Update reputation after a successful task
    pub fn record_success(&mut self, block: BlockNumber) {
        self.tasks_completed += 1;
        self.score = self.score.saturating_add(crate::REPUTATION_REWARD_SUCCESS);
        self.score = self.score.min(crate::MAX_REPUTATION);
        self.last_update = block;
    }
    
    /// Update reputation after a failed task
    pub fn record_failure(&mut self, block: BlockNumber) {
        self.tasks_failed += 1;
        self.score = self.score.saturating_sub(crate::REPUTATION_PENALTY_FAILED);
        self.score = self.score.max(crate::MIN_REPUTATION);
        self.last_update = block;
    }
    
    /// Update reputation after malicious behavior (slash)
    pub fn record_slash(&mut self, block: BlockNumber) {
        self.slash_count += 1;
        self.score = self.score.saturating_sub(crate::REPUTATION_PENALTY_MALICIOUS);
        self.score = self.score.max(crate::MIN_REPUTATION);
        self.last_update = block;
    }
    
    /// Calculate success rate
    pub fn success_rate(&self) -> Option<sp_runtime::Perbill> {
        let total = self.tasks_completed.saturating_add(self.tasks_failed);
        if total == 0 {
            None
        } else {
            Some(sp_runtime::Perbill::from_rational(
                self.tasks_completed,
                total,
            ))
        }
    }
}

/// Task status enumeration
#[derive(
    Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug,
)]
#[repr(u8)]
pub enum TaskStatus {
    /// Task is pending execution
    Pending = 0,
    /// Task is currently being executed
    InProgress = 1,
    /// Task completed successfully
    Completed = 2,
    /// Task failed
    Failed = 3,
    /// Task was cancelled
    Cancelled = 4,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Pending
    }
}

/// Task priority levels
#[derive(
    Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug,
)]
#[repr(u8)]
pub enum TaskPriority {
    /// Low priority task
    Low = 0,
    /// Normal priority task
    Normal = 1,
    /// High priority task
    High = 2,
    /// Critical task (slashable if not completed)
    Critical = 3,
}

impl Default for TaskPriority {
    fn default() -> Self {
        TaskPriority::Normal
    }
}

/// Task definition
#[derive(
    Clone, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug,
)]
pub struct Task<T: Config> {
    /// Unique task identifier
    pub id: TaskId,
    
    /// The agent assigned to this task
    pub agent_id: Option<AgentId>,
    
    /// Who created this task
    pub creator: T::AccountId,
    
    /// Task priority
    pub priority: TaskPriority,
    
    /// Current status
    pub status: TaskStatus,
    
    /// IPFS CID to task specification/input data
    pub input_uri: BoundedVec<u8, T::MaxMetadataSize>,
    
    /// IPFS CID to expected output (for verification)
    pub expected_output_uri: Option<BoundedVec<u8, T::MaxMetadataSize>>,
    
    /// Maximum gas/fee allowed for this task
    pub max_fee: BalanceOf<T>,
    
    /// Deadline block number
    pub deadline: BlockNumberOf<T>,
    
    /// Block number when task was created
    pub created_at: BlockNumberOf<T>,
    
    /// Block number when task was completed
    pub completed_at: Option<BlockNumberOf<T>>,
}

/// Model specification for STARK verification
#[derive(
    Clone, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug,
)]
pub struct ModelSpec {
    /// Unique model identifier
    pub id: ModelId,
    
    /// SHA-256 hash of model weights
    pub model_hash: [u8; 32],
    
    /// Model type (e.g., "gpt-4", "llama-2-70b")
    pub model_type: Vec<u8>,
    
    /// Input schema (JSON schema)
    pub input_schema: Vec<u8>,
    
    /// Output schema (JSON schema)
    pub output_schema: Vec<u8>,
    
    /// Maximum execution steps allowed
    pub max_steps: u32,
    
    /// Whether this model is allowed for inference
    pub is_allowed: bool,
}

impl Default for ModelSpec {
    fn default() -> Self {
        ModelSpec {
            id: ModelId([0u8; 32]),
            model_hash: [0u8; 32],
            model_type: Vec::new(),
            input_schema: Vec::new(),
            output_schema: Vec::new(),
            max_steps: 1000,
            is_allowed: true,
        }
    }
}

/// Agent message for multi-agent communication
#[derive(
    Clone, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug,
)]
pub struct AgentMessage<T: Config> {
    /// Sender agent ID
    pub from: AgentId,
    
    /// Recipient agent ID
    pub to: AgentId,
    
    /// Message type
    pub message_type: MessageType,
    
    /// Encrypted payload
    pub payload: BoundedVec<u8, T::MaxMetadataSize>,
    
    /// Nonce for replay protection
    pub nonce: u64,
    
    /// FALCON signature from sender
    pub signature: Vec<u8>,
    
    /// Block number when message was sent
    pub timestamp: BlockNumberOf<T>,
}

/// Message types for agent-to-agent communication
#[derive(
    Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug,
)]
#[repr(u8)]
pub enum MessageType {
    /// Direct task request
    TaskRequest = 0,
    /// Task response
    TaskResponse = 1,
    /// Collaboration offer
    CollaborationOffer = 2,
    /// Collaboration response
    CollaborationResponse = 3,
    /// General inquiry
    Inquiry = 4,
    /// General response
    Response = 5,
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::Inquiry
    }
}

/// Agent statistics for display/ranking
#[derive(
    Clone, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug,
)]
pub struct AgentStats {
    /// Total tasks completed
    pub total_tasks: u64,
    
    /// Current reputation score
    pub reputation: i32,
    
    /// Success rate (as perbill)
    pub success_rate: u32, // per million
    
    /// Average task completion time (in blocks)
    pub avg_completion_time: u32,
    
    /// Total fees earned
    pub total_fees_earned: u128,
}

impl Default for AgentStats {
    fn default() -> Self {
        AgentStats {
            total_tasks: 0,
            reputation: 1000,
            success_rate: 0,
            avg_completion_time: 0,
            total_fees_earned: 0,
        }
    }
}
