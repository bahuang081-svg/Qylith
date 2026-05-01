//! Qylith Runtime Constants
//!
//! This module defines all the constant parameters for the Qylith runtime.

/// Time in milliseconds for a block
pub const MILLISECS_PER_BLOCK: u64 = 6_000;

/// Time in slots for a block
pub const SLOT_DURATION: u64 = 6_000;

/// Blocks per year (assuming 6 second block time)
pub const BLOCKS_PER_YEAR: u32 = 5256000;

/// Time in epochs (sessions)
pub const EPOCH_DURATION_IN_BLOCKS: u32 = 300;

/// Time in sessions
pub const EPOCH_DURATION_IN_SLOTS: u64 = 300;

/// Number of sessions per era
pub const SESSIONS_PER_ERA: u32 = 6;

/// Number of blocks per era
pub const BLOCKS_PER_ERA: u32 = EPOCH_DURATION_IN_BLOCKS * SESSIONS_PER_ERA;

/// Number of eras per year
pub const ERAS_PER_YEAR: u32 = BLOCKS_PER_YEAR / BLOCKS_PER_ERA;

/// QYL token decimals
pub const DOLLARS: u32 = 1_000_000_000_000;
pub const CENTS: u32 = DOLLARS / 100; // 10_000_000_000
pub const MILLICENTS: u32 = CENTS / 10; // 1_000_000_000

/// Initial era for staking
pub const NORMAL_DISPATCH_RATIO: u32 = 75;

/// Minimum percentage of fill level for warnings
pub const AVERAGE_ON_INITIALIZE_RATIO: u32 = 25;

/// Maximum percentage of block weight for mortal extrinsics
pub const MAX_DRR_RATIO: u32 = 35;

/// Maximum height for scheduling
pub const MAX_BLOCK_HEIGHT: u32 = 10 * 60 * 60 / 6; // ~10 hours

/// Minimum period for a block
pub const MINIMUM_PERIOD: u64 = SLOT_DURATION / 2;

/// Maximum number of validators
pub const MAX_VALIDATORS: u32 = 1000;

/// Target number of validators
pub const TARGET_VALIDATORS: u32 = 500;

/// Minimum validator bond
pub const MIN_VALIDATOR_BOND: u128 = 100_000 * DOLLARS;

/// Minimum nominator bond
pub const MIN_NOMINATOR_BOND: u128 = 1_000 * DOLLARS;

/// Maximum nominations
pub const MAX_NOMINATIONS: u32 = 16;

/// Commission for validators
pub const VALIDATOR_COMMISSION: u32 = 20_000; // 20%

/// Slash defer duration (in eras)
pub const SLASH_DEFER_DURATION: u32 = 2;

/// Maximum lock duration for staking
pub const MAX_LOCK_DURATION: u32 = 36;

/// Maximum scheduled exits per era
pub const MAX_ELECTED_JOBS: u32 = 500;

/// Maximum scheduled exits per block
pub const MAX_SCHEDULED_EXITS: u32 = 10;

/// Number of eras to keep for slashing
pub const ERAS_TO_KEEP: u32 = 84;

/// Number of eras in a bonding period
pub const BONDING_DURATION_IN_ERAS: u32 = 28;

/// Number of eras to wait before being eligible to withdraw stake
pub const REMAINTING_ERAS: u32 = BONDING_DURATION_IN_ERAS - SLASH_DEFER_DURATION;

use sp_runtime::Perbill;

/// Reward rate per era (annual percentage)
pub const REWARD_RATE: Perbill = Perbill::from_percent(20);

/// Fallback values
pub const FALLBACK_MAX_VOTES: u32 = 16;

/// Preimage deposit base
pub const PREIMAGE_DEPOSIT_BASE: u128 = 10 * DOLLARS;

/// Preimage deposit per byte
pub const PREIMAGE_DEPOSIT_PER_BYTE: u128 = 10 * CENTS;

/// Vesting period
pub const VESTING_PERIOD: u32 = 48 * WEEKS;

/// Lock period
pub const LOCK_PERIOD: u32 = 48 * WEEKS;

/// Weeks to blocks conversion
pub const WEEKS: u32 = 201600;

/// Hours to blocks conversion
pub const HOURS: u32 = 3600 / 6;
