//! AEM Dispatchable Functions
//!
//! This module implements all callable extrinsics (dispatchable functions)
//! for the AEM pallet.

use super::*;
use frame_support::pallet_prelude::*;
use sp_core::sha2::Sha256;

/// Weight info for AEM extrinsics
pub trait WeightInfo {
    fn register_agent() -> Weight;
    fn deactivate_agent() -> Weight;
    fn reactivate_agent() -> Weight;
    fn update_permissions() -> Weight;
    fn deregister_agent() -> Weight;
    fn slash_agent() -> Weight;
    fn create_task() -> Weight;
    fn assign_task() -> Weight;
    fn start_task() -> Weight;
    fn complete_task() -> Weight;
    fn fail_task() -> Weight;
    fn cancel_task() -> Weight;
    fn update_reputation() -> Weight;
    fn approve_model() -> Weight;
    fn remove_model() -> Weight;
    fn send_message() -> Weight;
}

/// Implementation of weight info for production
pub struct SubstrateWeight<T>(sp_std::marker::PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn register_agent() -> Weight {
        // TODO: Calculate actual weight based on benchmarking
        Weight::from_parts(100_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads_writes(10, 5))
    }
    
    fn deactivate_agent() -> Weight {
        Weight::from_parts(50_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads_writes(5, 3))
    }
    
    fn reactivate_agent() -> Weight {
        Weight::from_parts(50_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads_writes(5, 3))
    }
    
    fn update_permissions() -> Weight {
        Weight::from_parts(30_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads_writes(3, 2))
    }
    
    fn deregister_agent() -> Weight {
        Weight::from_parts(100_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads_writes(8, 6))
    }
    
    fn slash_agent() -> Weight {
        Weight::from_parts(80_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads_writes(6, 4))
    }
    
    fn create_task() -> Weight {
        Weight::from_parts(50_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads_writes(5, 3))
    }
    
    fn assign_task() -> Weight {
        Weight::from_parts(30_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads_writes(4, 3))
    }
    
    fn start_task() -> Weight {
        Weight::from_parts(20_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads_writes(3, 2))
    }
    
    fn complete_task() -> Weight {
        Weight::from_parts(40_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads_writes(5, 4))
    }
    
    fn fail_task() -> Weight {
        Weight::from_parts(40_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads_writes(5, 4))
    }
    
    fn cancel_task() -> Weight {
        Weight::from_parts(30_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads_writes(4, 3))
    }
    
    fn update_reputation() -> Weight {
        Weight::from_parts(20_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads_writes(2, 2))
    }
    
    fn approve_model() -> Weight {
        Weight::from_parts(30_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads_writes(3, 2))
    }
    
    fn remove_model() -> Weight {
        Weight::from_parts(20_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads_writes(2, 2))
    }
    
    fn send_message() -> Weight {
        Weight::from_parts(40_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads_writes(4, 3))
    }
}

// Implement the pallet's dispatchable functions
impl<T: Config> Pallet<T> {
    // ========== Agent Management ==========
    
