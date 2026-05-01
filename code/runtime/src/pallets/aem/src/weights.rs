//! AEM Weights
//!
//! This module defines weight information for AEM extrinsics.
//! Weights are used to calculate transaction fees and prevent spam.

use frame_support::traits::Get;
use frame_support::weights::{Weight, constants::RocksDbWeight};

/// Weight functions for the AEM pallet
pub struct QylithWeight<T>(sp_std::marker::PhantomData<T>);

impl<T: frame_system::Config> QylithWeight<T> {
    /// Returns the weight of database operations
    pub fn db_weight() -> Weight {
        RocksDbWeight::get()
    }
}

impl<T: frame_system::Config> WeightInfo for QylithWeight<T> {
    fn register_agent() -> Weight {
        Weight::from_parts(100_000_000, 0)
            .saturating_add(Self::db_weight().reads_writes(10, 5))
    }
    
    fn deactivate_agent() -> Weight {
        Weight::from_parts(50_000_000, 0)
            .saturating_add(Self::db_weight().reads_writes(5, 3))
    }
    
    fn reactivate_agent() -> Weight {
        Weight::from_parts(50_000_000, 0)
            .saturating_add(Self::db_weight().reads_writes(5, 3))
    }
    
    fn update_permissions() -> Weight {
        Weight::from_parts(30_000_000, 0)
            .saturating_add(Self::db_weight().reads_writes(3, 2))
    }
    
    fn deregister_agent() -> Weight {
        Weight::from_parts(100_000_000, 0)
            .saturating_add(Self::db_weight().reads_writes(8, 6))
    }
    
    fn slash_agent() -> Weight {
        Weight::from_parts(80_000_000, 0)
            .saturating_add(Self::db_weight().reads_writes(6, 4))
    }
    
    fn create_task() -> Weight {
        Weight::from_parts(50_000_000, 0)
            .saturating_add(Self::db_weight().reads_writes(5, 3))
    }
    
    fn assign_task() -> Weight {
        Weight::from_parts(30_000_000, 0)
            .saturating_add(Self::db_weight().reads_writes(4, 3))
    }
    
    fn start_task() -> Weight {
        Weight::from_parts(20_000_000, 0)
            .saturating_add(Self::db_weight().reads_writes(3, 2))
    }
    
    fn complete_task() -> Weight {
        Weight::from_parts(40_000_000, 0)
            .saturating_add(Self::db_weight().reads_writes(5, 4))
    }
    
    fn fail_task() -> Weight {
        Weight::from_parts(40_000_000, 0)
            .saturating_add(Self::db_weight().reads_writes(5, 4))
    }
    
    fn cancel_task() -> Weight {
        Weight::from_parts(30_000_000, 0)
            .saturating_add(Self::db_weight().reads_writes(4, 3))
    }
    
    fn update_reputation() -> Weight {
        Weight::from_parts(20_000_000, 0)
            .saturating_add(Self::db_weight().reads_writes(2, 2))
    }
    
    fn approve_model() -> Weight {
        Weight::from_parts(30_000_000, 0)
            .saturating_add(Self::db_weight().reads_writes(3, 2))
    }
    
    fn remove_model() -> Weight {
        Weight::from_parts(20_000_000, 0)
            .saturating_add(Self::db_weight().reads_writes(2, 2))
    }
    
    fn send_message() -> Weight {
        Weight::from_parts(40_000_000, 0)
            .saturating_add(Self::db_weight().reads_writes(4, 3))
    }
}

// For tests and no_std environments
impl WeightInfo for () {
    fn register_agent() -> Weight {
        Weight::MAX
    }
    
    fn deactivate_agent() -> Weight {
        Weight::MAX
    }
    
    fn reactivate_agent() -> Weight {
        Weight::MAX
    }
    
    fn update_permissions() -> Weight {
        Weight::MAX
    }
    
    fn deregister_agent() -> Weight {
        Weight::MAX
    }
    
    fn slash_agent() -> Weight {
        Weight::MAX
    }
    
    fn create_task() -> Weight {
        Weight::MAX
    }
    
    fn assign_task() -> Weight {
        Weight::MAX
    }
    
    fn start_task() -> Weight {
        Weight::MAX
    }
    
    fn complete_task() -> Weight {
        Weight::MAX
    }
    
    fn fail_task() -> Weight {
        Weight::MAX
    }
    
    fn cancel_task() -> Weight {
        Weight::MAX
    }
    
    fn update_reputation() -> Weight {
        Weight::MAX
    }
    
    fn approve_model() -> Weight {
        Weight::MAX
    }
    
    fn remove_model() -> Weight {
        Weight::MAX
    }
    
    fn send_message() -> Weight {
        Weight::MAX
    }
}
