//! Qylith RPC Configuration
//!
//! This module defines the RPC APIs exposed by the Qylith node.

use jsonrpsee::{RpcModule, RpcContext};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::{HeaderMetadata, HeaderBackend};
use sp_runtime::traits::Block as BlockT;

use qylith_runtime::{RuntimeApi, AccountId, Block, BlockNumber, Hash};

/// Full client type
pub type FullClient<C, BE> = TFullClient<Block, RuntimeApi, WasmExecutor<NativeElseWasmCall<Executor>>;

/// Instantiate all RPC extensions.
pub fn create_full<C, BE>(
    client: Arc<FullClient<C, BE>>,
    pool: Arc<TransactionPool>,
    _deny_unsafe: DenyUnsafe,
    _features: Ordering,
) -> Result<RpcModule<sc_service::TaskExecutor>, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block>
        + HeaderBackend<Block>
        + HeaderMetadata<Block, Error = BlockChainError>
        + 'static,
    BE: sc_client_api::Backend<Block> + 'static,
    <BE as sc_client_api::Backend<Block>>::State: sc_client_api::StateBackend<sp_runtime::traits::HashingFor<Block>>,
{
    use pallet_contracts_rpc::{Contracts, ContractsApiServer};
    use pallet_mmr_rpc::{Mmr, MmrApiServer};
    use pallet_staking_rpc::{Staking, StakingApiServer};
    use pallet_system_rpc::{System, SystemApiServer};
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};

    let mut module = RpcModule::new(());
    let transaction_payment = TransactionPayment::new(client.clone());
    module.merge(transaction_payment.into_rpc())?;
    module.merge(System::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
    module.merge(Contracts::new(client.clone()).into_rpc())?;
    module.merge(Mmr::new(client.clone()).into_rpc())?;
    module.merge(Staking::new(client.clone(), pool).into_rpc())?;

    // AEM RPC endpoints
    // Note: These would be implemented once the AEM RPC is defined
    // module.merge(Aem::new(client.clone()).into_rpc())?;

    Ok(module)
}

/// Helper function to create a service.
pub fn new_full<C, BE>(
    client: Arc<FullClient<C, BE>>,
    pool: Arc<TransactionPool>,
    prometheus_registry: Option<Registry>,
    telemetry: Option<TelemetryHandle>,
) -> Result<RpcModule<sc_service::TaskExecutor>, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block>
        + HeaderBackend<Block>
        + HeaderMetadata<Block, Error = BlockChainError>
        + 'static,
    BE: sc_client_api::Backend<Block> + 'static,
    <BE as sc_client_api::Backend<Block>>::State: sc_client_api::StateBackend<sp_runtime::traits::HashingFor<Block>>,
{
    let deny_unsafe = sc_rpc::DenyUnsafe::new();
    let features = Ordering::Unordered;

    create_full(client, pool, deny_unsafe, features)
}
