//! AEM Benchmarking
//!
//! This module provides benchmarking support for the AEM pallet.
//! Benchmarks measure the execution time of extrinsics for accurate weight calculation.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::{Pallet as Aem, *};
use frame_benchmarking::v1::*;
use frame_support::traits::Get;
use frame_system::RawOrigin;

const SEED: u32 = 0;

fn create_agent<T: Config>(
    caller: T::AccountId,
    index: u32,
) -> Result<AgentId, sp_runtime::DispatchError> {
    // Generate dummy keys
    let signing_key = qylith_primitives::falcon::Falcon1024::keygen()
        .map_err(|_| sp_runtime::DispatchError::Other("Keygen failed"))?
        .0;
    let encryption_key = qylith_primitives::ml_kem::MlKem768::keygen()
        .map_err(|_| sp_runtime::DispatchError::Other("Keygen failed"))?
        .0;
    
    let metadata_uri: BoundedVec<u8, T::MaxMetadataSize> = vec![0u8; 100].try_into().unwrap();
    let allowed_tokens: BoundedVec<TokenId, T::MaxPermissionLevel> = vec![1].try_into().unwrap();
    
    // Reserve some balance for stake
    let min_stake = MIN_AGENT_STAKE as u128;
    T::Currency::make_free_balance_be(&caller, T::Currency::minimum_balance() + min_stake);
    T::Currency::reserve(&caller, min_stake)
        .map_err(|_| sp_runtime::DispatchError::Other("Reserve failed"))?;
    
    // Generate unique agent ID based on index
    let mut agent_id_bytes = [0u8; 32];
    agent_id_bytes[0..4].copy_from_slice(&index.to_le_bytes());
    let agent_id = AgentId::new(agent_id_bytes);
    
    // Insert directly to storage to avoid keygen issues
    let registration = AgentRegistration {
        owner: caller.clone(),
        signing_key,
        encryption_key,
        metadata_uri,
        permission_level: PermissionLevel::Unilateral,
        allowed_tokens,
        registered_at: frame_system::Pallet::<T>::block_number(),
        is_active: true,
    };
    
    AgentRegistry::<T>::insert(agent_id, registration);
    Reputation::<T>::insert(agent_id, ReputationData::default());
    AgentCounter::<T>::mutate(|c| *c += 1);
    TotalAgents::<T>::mutate(|t| *t += 1);
    ActiveAgents::<T>::mutate(|a| *a += 1);
    
    Ok(agent_id)
}

fn create_task<T: Config>(
    creator: T::AccountId,
    agent_id: Option<AgentId>,
    index: u32,
) -> Result<TaskId, sp_runtime::DispatchError> {
    let input_uri: BoundedVec<u8, T::MaxMetadataSize> = vec![0u8; 100].try_into().unwrap();
    let max_fee: BalanceOf<T> = 1000u32.into();
    
    // Generate unique task ID
    let mut task_id_bytes = [0u8; 32];
    task_id_bytes[4..8].copy_from_slice(&index.to_le_bytes());
    let task_id = TaskId::new(task_id_bytes);
    
    let current_block = frame_system::Pallet::<T>::block_number();
    
    let task = Task {
        id: task_id,
        agent_id,
        creator: creator.clone(),
        priority: TaskPriority::Normal,
        status: if agent_id.is_some() { TaskStatus::Pending } else { TaskStatus::Pending },
        input_uri,
        expected_output_uri: None,
        max_fee,
        deadline: current_block + 100u32.into(),
        created_at: current_block,
        completed_at: None,
    };
    
    TaskRegistry::<T>::insert(task_id, task);
    TaskCounter::<T>::mutate(|c| *c += 1);
    
    Ok(task_id)
}