    /// Register a new AI agent
    ///
    /// This extrinsic allows an account to register a new AI agent with on-chain identity.
    /// The agent must provide:
    /// - FALCON-1024 public key for identity
    /// - ML-KEM-768 public key for encrypted communications
    /// - Metadata URI (IPFS CID)
    /// - Permission level
    ///
    /// The agent must also deposit a minimum stake.
    ///
    /// # Parameters
    /// - `signing_key`: FALCON-1024 public key for agent identity
    /// - `encryption_key`: ML-KEM-768 public key for communications
    /// - `metadata_uri`: IPFS CID to agent metadata
    /// - `permission_level`: Permission level for this agent
    /// - `allowed_tokens`: For TradeOnly permission, list of allowed token IDs
    ///
    /// # Errors
    /// - `AgentAlreadyExists`: If an agent with this signing key already exists
    /// - `InsufficientStake`: If the reserved balance is below minimum
    /// - `InvalidFalconKey`: If the FALCON public key is invalid
    /// - `InvalidMlKemKey`: If the ML-KEM public key is invalid
    /// - `MetadataTooLarge`: If metadata exceeds maximum size
    pub fn register_agent(
        origin: OriginFor<T>,
        signing_key: FalconPublicKey,
        encryption_key: MlKemPublicKey,
        metadata_uri: BoundedVec<u8, T::MaxMetadataSize>,
        permission_level: PermissionLevel,
        allowed_tokens: BoundedVec<TokenId, T::MaxPermissionLevel>,
    ) -> DispatchResult {
        let owner = ensure_signed(origin)?;
        
        // Validate FALCON public key
        ensure!(
            signing_key.0.len() == crate::qylith_primitives::falcon::Falcon1024::PUBLIC_KEY_SIZE,
            Error::<T>::InvalidFalconKey
        );
        
        // Validate ML-KEM public key
        ensure!(
            encryption_key.0.len() == crate::qylith_primitives::ml_kem::MlKem768::PUBLIC_KEY_SIZE,
            Error::<T>::InvalidMlKemKey
        );
        
        // Generate agent ID from signing key hash
        let signing_key_bytes = signing_key.0.clone();
        let agent_id = AgentId::from(Sha256::hash(&signing_key_bytes));
        
        // Check agent doesn't already exist
        ensure!(
            !AgentRegistry::<T>::contains_key(agent_id),
            Error::<T>::AgentAlreadyExists
        );
        
        // Check stake
        let min_stake = Self::get_min_agent_stake();
        ensure!(
            T::Currency::free_balance(&owner) >= min_stake,
            Error::<T>::InsufficientStake
        );
        
        // Reserve stake
        T::Currency::reserve(&owner, min_stake)
            .map_err(|_| Error::<T>::InsufficientStake)?;
        
        let current_block = frame_system::Pallet::<T>::block_number();
        
        // Create agent registration
        let registration = AgentRegistration {
            owner: owner.clone(),
            signing_key: signing_key.clone(),
            encryption_key,
            metadata_uri: metadata_uri.clone(),
            permission_level,
            allowed_tokens,
            registered_at: current_block,
            is_active: true,
        };
        
        // Store agent
        AgentRegistry::<T>::insert(agent_id, registration);
        
        // Update owner mapping
        let mut owner_agents = AgentByOwner::<T>::get(&owner);
        owner_agents.push(agent_id);
        AgentByOwner::<T>::insert(&owner, owner_agents);
        
        // Initialize reputation
        let reputation = ReputationData::default();
        Reputation::<T>::insert(agent_id, reputation);
        
        // Update counters
        AgentCounter::<T>::mutate(|c| *c += 1);
        TotalAgents::<T>::mutate(|t| *t += 1);
        ActiveAgents::<T>::mutate(|a| *a += 1);
        
        // Emit event
        Self::deposit_event(Event::AgentRegistered {
            agent_id,
            owner,
            signing_key,
            permission_level,
        });
        
        Ok(())
    }
    
    /// Deactivate an agent
    ///
    /// Temporarily disables an agent without releasing the stake.
    /// The agent can be reactivated later.
    ///
    /// # Parameters
    /// - `agent_id`: The agent to deactivate
    ///
    /// # Errors
    /// - `AgentNotFound`: If the agent doesn't exist
    /// - `NotAgentOwner`: If the caller is not the agent owner
    /// - `AgentNotActive`: If the agent is already inactive
    pub fn deactivate_agent(
        origin: OriginFor<T>,
        agent_id: AgentId,
    ) -> DispatchResult {
        let owner = ensure_signed(origin)?;
        
        let mut registration = AgentRegistry::<T>::get(agent_id)
            .ok_or(Error::<T>::AgentNotFound)?;
        
        ensure!(registration.owner == owner, Error::<T>::NotAgentOwner);
        ensure!(registration.is_active, Error::<T>::AgentNotActive);
        
        registration.is_active = false;
        AgentRegistry::<T>::insert(agent_id, registration);
        
        ActiveAgents::<T>::mutate(|a| *a = a.saturating_sub(1));
        
        Self::deposit_event(Event::AgentDeactivated {
            agent_id,
            owner,
        });
        
        Ok(())
    }
    
