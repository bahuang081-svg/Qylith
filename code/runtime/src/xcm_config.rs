//! XCM Configuration for Qylith
//!
//! This module defines the cross-chain message passing configuration.

use super::{
    AccountId, AllPalletsWithSystem, Balances, ParachainInfo, ParachainSystem, Runtime,
    TransactionConverter, WeightToFee, XcmpQueue,
};
use frame_support::traits::{ConstU32, Contains, Get, OnFinalize, OnInitialize, OnRuntimeUpgrade};
use pallet_xcm::XcmPassthrough;
use polkadot_runtime_common::xcm_sender::{ChildTombstoneEmitter, ExponentialError};
use xcm::latest::prelude::*;
use xcm_builder::{
    AccountId32Aliases, AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom,
    ChildParachainAsNative, ChildParachainConveyorViaMiddleware, ChildSystemParachainAsSuperuser,
    DescribeAllTerminal, DescribeFamily, HashedAs, NormalBoundedTransferKeepAlive, OriginAlias,
    ParentAsSuperuser, ParentT自动化, RelayChainAsNative, SiblingParachainAsNative,
    SiblingParachainConveyorViaWorkflow, SignedAccountId32AsNative, SignedToAccountId32,
    SovereignSignedViaLocation, TakeWeightCredit, UsingComponents,
};
use xcm_executor::traits::{ConvertLocation, WeightTrader};
use xcm_executor::XcmExecutor;

/// Qylith XCM configuration
pub struct QylithXcmConfig;

impl xcm_executor::Config for QylithXcmConfig {
    type RuntimeCall = RuntimeCall;
    type XcmSender = XcmRouter;
    type AssetTransactor = AssetTransactors;
    type OriginConverter = QylithOriginConverter;
    type IsReserve = QylithOrNativeReserve;
    type IsTeleporter = QylithNativeAsTeleporter;
    type UniversalLocation = UniversalLocation;
    type Barrier = QylithBarrier;
    type Weigher = FixedWeightBounds<BaseCallFilter, RuntimeCall, ConstU32<1000>>;
    type Trader = UsingComponents<WeightToFee, RelayChainLocation, AccountId, Balances, ()>;
    type ResponseHandler = ResponseHandler;
    type AssetTrap = AutoHeldXcmFees<Self::AssetTrap>;
    type AssetClaims = XcmAssets;
    type SubscriptionService = XcmSink;
}

pub type SovereignAccountOf = (
    ParentAsSuperuser<RuntimeOrigin>,
    RelayChainAsNative<sp_consensus_aura::LegacydapOrigin, RuntimeOrigin>,
    SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
    SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
    ChildParachainAsNative<ParachainInfo, RuntimeOrigin>,
    SignedToAccountId32<RelayAccountId, RuntimeOrigin, AccountId>,
);

pub type LocalOriginConverter = (
    SiblingParachainConveyorViaWorkflow<ParachainInfo, RuntimeOrigin, AccountId>,
    ParentAsSuperuser<RuntimeOrigin>,
);

/// Qylith Origin Converter
pub struct QylithOriginConverter;

impl ConvertLocation<RuntimeOrigin> for QylithOriginConverter {
    fn convert_location(location: &Location) -> Option<RuntimeOrigin> {
        LocalOriginConverter::convert_location(location)
    }
}

/// Barrier for XCM execution
pub type QylithBarrier = (
    TakeWeightCredit,
    AllowTopLevelPaidExecutionFrom<AllExcept<CouldBeESG>>,
    AllowUnpaidExecutionFrom<All<CouldBeESG>>,
);

/// Asset transactors
pub type AssetTransactors = XcmTransactor;

/// XCM Router
pub type XcmRouter = (
    cumulus_pallet_xcm::DoubleEncoded<Router>,
    XcmpQueue,
);

/// Response handler
pub type ResponseHandler = ();
