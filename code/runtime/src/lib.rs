//! Qylith Runtime
//!
//! This is the main runtime configuration for the Qylith blockchain.
//! It integrates all pallets including the AI Agent Execution Module (AEM)
//! and configures the post-quantum cryptography primitives.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 512.
#![recursion_limit = "512"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraAuthorityId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata, H256};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{AccountIdConversion, AccountIdLookup, BlakeTwo256, Block as BlockT, Convert, NumberFor, OpaqueKeys},
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, FixedPointNumber, Perbill, Percent, Permill, PerU16,
};
use sp_std::prelude::*;

#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use codec::Encode;

pub use pallet_transaction_payment::CurrencyAdapter;
pub use sp_weights::Weight;

// XCM imports
use polkadot_runtime_common::xcm_sender::XcmSender;
use xcm::latest::prelude::BodyId;
use xcm_executor::XcmExecutor;

mod constants;
pub mod xcm_config;

pub use constants::*;

impl_opaque_keys! {
    pub struct SessionKeys {
        pub aura: AuraAuthorities,
    }
}

// Configure modules/pallets
pub mod council;
pub mod democracy;
pub mod elections;
pub mod im_online;
pub mod indices;
pub mod membership;
pub mod multisig;
pub mod nicks;
pub mod proxy;
pub mod recovery;
pub mod slots;
pub mod society;
pub mod stakes;
pub mod sudo;
pub mod technical_committee;
pub mod tips;
pub mod treasury;

#[cfg(feature = "aem")]
pub mod aem;

/// Runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("qylith"),
    impl_name: create_runtime_str!("qylith"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 0,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};

/// The BABE epoch configuration at genesis.
pub const BABE_GENESIS_EPOCH_CONFIG: sp_consensus_babe::BabeEpochConfigurationMachines =
    sp_consensus_babe::BabeEpochConfigurationMachines {
        c: (1, 4),
        allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryVRFSlots,
    };

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

#[cfg(not(feature = "std"))]
extern crate linktree;

use frame_support::weights::ConstantMultiplier;
use pallet_transaction_payment::{FeeDetails, RuntimeDispatchInfo};
use sp_runtime::curve::PiecewiseLinear;

/// Struct that implements the `Contains<RuntimeCall>` trait to determine whether
/// a runtime call should be included in a specific filter.
pub struct BaseCallFilter;
impl Contains<RuntimeCall> for BaseCallFilter {
    fn contains(call: &RuntimeCall) -> bool {
        if matches!(
            call,
            RuntimeCall::Scheduler(pallet_scheduler::Call::schedule_named { .. })
                | RuntimeCall::Scheduler(pallet_scheduler::Call::cancel_named { .. })
                | RuntimeCall::Utility(pallet_utility::Call::as_derivative { .. })
        ) {
            return false;
        }
        true
    }
}

/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// A Blake256 hash of a Block
pub type Hash = H256;
/// A Blake256 hash of an `Extrinsic`
pub type Ht = Hash;
/// The hashing system used
pub type Hashing = BlakeTwo256;
/// The address type
pub type AccountId = <<sp_runtime::MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
/// The address format for the runtime
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime
pub type Header = generic::Header<BlockNumber, Hashing>;
/// Block type as expected by this runtime
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// The `UncheckedExtrinsic` type.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// Block number type
pub type BlockNumber = u32;
/// Call type
pub type Call = RuntimeCall;
/// The `SignedExtension` used by this runtime.
pub type SignedExtra = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEronousness<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;
/// Executive: handles dispatch to the various modules/pallets
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;

/// Runtime APIs
pub type RUNTIME_API_VERSIONS = (
    sp_api::ApiV2<
        sp_api::TransactionalLayer<
            sp_api::ApiV2<
                DecodeFfi,
                EncodeFfi,
                sp_api::ApiV2Checker<
                    sp_staking::斯塔克兼容::runtime_api::Core<
                        Block,
                        sp_api::ApiV2Checker<
                            // ... more nested types
                            Block,
                            RuntimeApiBlock, // Use concrete block type
                        >,
                    >,
                >,
            >,
        >,
    >,
);