    /// Reactivate a previously deactivated agent
    ///
    /// # Parameters
    /// - `agent_id`: The agent to reactivate
    ///
    /// # Errors
    /// - `AgentNotFound`: If the agent doesn't exist
    /// - `NotAgentOwner`: If the caller is not the agent owner
    /// - `AgentAlreadyActive`: If the agent is already active
    pub fn reactivate_agent(
        origin: OriginFor<T>,
        agent_id: AgentId,
    ) -> DispatchResult {
        let owner = ensure_signed(origin)?;
        
        let mut registration = AgentRegistry::<T>::get(agent_id)
            .ok_or(Error::<T>::AgentNotFound)?;
        
        ensure!(registration.owner == owner, Error::<T>::NotAgentOwner);
        ensure!(!registration.is_active, Error::<T>::AgentAlreadyActive);
        
        registration.is_active = true;
        AgentRegistry::<T>::insert(agent_id, registration);
        
        ActiveAgents::<T>::mutate(|a| *a += 1);
        
        Self::deposit_event(Event::AgentReactivated {
            agent_id,
            owner,
        });
        
        Ok(())
    }
    
    /// Update agent permissions
    ///
    /// Only the agent owner can change permissions.
    ///
    /// # Parameters
    /// - `agent_id`: The agent to update
    /// - `new_permission_level`: New permission level
    /// - `new_allowed_tokens`: New allowed tokens (for TradeOnly)
    ///
    /// # Errors
    /// - `AgentNotFound`: If the agent doesn't exist
    /// - `NotAgentOwner`: If the caller is not the agent owner
    pub fn update_permissions(
        origin: OriginFor<T>,
        agent_id: AgentId,
        new_permission_level: PermissionLevel,
        new_allowed_tokens: BoundedVec<TokenId, T::MaxPermissionLevel>,
    ) -> DispatchResult {
        let owner = ensure_signed(origin)?;
        
        let mut registration = AgentRegistry::<T>::get(agent_id)
            .ok_or(Error::<T>::AgentNotFound)?;
        
        ensure!(registration.owner == owner, Error::<T>::NotAgentOwner);
        
        let old_level = registration.permission_level;
        registration.permission_level = new_permission_level;
        registration.allowed_tokens = new_allowed_tokens;
        
        AgentRegistry::<T>::insert(agent_id, registration);
        
        Self::deposit_event(Event::AgentPermissionsUpdated {
            agent_id,
            old_level,
            new_level: new_permission_level,
        });
        
        Ok(())
    }
    
    /// Deregister an agent and release the stake
    ///
    /// # Parameters
    /// - `agent_id`: The agent to deregister
    ///
    /// # Errors
    /// - `AgentNotFound`: If the agent doesn't exist
    /// - `NotAgentOwner`: If the caller is not the agent owner
    pub fn deregister_agent(
        origin: OriginFor<T>,
        agent_id: AgentId,
    ) -> DispatchResult {
        let owner = ensure_signed(origin)?;
        
        let registration = AgentRegistry::<T>::get(agent_id)
            .ok_or(Error::<T>::AgentNotFound)?;
        
        ensure!(registration.owner == owner, Error::<T>::NotAgentOwner);
        
        // Unreserve stake
        let min_stake = Self::get_min_agent_stake();
        T::Currency::unreserve(&owner, min_stake);
        
        // Remove agent
        AgentRegistry::<T>::remove(agent_id);
        Reputation::<T>::remove(agent_id);
        
        // Update owner mapping
        AgentByOwner::<T>::mutate(&owner, |agents| {
            agents.retain(|&id| id != agent_id);
        });
        
        // Update counters
        TotalAgents::<T>::mutate(|t| *t = t.saturating_sub(1));
        if registration.is_active {
            ActiveAgents::<T>::mutate(|a| *a = a.saturating_sub(1));
        }
        
        Self::deposit_event(Event::AgentDeregistered {
            agent_id,
            owner,
            released_amount: min_stake,
        });
        
        Ok(())
    }
    
