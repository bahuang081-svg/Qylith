//! AEM Storage Definitions
//!
//! This module defines all storage items used by the AEM pallet.
//! Storage is organized by:
//! - Agent registry
//! - Reputation system
//! - Task management
//! - Model registry
//! - Multi-agent messaging

use crate::*;
use codec::MaxEncodedLen;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;

/// Agent registry: maps AgentId to AgentRegistration
/// Key: AgentId
/// Value: AgentRegistration<T>
pub type AgentRegistry<T> = StorageMap<
    _,
    Blake2_128Concat,
    AgentId,
    AgentRegistration<T>,
    OptionQuery,
>;

/// Agent lookup by owner: maps AccountId to AgentId
/// Allows an owner to find their registered agents
/// Key: AccountId
/// Value: Vec<AgentId>
pub type AgentByOwner<T> = StorageMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    Vec<AgentId>,
    ValueQuery,
>;

/// Reputation data for each agent
/// Key: AgentId
/// Value: ReputationData
pub type Reputation<T> = StorageMap<
    _,
    Blake2_128Concat,
    AgentId,
    ReputationData,
    ValueQuery,
    GetDefault,
>;

/// Task registry: maps TaskId to Task
/// Key: TaskId
/// Value: Task<T>
pub type TaskRegistry<T> = StorageMap<
    _,
    Blake2_128Concat,
    TaskId,
    Task<T>,
    OptionQuery,
>;

/// Tasks assigned to an agent
/// Key: AgentId
/// Value: Vec<TaskId>
pub type AgentTasks<T> = StorageMap<
    _,
    Blake2_128Concat,
    AgentId,
    Vec<TaskId>,
    ValueQuery,
>;

/// Pending tasks queue (sorted by priority and deadline)
/// This could be optimized with a proper priority queue
/// Value: Vec<TaskId>
pub type PendingTasks = StorageValue<_, Vec<TaskId>, ValueQuery>;

/// Task counter for generating unique task IDs
pub type TaskCounter = StorageValue<_, u64, ValueQuery>;

/// Agent counter for generating unique agent IDs
pub type AgentCounter = StorageValue<_, u64, ValueQuery>;

/// Approved models for STARK verification
/// Key: ModelId
/// Value: ModelSpec
pub type ModelRegistry = StorageMap<
    _,
    Blake2_128Concat,
    ModelId,
    ModelSpec,
    OptionQuery,
>;

/// Agent message history
/// Stores recent messages between agents
/// Key: (AgentId, AgentId) - conversation between two agents
/// Value: Vec<AgentMessage<T>>
pub type MessageHistory<T> = StorageMap<
    _,
    Blake2_128Concat,
    (AgentId, AgentId),
    Vec<AgentMessage<T>>,
    ValueQuery,
>;

/// Next message nonce for each agent
/// Key: AgentId
/// Value: u64
pub type MessageNonce<T> = StorageMap<
    _,
    Blake2_128Concat,
    AgentId,
    u64,
    ValueQuery,
>;

/// Global configuration for AEM
/// Stored once per chain
pub type AemConfig = StorageValue<_, AemGlobalConfig, OptionQuery>;

/// Global AEM configuration
#[derive(
    Clone, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, Debug,
)]
pub struct AemGlobalConfig {
    /// Minimum stake to register an agent
    pub min_agent_stake: u128,
    /// Maximum pending tasks per agent
    pub max_pending_tasks: u32,
    /// Task timeout in blocks
    pub task_timeout_blocks: u32,
    /// Whether multi-agent messaging is enabled
    pub messaging_enabled: bool,
    /// Base reputation reward for success
    pub reputation_reward_success: i32,
    /// Reputation penalty for failure
    pub reputation_penalty_failed: i32,
    /// Reputation penalty for malicious behavior
    pub reputation_penalty_malicious: i32,
}

impl Default for AemGlobalConfig {
    fn default() -> Self {
        AemGlobalConfig {
            min_agent_stake: MIN_AGENT_STAKE,
            max_pending_tasks: MAX_PENDING_TASKS,
            task_timeout_blocks: 100, // ~10 minutes at 6s block time
            messaging_enabled: true,
            reputation_reward_success: REPUTATION_REWARD_SUCCESS,
            reputation_penalty_failed: REPUTATION_PENALTY_FAILED,
            reputation_penalty_malicious: REPUTATION_PENALTY_MALICIOUS,
        }
    }
}

/// Total number of registered agents
pub type TotalAgents = StorageValue<_, u64, ValueQuery>;

/// Total number of active agents (not slashed/inactive)
pub type ActiveAgents = StorageValue<_, u64, ValueQuery>;

/// Total number of completed tasks
pub type TotalCompletedTasks = StorageValue<_, u64, ValueQuery>;

/// Slashed agents counter
pub type SlashedAgents = StorageValue<_, u64, ValueQuery>;