benchmarks! {
    register_agent {
        let caller: T::AccountId = account("caller", 0, SEED);
        
        let signing_key = qylith_primitives::falcon::Falcon1024::keygen()
            .map_err(|_| "Keygen failed")?
            .0;
        let encryption_key = qylith_primitives::ml_kem::MlKem768::keygen()
            .map_err(|_| "Keygen failed")?
            .0;
        let metadata_uri: BoundedVec<u8, T::MaxMetadataSize> = vec![0u8; 100].try_into().unwrap();
        let allowed_tokens: BoundedVec<TokenId, T::MaxPermissionLevel> = vec![1].try_into().unwrap();
        
        let min_stake = MIN_AGENT_STAKE as u128;
        T::Currency::make_free_balance_be(&caller, T::Currency::minimum_balance() + min_stake);
        
        let agent_id_bytes = [0u8; 32];
        let agent_id = AgentId::from(sp_core::H256::from(agent_id_bytes));
    }: _(RawOrigin::Signed(caller.clone()), signing_key, encryption_key, metadata_uri, PermissionLevel::Unilateral, allowed_tokens)
    verify {
        assert!(AgentRegistry::<T>::contains_key(agent_id));
    }
    
    deactivate_agent {
        let caller: T::AccountId = account("caller", 0, SEED);
        let agent_id = create_agent::<T>(caller.clone(), 0)?;
    }: _(RawOrigin::Signed(caller), agent_id)
    verify {
        let reg = AgentRegistry::<T>::get(agent_id).unwrap();
        assert!(!reg.is_active);
    }
    
    reactivate_agent {
        let caller: T::AccountId = account("caller", 0, SEED);
        let agent_id = create_agent::<T>(caller.clone(), 0)?;
        
        // First deactivate
        AgentRegistry::<T>::mutate(agent_id, |reg| {
            reg.is_active = false;
        });
        ActiveAgents::<T>::mutate(|a| *a = a.saturating_sub(1));
    }: _(RawOrigin::Signed(caller), agent_id)
    verify {
        let reg = AgentRegistry::<T>::get(agent_id).unwrap();
        assert!(reg.is_active);
    }
    
    create_task {
        let caller: T::AccountId = account("caller", 0, SEED);
        let input_uri: BoundedVec<u8, T::MaxMetadataSize> = vec![0u8; 100].try_into().unwrap();
        let max_fee: BalanceOf<T> = 1000u32.into();
        let deadline = frame_system::Pallet::<T>::block_number() + 100u32.into();
    }: _(RawOrigin::Signed(caller), input_uri, None, TaskPriority::Normal, max_fee, deadline)
    
    complete_task {
        let caller: T::AccountId = account("caller", 0, SEED);
        let agent_id = create_agent::<T>(caller.clone(), 0)?;
        let task_id = create_task::<T>(caller.clone(), Some(agent_id), 0)?;
        
        // Set task to in progress
        TaskRegistry::<T>::mutate(task_id, |task| {
            task.status = TaskStatus::InProgress;
        });
        
        let output_uri: BoundedVec<u8, T::MaxMetadataSize> = vec![0u8; 100].try_into().unwrap();
    }: _(RawOrigin::Signed(caller), task_id, output_uri)
    verify {
        let task = TaskRegistry::<T>::get(task_id).unwrap();
        assert_eq!(task.status, TaskStatus::Completed);
    }
    
    fail_task {
        let caller: T::AccountId = account("caller", 0, SEED);
        let agent_id = create_agent::<T>(caller.clone(), 0)?;
        let task_id = create_task::<T>(caller.clone(), Some(agent_id), 0)?;
        
        TaskRegistry::<T>::mutate(task_id, |task| {
            task.status = TaskStatus::InProgress;
        });
        
        let reason = vec![0u8; 100];
    }: _(RawOrigin::Signed(caller), task_id, reason)
    verify {
        let task = TaskRegistry::<T>::get(task_id).unwrap();
        assert_eq!(task.status, TaskStatus::Failed);
    }
    
    update_reputation {
        let caller: T::AccountId = account("caller", 0, SEED);
        let agent_id = create_agent::<T>(caller.clone(), 0)?;
    }: _(RawOrigin::Signed(caller), agent_id, 100, ReputationUpdateReason::TaskSuccess)
    
    slash_agent {
        let slasher: T::AccountId = account("slasher", 0, SEED);
        let caller: T::AccountId = account("caller", 0, SEED);
        let agent_id = create_agent::<T>(caller, 0)?;
        
        T::Currency::make_free_balance_be(&slasher, T::Currency::minimum_balance() + 1u32.into());
    }: _(RawOrigin::Signed(slasher), agent_id, 1000, vec![0u8; 100])
    
    send_message {
        let caller: T::AccountId = account("caller", 0, SEED);
        let agent_id1 = create_agent::<T>(caller.clone(), 0)?;
        let agent_id2 = create_agent::<T>(caller.clone(), 1)?;
        
        let payload: BoundedVec<u8, T::MaxMetadataSize> = vec![0u8; 100].try_into().unwrap();
    }: _(RawOrigin::Signed(caller), agent_id1, agent_id2, MessageType::TaskRequest, payload)
    
    // Add more benchmarks as needed...
    
    impl_benchmark_test_suite!(
        Aem,
        crate::tests::new_test_ext(),
        crate::tests::Test
    );
}