    // ========== Task Management ==========
    
    /// Create a new task for an agent to execute
    ///
    /// # Parameters
    /// - `input_uri`: IPFS CID to task input data
    /// - `expected_output_uri`: IPFS CID to expected output (for verification)
    /// - `priority`: Task priority level
    /// - `max_fee`: Maximum fee the creator is willing to pay
    /// - `deadline`: Block number by which task should complete
    ///
    /// # Errors
    /// - `ValueTooLow`: If max_fee is zero
    /// - `TaskDeadlinePassed`: If deadline is in the past
    pub fn create_task(
        origin: OriginFor<T>,
        input_uri: BoundedVec<u8, T::MaxMetadataSize>,
        expected_output_uri: Option<BoundedVec<u8, T::MaxMetadataSize>>,
        priority: TaskPriority,
        max_fee: BalanceOf<T>,
        deadline: BlockNumberOf<T>,
    ) -> DispatchResult {
        let creator = ensure_signed(origin)?;
        
        ensure!(max_fee > Zero::zero(), Error::<T>::ValueTooLow);
        
        let current_block = frame_system::Pallet::<T>::block_number();
        ensure!(deadline > current_block, Error::<T>::TaskDeadlinePassed);
        
        // Generate task ID
        let task_counter = TaskCounter::<T>::get();
        let task_id_bytes = (creator.encode(), task_counter, current_block.encode());
        let task_id = TaskId::from(Sha256::hash(&task_id_bytes));
        
        // Create task
        let task = Task {
            id: task_id,
            agent_id: None,
            creator: creator.clone(),
            priority,
            status: TaskStatus::Pending,
            input_uri: input_uri.clone(),
            expected_output_uri,
            max_fee,
            deadline,
            created_at: current_block,
            completed_at: None,
        };
        
        // Store task
        TaskRegistry::<T>::insert(task_id, task);
        
        // Add to pending queue
        PendingTasks::<T>::mutate(|queue| {
            queue.push(task_id);
        });
        
        // Update counter
        TaskCounter::<T>::mutate(|c| *c += 1);
        
        Self::deposit_event(Event::TaskCreated {
            task_id,
            creator,
            priority,
            max_fee,
        });
        
        Ok(())
    }
    
    /// Assign a task to an agent
    ///
    /// # Parameters
    /// - `task_id`: The task to assign
    /// - `agent_id`: The agent to assign the task to
    ///
    /// # Errors
    /// - `TaskNotFound`: If the task doesn't exist
    /// - `AgentNotFound`: If the agent doesn't exist
    /// - `AgentNotActive`: If the agent is not active
    /// - `InvalidTaskStatus`: If the task is not pending
    /// - `MaxPendingTasksExceeded`: If agent has too many pending tasks
    pub fn assign_task(
        origin: OriginFor<T>,
        task_id: TaskId,
        agent_id: AgentId,
    ) -> DispatchResult {
        let _creator = ensure_signed(origin)?;
        
        let mut task = TaskRegistry::<T>::get(task_id)
            .ok_or(Error::<T>::TaskNotFound)?;
        
        ensure!(task.status == TaskStatus::Pending, Error::<T>::InvalidTaskStatus);
        
        let agent_reg = AgentRegistry::<T>::get(agent_id)
            .ok_or(Error::<T>::AgentNotFound)?;
        
        ensure!(agent_reg.is_active, Error::<T>::AgentNotActive);
        
        // Check agent's pending tasks
        let agent_tasks = AgentTasks::<T>::get(agent_id);
        ensure!(
            agent_tasks.len() as u32 < MAX_PENDING_TASKS,
            Error::<T>::MaxPendingTasksExceeded
        );
        
        // Update task
        task.agent_id = Some(agent_id);
        TaskRegistry::<T>::insert(task_id, task.clone());
        
        // Update agent's task list
        AgentTasks::<T>::mutate(agent_id, |tasks| {
            tasks.push(task_id);
        });
        
        // Remove from pending queue
        PendingTasks::<T>::mutate(|queue| {
            queue.retain(|&id| id != task_id);
        });
        
        Self::deposit_event(Event::TaskAssigned {
            task_id,
            agent_id,
        });
        
        Ok(())
    }
    