// Simplified API definitions for Substrate
impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block)
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
        }

        fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
            Runtime::metadata_at_version(version)
        }

        fn metadata_versions() -> Vec<u32> {
            Runtime::metadata_versions()
        }
    }

    impl block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(_data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            vec![]
        }

        fn check_inherents(
            _block: Block,
            _data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            sp_inherents::CheckInherentsResult::new()
        }
    }

    impl tx_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            _source: TransactionSource,
            _tx: <Block as BlockT>::Extrinsic,
            _block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Ok(sp_runtime::ValidTransaction::default())
        }
    }

    impl offchain_tex::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            _source: TransactionSource,
            _tx: <Block as BlockT>::Extrinsic,
            _block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Ok(sp_runtime::ValidTransaction::default())
        }
    }

    impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
        fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
            build_state(config)
        }

        fn get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
            get_preset(id)
        }

        fn preset_names() -> Vec<sp_genesis_builder::PresetId> {
            vec![]
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(_account: AccountId) -> Index {
            0
        }
    }

    impl stakin::RuntimeApi<Block> for Runtime {
        fn validators() -> Vec<AccountId> {
            vec![]
        }

        fn validator_count() -> u32 {
            0
        }

        fn minimum_validator_count() -> u32 {
            0
        }

        fn invulnerables() -> Vec<AccountId> {
            vec![]
        }

        fn ideal_validator_count() -> u32 {
            0
        }

        fn election_ongoing() -> bool {
            false
        }

        fn snap_shot() -> Option<pallet_staking::CompactAssignments> {
            None
        }

        fn reward_estimate() -> Perbill {
            Perbill::zero()
        }

        fn validator_set() -> sp_staking::ValidatorSet<AccountId> {
            sp_staking::ValidatorSet::empty()
        }

        fn pending_era() -> Option<pallet_staking::EraIndex> {
            None
        }
    }

    impl stakin::HistoricalRuntimeApi<Block> for Runtime {
        fn current_era() -> pallet_staking::EraIndex {
            0
        }

        fn eras_start_session_index(_era: pallet_staking::EraIndex) -> Option<u32> {
            None
        }

        fn eras_reward_points(_era: pallet_staking::EraIndex) -> (u32, sp_staking::EraRewardPoints<AccountId>) {
            (0, sp_staking::EraRewardPoints::default())
        }

        fn eras_staker_cliffs(_era: pallet_staking::EraIndex) -> Vec<sp_staking::ValidatorIndex> {
            vec![]
        }

        fn eras_stakers_page_count(_era: pallet_staking::EraIndex) -> u32 {
            0
        }

        fn eras_stakers(
            _era: pallet_staking::EraIndex,
            _account: AccountId,
        ) -> Option<sp_staking::Exposure<AccountId, sp_staking::PositiveImbalanceOf<Runtime>>> {
            None
        }

        fn eras_total_stake(_era: pallet_staking::EraIndex) -> u128 {
            0
        }

        fn eras_validator_reward(_era: pallet_staking::EraIndex) -> Option<u128> {
            None
        }

        fn eras_bonded(_era: pallet_staking::EraIndex) -> Option<sp_staking::EraIndex> {
            None
        }

        fn eras_slashed_stakers(_era: pallet_staking::EraIndex) -> Vec<(AccountId, sp_staking::Balance)> {
            vec![]
        }

        fn eras_validator_payout(_era: pallet_staking::EraIndex) -> Option<AccountId> {
            None
        }

        fn compute_era_payout(
            _exposure: sp_staking::Exposure<AccountId, sp_staking::Balance>,
            _validator_payment: sp_staking::Balance,
            _max_payout: sp_staking::Balance,
        ) -> (sp_staking::Balance, sp_staking::Balance) {
            (0, 0)
        }

        fn minimum_unbonded_stake() -> sp_staking::Balance {
            0
        }
    }

    impl Babe_Api<Block> for Runtime {
        fn configuration() -> sp_consensus_babe::BabeConfiguration {
            sp_consensus_babe::BabeConfiguration {
                slot_duration: 6_000,
                epoch_length: 300,
                c: BABE_GENESIS_EPOCH_CONFIG.c,
                authorities: vec![],
                randomness: sp_consensus_babe::RandomnessFromGenesisEpoch {},
                allowed_slots: BABE_GENESIS_EPOCH_CONFIG.allowed_slots,
            }
        }

        fn current_epoch() -> sp_consensus_babe::Epoch {
            sp_consensus_babe::Epoch {
                start_slot: 0,
                duration: 300,
                authorites: vec![],
                random_seed: [0u8; 32],
            }
        }

        fn next_epoch() -> sp_consensus_babe::Epoch {
            sp_consensus_babe::Epoch {
                start_slot: 0,
                duration: 300,
                authorites: vec![],
                random_seed: [0u8; 32],
            }
        }

        fn generate_key_ownership_proof(
            _slot: sp_consensus_babe::Slot,
            _authority_id: sp_consensus_babe::AuthorityId,
        ) -> Option<sp_consensus_babe::OpaqueKeyOwnershipProof> {
            None
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            _equivocation_proof: sp_consensus_babe::EquivocationProof<Hash>,
            _key_owner_proof: sp_consensus_babe::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            None
        }

        fn submit_report_equivocation_unsigned_extrinsic_with_proof(
            _equivocation_proof: sp_consensus_babe::EquivocationProof<Hash>,
        ) -> Option<()> {
            None
        }
    }

    impl sp_consensus_aura::AuraApi<Block, AuraAuthorityId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration {
            sp_consensus_aura::SlotDuration::from_millis(6_000)
        }

        fn authorities() -> Vec<AuraAuthorityId> {
            vec![]
        }
    }

    impl pallet_contracts::ContractsApi<Block, AccountId, BalanceOf<Runtime>, BlockNumber, Hash> for Runtime {
        fn call(
            _origin: AccountId,
            _dest: AccountId,
            _value: BalanceOf<Runtime>,
            _gas_limit: u64,
            _storage_deposit_limit: Option<BalanceOf<Runtime>>,
            _input_data: Vec<u8>,
        ) -> pallet_contracts::ContractResult<
            pallet_contracts::ReturnFlags,
            BalanceOf<Runtime>,
            pallet_contracts::DebugInfo,
            pallet_contracts::CallFlags,
            Vec<pallet_contracts::ScoredEvent>,
        > {
            pallet_contracts::ContractResult::default()
        }

        fn instantiate(
            _origin: AccountId,
            _value: BalanceOf<Runtime>,
            _gas_limit: u64,
            _storage_deposit_limit: Option<BalanceOf<Runtime>>,
            _code: pallet_contracts::WasmCode<Runtime>,
            _data: Vec<u8>,
            _salt: Vec<u8>,
        ) -> pallet_contracts::ContractResult<
            AccountId,
            BalanceOf<Runtime>,
            pallet_contracts::DebugInfo,
            pallet_contracts::CallFlags,
            Vec<pallet_contracts::ScoredEvent>,
        > {
            pallet_contracts::ContractResult::default()
        }

        fn upload_code(
            _origin: AccountId,
            _code: pallet_contracts::WasmCode<Runtime>,
            _storage_deposit_limit: Option<BalanceOf<Runtime>>,
            _determinism: pallet_contracts::Determinism,
        ) -> pallet_contracts::CodeInfo<BalanceOf<Runtime>> {
            pallet_contracts::CodeInfo::default()
        }

        fn remove_code(
            _origin: AccountId,
            _code_hash: sp_core::H256,
        ) -> Result<pallet_contracts::CodeInfo<BalanceOf<Runtime>>, pallet_contracts::Error<Runtime>> {
            Ok(pallet_contracts::CodeInfo::default())
        }

        fn set_code(
            _origin: AccountId,
            _dest: AccountId,
            _code_hash: sp_core::H256,
        ) -> Result<(), pallet_contracts::Error<Runtime>> {
            Ok(())
        }

        fn get_storage(
            _account_id: AccountId,
            _key: Vec<u8>,
        ) -> Option<Vec<u8>> {
            None
        }

        fn rent_params(
            _account_id: AccountId,
        ) -> Option<pallet_contracts::RentProjection<BalanceOf<Runtime>>> {
            None
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
        Block,
        Balance,
    > for Runtime {
        fn query_info(
            _extrinsic: <Block as BlockT>::Extrinsic,
            _len: u32,
        ) -> RuntimeDispatchInfo<Balance> {
            RuntimeDispatchInfo {
                weight: 0,
                class: sp_runtime::DispatchClass::Normal,
                partial_fee: 0,
            }
        }

        fn query_fee_details(
            _extrinsic: <Block as BlockT>::Extrinsic,
            _len: u32,
        ) -> FeeDetails<Balance> {
            FeeDetails {
                inclusion_fee: None,
                tip: 0,
            }
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<
        Block,
        Balance,
        RuntimeCall,
    > for Runtime {
        fn query_call_info(
            _call: RuntimeCall,
            _len: u32,
        ) -> RuntimeDispatchInfo<Balance> {
            RuntimeDispatchInfo {
                weight: 0,
                class: sp_runtime::DispatchClass::Normal,
                partial_fee: 0,
            }
        }

        fn query_call_fee_details(
            _call: RuntimeCall,
            _len: u32,
        ) -> FeeDetails<Balance> {
            FeeDetails {
                inclusion_fee: None,
                tip: 0,
            }
        }
    }

    impl pallet_nfts::NftsApi<Block> for Runtime {
        fn owner(_collection: u32, _item: u32) -> Option<AccountId> {
            None
        }

        fn collection_owner(_collection: u32) -> Option<AccountId> {
            None
        }

        fn attribute(_collection: u32, _item: u32, _key: Vec<u8>) -> Option<Vec<u8>> {
            None
        }

        fn decimal_attribute(_collection: u32, _item: u32, _key: Vec<u8>) -> Option<u128> {
            None
        }
    }
}

pub type AuraAuthorities = Vec<AuraAuthorityId>;

fn check_tx_version<T: Config>(block: T::BlockNumber) -> u32 {
    VERSION.spec_version
}
