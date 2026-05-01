//! AEM Events
//!
//! This module defines all events emitted by the AEM pallet.
//! Events provide off-chain notification of on-chain state changes.

use crate::*;
use frame_support::pallet_prelude::*;

/// Events emitted by the AEM pallet
#[derive(
    Clone, Eq, PartialEq, Eq, Encode, Decode, TypeInfo, RuntimeDebug,
)]
pub enum Event<T: Config> {
    // ========== Agent Events ==========
    
    /// A new agent was registered
    AgentRegistered {
        agent_id: AgentId,
        owner: T::AccountId,
        signing_key: FalconPublicKey,
        permission_level: PermissionLevel,
    },
    
    /// An agent was deactivated
    AgentDeactivated {
        agent_id: AgentId,
        owner: T::AccountId,
    },
    
    /// An agent was reactivated
    AgentReactivated {
        agent_id: AgentId,
        owner: T::AccountId,
    },
    
    /// Agent permissions were updated
    AgentPermissionsUpdated {
        agent_id: AgentId,
        old_level: PermissionLevel,
        new_level: PermissionLevel,
    },
    
    /// Agent was slashed for malicious behavior
    AgentSlashed {
        agent_id: AgentId,
        owner: T::AccountId,
        slash_amount: BalanceOf<T>,
        reason: Vec<u8>,
    },
    
    /// Agent was deregistered and stake was released
    AgentDeregistered {
        agent_id: AgentId,
        owner: T::AccountId,
        released_amount: BalanceOf<T>,
    },
    
    // ========== Reputation Events ==========
    
    /// Agent reputation was updated
    ReputationUpdated {
        agent_id: AgentId,
        old_score: i32,
        new_score: i32,
        reason: ReputationUpdateReason,
    },
    
    /// Agent reputation reached a milestone
    ReputationMilestone {
        agent_id: AgentId,
        score: i32,
        milestone: ReputationMilestoneType,
    },
    
    // ========== Task Events ==========
    
    /// A new task was created
    TaskCreated {
        task_id: TaskId,
        creator: T::AccountId,
        priority: TaskPriority,
        max_fee: BalanceOf<T>,
    },
    
    /// A task was assigned to an agent
    TaskAssigned {
        task_id: TaskId,
        agent_id: AgentId,
    },
    
    /// A task was started by an agent
    TaskStarted {
        task_id: TaskId,
        agent_id: AgentId,
    },
    
    /// A task was completed successfully
    TaskCompleted {
        task_id: TaskId,
        agent_id: AgentId,
        output_uri: Vec<u8>,
    },
    
    /// A task failed
    TaskFailed {
        task_id: TaskId,
        agent_id: AgentId,
        reason: Vec<u8>,
    },
    
    /// A task was cancelled
    TaskCancelled {
        task_id: TaskId,
        reason: Vec<u8>,
    },
    
    /// A task timed out
    TaskTimeout {
        task_id: TaskId,
        agent_id: Option<AgentId>,
    },
    
    // ========== Model Events ==========
    
    /// A new model was approved for inference
    ModelApproved {
        model_id: ModelId,
        model_type: Vec<u8>,
    },
    
    /// A model was removed from the registry
    ModelRemoved {
        model_id: ModelId,
    },
    
    // ========== Messaging Events ==========
    
    /// Message sent between agents
    MessageSent {
        from: AgentId,
        to: AgentId,
        message_type: MessageType,
        nonce: u64,
    },
    
    // ========== Configuration Events ==========
    
    /// AEM global configuration was updated
    ConfigUpdated {
        updated_by: T::AccountId,
    },
}

/// Reason for reputation update
#[derive(
    Clone, Eq, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug,
)]
pub enum ReputationUpdateReason {
    /// Task completed successfully
    TaskSuccess,
    /// Task failed
    TaskFailed,
    /// Malicious behavior detected
    MaliciousBehavior,
    /// Manual adjustment by governance
    GovernanceAction,
    /// Initial reputation
    Initial,
}

impl Default for ReputationUpdateReason {
    fn default() -> Self {
        ReputationUpdateReason::Initial
    }
}

/// Reputation milestone types
#[derive(
    Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug,
)]
pub enum ReputationMilestoneType {
    /// Reached 1000 reputation (basic agent)
    BasicAgent,
    /// Reached 3000 reputation (trusted agent)
    TrustedAgent,
    /// Reached 5000 reputation (reliable agent)
    ReliableAgent,
    /// Reached 8000 reputation (elite agent)
    EliteAgent,
    /// Reached maximum reputation
    MaximumReputation,
}