    /// Start executing a task (agent calls this)
    ///
    /// # Parameters
    /// - `task_id`: The task to start
    ///
    /// # Errors
    /// - `TaskNotFound`: If the task doesn't exist
    /// - `NotAssignedAgent`: If the caller is not the assigned agent
    /// - `InvalidTaskStatus`: If the task is not assigned
    pub fn start_task(
        origin: OriginFor<T>,
        task_id: TaskId,
    ) -> DispatchResult {
        let caller = ensure_signed(origin)?;
        
        // For simplicity, we verify the caller is the agent owner
        // In production, this would verify FALCON signature from the agent
        let mut task = TaskRegistry::<T>::get(task_id)
            .ok_or(Error::<T>::TaskNotFound)?;
        
        let agent_id = task.agent_id.ok_or(Error::<T>::NotAssignedAgent)?;
        
        let agent_reg = AgentRegistry::<T>::get(agent_id)
            .ok_or(Error::<T>::AgentNotFound)?;
        
        ensure!(agent_reg.owner == caller, Error::<T>::NotAssignedAgent);
        ensure!(task.status == TaskStatus::Pending, Error::<T>::InvalidTaskStatus);
        
        task.status = TaskStatus::InProgress;
        TaskRegistry::<T>::insert(task_id, task);
        
        Self::deposit_event(Event::TaskStarted {
            task_id,
            agent_id,
        });
        
        Ok(())
    }
    
    /// Mark a task as completed
    ///
    /// # Parameters
    /// - `task_id`: The completed task
    /// - `output_uri`: IPFS CID to the output data
    ///
    /// # Errors
    /// - `TaskNotFound`: If the task doesn't exist
    /// - `NotAssignedAgent`: If the caller is not the assigned agent
    /// - `InvalidTaskStatus`: If the task is not in progress
    pub fn complete_task(
        origin: OriginFor<T>,
        task_id: TaskId,
        output_uri: BoundedVec<u8, T::MaxMetadataSize>,
    ) -> DispatchResult {
        let caller = ensure_signed(origin)?;
        
        let mut task = TaskRegistry::<T>::get(task_id)
            .ok_or(Error::<T>::TaskNotFound)?;
        
        let agent_id = task.agent_id.ok_or(Error::<T>::NotAssignedAgent)?;
        
        let agent_reg = AgentRegistry::<T>::get(agent_id)
            .ok_or(Error::<T>::AgentNotFound)?;
        
        ensure!(agent_reg.owner == caller, Error::<T>::NotAssignedAgent);
        ensure!(task.status == TaskStatus::InProgress, Error::<T>::InvalidTaskStatus);
        
        let current_block = frame_system::Pallet::<T>::block_number();
        
        task.status = TaskStatus::Completed;
        task.completed_at = Some(current_block);
        TaskRegistry::<T>::insert(task_id, task);
        
        // Update reputation
        Reputation::<T>::mutate(agent_id, |rep| {
            rep.record_success(current_block);
        });
        
        // Remove from agent's task list
        AgentTasks::<T>::mutate(agent_id, |tasks| {
            tasks.retain(|&id| id != task_id);
        });
        
        TotalCompletedTasks::<T>::mutate(|t| *t += 1);
        
        Self::deposit_event(Event::TaskCompleted {
            task_id,
            agent_id,
            output_uri: output_uri.to_vec(),
        });
        
        Ok(())
    }
    
