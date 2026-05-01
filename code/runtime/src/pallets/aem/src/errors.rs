//! AEM Error Types
//!
//! This module defines all error types used by the AEM pallet.

use frame_support::pallet_prelude::*;

/// Errors that can occur in the AEM pallet
#[derive(
    Clone, Copy, PartialEq, Eq, sp_runtime::RuntimeDebug, Encode, Decode, TypeInfo,
)]
pub enum Error<T> {
    /// Agent is not registered
    AgentNotFound,
    
    /// Agent is already registered
    AgentAlreadyExists,
    
    /// Task is not found
    TaskNotFound,
    
    /// Task already exists
    TaskAlreadyExists,
    
    /// Not the owner of this agent
    NotAgentOwner,
    
    /// Not the creator of this task
    NotTaskCreator,
    
    /// Agent is not assigned to this task
    NotAssignedAgent,
    
    /// Insufficient balance for stake
    InsufficientStake,
    
    /// Invalid permission level
    InvalidPermissionLevel,
    
    /// Agent is not active
    AgentNotActive,
    
    /// Agent is already active
    AgentAlreadyActive,
    
    /// Task is not in valid state for this operation
    InvalidTaskStatus,
    
    /// Task deadline has passed
    TaskDeadlinePassed,
    
    /// Maximum pending tasks exceeded
    MaxPendingTasksExceeded,
    
    /// Invalid signature
    InvalidSignature,
    
    /// Invalid FALCON public key
    InvalidFalconKey,
    
    /// Invalid ML-KEM public key
    InvalidMlKemKey,
    
    /// Invalid agent ID
    InvalidAgentId,
    
    /// Invalid task ID
    InvalidTaskId,
    
    /// Model not found
    ModelNotFound,
    
    /// Model not allowed for inference
    ModelNotAllowed,
    
    /// Reputation below minimum threshold
    ReputationTooLow,
    
    /// Cannot slash own agent
    CannotSlashOwnAgent,
    
    /// Value is zero or below minimum
    ValueTooLow,
    
    /// Metadata exceeds maximum size
    MetadataTooLarge,
    
    /// Invalid nonce (replay attack protection)
    InvalidNonce,
    
    /// Messaging is disabled
    MessagingDisabled,
    
    /// Cannot message self
    CannotMessageSelf,
    
    /// Fee exceeds maximum allowed
    FeeExceedsMaximum,
    
    /// Configuration error
    InvalidConfiguration,
}

impl<T> TryFrom<Error<T>> for &'static str {
    type Error = ();
    
    fn try_from(e: Error<T>) -> Result<&'static str, Self::Error> {
        match e {
            Error::AgentNotFound => Ok("Agent not found"),
            Error::AgentAlreadyExists => Ok("Agent already exists"),
            Error::TaskNotFound => Ok("Task not found"),
            Error::TaskAlreadyExists => Ok("Task already exists"),
            Error::NotAgentOwner => Ok("Not the agent owner"),
            Error::NotTaskCreator => Ok("Not the task creator"),
            Error::NotAssignedAgent => Ok("Not the assigned agent"),
            Error::InsufficientStake => Ok("Insufficient stake"),
            Error::InvalidPermissionLevel => Ok("Invalid permission level"),
            Error::AgentNotActive => Ok("Agent is not active"),
            Error::AgentAlreadyActive => Ok("Agent is already active"),
            Error::InvalidTaskStatus => Ok("Invalid task status"),
            Error::TaskDeadlinePassed => Ok("Task deadline has passed"),
            Error::MaxPendingTasksExceeded => Ok("Maximum pending tasks exceeded"),
            Error::InvalidSignature => Ok("Invalid signature"),
            Error::InvalidFalconKey => Ok("Invalid FALCON public key"),
            Error::InvalidMlKemKey => Ok("Invalid ML-KEM public key"),
            Error::InvalidAgentId => Ok("Invalid agent ID"),
            Error::InvalidTaskId => Ok("Invalid task ID"),
            Error::ModelNotFound => Ok("Model not found"),
            Error::ModelNotAllowed => Ok("Model not allowed"),
            Error::ReputationTooLow => Ok("Reputation too low"),
            Error::CannotSlashOwnAgent => Ok("Cannot slash own agent"),
            Error::ValueTooLow => Ok("Value too low"),
            Error::MetadataTooLarge => Ok("Metadata too large"),
            Error::InvalidNonce => Ok("Invalid nonce"),
            Error::MessagingDisabled => Ok("Messaging is disabled"),
            Error::CannotMessageSelf => Ok("Cannot message self"),
            Error::FeeExceedsMaximum => Ok("Fee exceeds maximum"),
            Error::InvalidConfiguration => Ok("Invalid configuration"),
        }
    }
}

impl<T: Config> From<Error<T>> for &'static str {
    fn from(e: Error<T>) -> &'static str {
        <&'static str>::try_from(e).unwrap_or("Unknown error")
    }
}