    /// Mark a task as failed
    ///
    /// # Parameters
    /// - `task_id`: The failed task
    /// - `reason`: Human-readable failure reason
    ///
    /// # Errors
    /// - `TaskNotFound`: If the task doesn't exist
    /// - `NotAssignedAgent`: If the caller is not the assigned agent
    /// - `InvalidTaskStatus`: If the task is not in progress
    pub fn fail_task(
        origin: OriginFor<T>,
        task_id: TaskId,
        reason: Vec<u8>,
    ) -> DispatchResult {
        let caller = ensure_signed(origin)?;
        
        let mut task = TaskRegistry::<T>::get(task_id)
            .ok_or(Error::<T>::TaskNotFound)?;
        
        let agent_id = task.agent_id.ok_or(Error::<T>::NotAssignedAgent)?;
        
        let agent_reg = AgentRegistry::<T>::get(agent_id)
            .ok_or(Error::<T>::AgentNotFound)?;
        
        ensure!(agent_reg.owner == caller, Error::<T>::NotAssignedAgent);
        ensure!(task.status == TaskStatus::InProgress, Error::<T>::InvalidTaskStatus);
        
        task.status = TaskStatus::Failed;
        TaskRegistry::<T>::insert(task_id, task);
        
        // Update reputation (penalty for failure)
        let current_block = frame_system::Pallet::<T>::block_number();
        Reputation::<T>::mutate(agent_id, |rep| {
            rep.record_failure(current_block);
        });
        
        // Remove from agent's task list
        AgentTasks::<T>::mutate(agent_id, |tasks| {
            tasks.retain(|&id| id != task_id);
        });
        
        Self::deposit_event(Event::TaskFailed {
            task_id,
            agent_id,
            reason,
        });
        
        Ok(())
    }
    
    /// Cancel a pending task
    ///
    /// # Parameters
    /// - `task_id`: The task to cancel
    /// - `reason`: Cancellation reason
    ///
    /// # Errors
    /// - `TaskNotFound`: If the task doesn't exist
    /// - `NotTaskCreator`: If the caller is not the task creator
    /// - `InvalidTaskStatus`: If the task is not pending
    pub fn cancel_task(
        origin: OriginFor<T>,
        task_id: TaskId,
        reason: Vec<u8>,
    ) -> DispatchResult {
        let caller = ensure_signed(origin)?;
        
        let task = TaskRegistry::<T>::get(task_id)
            .ok_or(Error::<T>::TaskNotFound)?;
        
        ensure!(task.creator == caller, Error::<T>::NotTaskCreator);
        ensure!(task.status == TaskStatus::Pending, Error::<T>::InvalidTaskStatus);
        
        // Remove from pending queue
        PendingTasks::<T>::mutate(|queue| {
            queue.retain(|&id| id != task_id);
        });
        
        // Mark as cancelled
        TaskRegistry::<T>::mutate(task_id, |t| {
            if let Some(task) = t {
                task.status = TaskStatus::Cancelled;
            }
        });
        
        Self::deposit_event(Event::TaskCancelled {
            task_id,
            reason,
        });
        
        Ok(())
    }
    
    /// Update agent reputation (for governance or automatic triggers)
    ///
    /// # Parameters
    /// - `agent_id`: The agent to update
    /// - `score_change`: The change in reputation score
    /// - `reason`: Reason for the update
    ///
    /// # Errors
    /// - `AgentNotFound`: If the agent doesn't exist
    pub fn update_reputation(
        origin: OriginFor<T>,
        agent_id: AgentId,
        score_change: i32,
        reason: ReputationUpdateReason,
    ) -> DispatchResult {
        let _admin = ensure_signed(origin)?;
        
        ensure!(
            AgentRegistry::<T>::contains_key(agent_id),
            Error::<T>::AgentNotFound
        );
        
        Reputation::<T>::mutate(agent_id, |rep| {
            let old_score = rep.score;
            rep.score = rep.score.saturating_add(score_change);
            rep.score = rep.score.clamp(MIN_REPUTATION, MAX_REPUTATION);
            
            // Emit event if reputation changed
            if old_score != rep.score {
                Self::deposit_event(Event::ReputationUpdated {
                    agent_id,
                    old_score,
                    new_score: rep.score,
                    reason,
                });
                
                // Check for milestones
                Self::check_reputation_milestone(agent_id, rep.score);
            }
        });
        
        Ok(())
    }
    
    /// Slash an agent for malicious behavior
    ///
    /// This is called by governance or automatic detection systems.
    ///
    /// # Parameters
    /// - `agent_id`: The agent to slash
    /// - `slash_percent`: Percentage of stake to slash (in basis points)
    /// - `reason`: Reason for slashing
    ///
    /// # Errors
    /// - `AgentNotFound`: If the agent doesn't exist
    /// - `CannotSlashOwnAgent`: Cannot slash your own agent
    pub fn slash_agent(
        origin: OriginFor<T>,
        agent_id: AgentId,
        slash_percent: u32,
        reason: Vec<u8>,
    ) -> DispatchResult {
        let slasher = ensure_signed(origin)?;
        
        let registration = AgentRegistry::<T>::get(agent_id)
            .ok_or(Error::<T>::AgentNotFound)?;
        
        ensure!(registration.owner != slasher, Error::<T>::CannotSlashOwnAgent);
        
        // Calculate slash amount
        let min_stake = Self::get_min_agent_stake();
        let slash_amount = min_stake * slash_percent as u128 / 10000u128;
        
        // Slash the stake
        T::Currency::slash_reserved(&registration.owner, slash_amount);
        
        // Update reputation
        let current_block = frame_system::Pallet::<T>::block_number();
        Reputation::<T>::mutate(agent_id, |rep| {
            rep.record_slash(current_block);
        });
        
        // Update counter
        SlashedAgents::<T>::mutate(|s| *s += 1);
        
        Self::deposit_event(Event::AgentSlashed {
            agent_id,
            owner: registration.owner,
            slash_amount,
            reason,
        });
        
        Ok(())
    }
    
    // ========== Model Management ==========
    
    /// Approve a model for STARK verification
    ///
    /// # Parameters
    /// - `model_id`: Unique model identifier
    /// - `model_hash`: SHA-256 hash of model weights
    /// - `model_type`: Model type string
    /// - `input_schema`: Input schema
    /// - `output_schema`: Output schema
    /// - `max_steps`: Maximum execution steps
    ///
    /// # Errors
    /// - `ModelNotAllowed`: If model is not allowed (always succeeds here)
    pub fn approve_model(
        origin: OriginFor<T>,
        model_id: ModelId,
        model_hash: [u8; 32],
        model_type: Vec<u8>,
        input_schema: Vec<u8>,
        output_schema: Vec<u8>,
        max_steps: u32,
    ) -> DispatchResult {
        let _admin = ensure_signed(origin)?;
        
        let spec = ModelSpec {
            id: model_id,
            model_hash,
            model_type: model_type.clone(),
            input_schema,
            output_schema,
            max_steps,
            is_allowed: true,
        };
        
        ModelRegistry::insert(model_id, spec);
        
        Self::deposit_event(Event::ModelApproved {
            model_id,
            model_type,
        });
        
        Ok(())
    }
    
    /// Remove a model from the registry
    ///
    /// # Parameters
    /// - `model_id`: The model to remove
    ///
    /// # Errors
    /// - `ModelNotFound`: If the model doesn't exist
    pub fn remove_model(
        origin: OriginFor<T>,
        model_id: ModelId,
    ) -> DispatchResult {
        let _admin = ensure_signed(origin)?;
        
        ensure!(
            ModelRegistry::contains_key(model_id),
            Error::<T>::ModelNotFound
        );
        
        ModelRegistry::remove(model_id);
        
        Self::deposit_event(Event::ModelRemoved {
            model_id,
        });
        
        Ok(())
    }
    
    // ========== Messaging ==========
    
    /// Send a message to another agent
    ///
    /// # Parameters
    /// - `to`: Recipient agent ID
    /// - `message_type`: Type of message
    /// - `payload`: Encrypted message payload
    ///
    /// # Errors
    /// - `AgentNotFound`: If sender or recipient doesn't exist
    /// - `MessagingDisabled`: If messaging is disabled
    /// - `CannotMessageSelf`: Cannot send message to self
    pub fn send_message(
        origin: OriginFor<T>,
        from_agent_id: AgentId,
        to: AgentId,
        message_type: MessageType,
        payload: BoundedVec<u8, T::MaxMetadataSize>,
    ) -> DispatchResult {
        let caller = ensure_signed(origin)?;
        
        // Validate sender exists and caller is owner
        let from_reg = AgentRegistry::<T>::get(from_agent_id)
            .ok_or(Error::<T>::AgentNotFound)?;
        ensure!(from_reg.owner == caller, Error::<T>::AgentNotFound);
        ensure!(from_reg.is_active, Error::<T>::AgentNotActive);
        
        // Validate recipient exists
        ensure!(
            AgentRegistry::<T>::contains_key(to),
            Error::<T>::AgentNotFound
        );
        
        ensure!(from_agent_id != to, Error::<T>::CannotMessageSelf);
        
        // Check messaging is enabled
        let config = AemConfig::get().unwrap_or_default();
        ensure!(config.messaging_enabled, Error::<T>::MessagingDisabled);
        
        // Get and increment nonce
        let nonce = MessageNonce::<T>::get(from_agent_id);
        MessageNonce::<T>::insert(from_agent_id, nonce + 1);
        
        let current_block = frame_system::Pallet::<T>::block_number();
        
        // Create message
        let message = AgentMessage {
            from: from_agent_id,
            to,
            message_type,
            payload: payload.clone(),
            nonce,
            signature: Vec::new(), // TODO: Add FALCON signature
            timestamp: current_block,
        };
        
        // Store message
        let conversation = if from_agent_id < to {
            (from_agent_id, to)
        } else {
            (to, from_agent_id)
        };
        
        MessageHistory::<T>::mutate(conversation, |msgs| {
            msgs.push(message.clone());
        });
        
        Self::deposit_event(Event::MessageSent {
            from: from_agent_id,
            to,
            message_type,
            nonce,
        });
        
        Ok(())
    }
}

// Helper functions
impl<T: Config> Pallet<T> {
    /// Get the minimum agent stake from config
    pub fn get_min_agent_stake() -> BalanceOf<T> {
        AemConfig::get()
            .map(|c| c.min_agent_stake)
            .unwrap_or(MIN_AGENT_STAKE) as BalanceOf<T>
    }
    
    /// Check if agent has reached a reputation milestone
    fn check_reputation_milestone(agent_id: AgentId, score: i32) {
        let milestone = if score >= MAX_REPUTATION {
            Some(ReputationMilestoneType::MaximumReputation)
        } else if score >= 8000 {
            Some(ReputationMilestoneType::EliteAgent)
        } else if score >= 5000 {
            Some(ReputationMilestoneType::ReliableAgent)
        } else if score >= 3000 {
            Some(ReputationMilestoneType::TrustedAgent)
        } else if score >= 1000 {
            Some(ReputationMilestoneType::BasicAgent)
        } else {
            None
        };
        
        if let Some(m) = milestone {
            Self::deposit_event(Event::ReputationMilestone {
                agent_id,
                score,
                milestone: m,
            });
        }
    }
}
