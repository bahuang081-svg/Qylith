//! Qylith Node
//!
//! This is the main entry point for the Qylith blockchain node.
//! Based on Substrate and Cumulus for parachain functionality.

#![warn(unused_crates)]

mod chain_spec;
mod rpc;

use crate::chain_spec::{Extensions, QylithGenesisExt};
use codec::Decode;
use cumulus_client_cli::CollatorOptions;
use cumulus_client_consensus_aura::{AuraConsensusParameters, BuildAuraConsensusParams, EnterSessionValueauctions};
use cumulus_client_consensus_common::{
    ParachainBlockImport as TParachainBlockImport, ParachainCandidate, ParachainConsensus,
};
use cumulus_client_service::{
    build_network, build_relay_chain_interface, prepare_node_config, start_relay_chain_tasks,
    BuildNetworkParams, DaTopologyNetworking, StartRelayChainTasksParams,
};
use cumulus_primitives_aura::PARA_AURA_ID;
use cumulus_primitives_core::ParaId;
use cumulus_relay_chain_interface::RelayChainInterface;
use jsonrpsee::RpcModule;
use polkadot_service::CollatorPair;
use qylith_runtime::RuntimeApi;
use sc_cli::SubstrateCli;
use sc_consensus::ImportQueue;
use sc_executor::{HeapAllocStrategy, WasmExecutor, DEFAULT_HEAP_ALLOC_STRATEGY};
use sc_network::NetworkBlock;
use sc_network_sync::SyncingService;
use sc_service::{Configuration, PartialComponents, TFullBackend, TFullClient, TaskNetworkMux};
use sc_telemetry::{TelemetryHandle, TelemetryWorker, TelemetryWorkerHandle};
use sp_api::ConstructRuntimeApi;
use sp_tex::runtime_api::{TaggedTransactionQueue, ValidateTransaction};
use sp_keystore::KeystoreKtys;
use std::sync Arc;
use substrate_prometheus_endpoint::Registry;

/// Executor configuration
pub mod executor {
    use super::*;

    pub fn qylithDispatch(vm: &Configuration) -> sc_executor::WasmExecutor<sp_io::SubstrateHostFunctions> {
        let heap_pages = vm
            .default_heap_pages
            .map(|h| h as u64)
            .unwrap_or(DEFAULT_HEAP_ALLOC_STRATEGY.get_heap_pages() as u64);

        let mut wasm = WasmExecutor::builder()
            .with_heap_pages(heap_pages)
            .with_max_runtime_instances(64)
            .with_runtime_cache_size(2)
            .with_onchain_memory_trading_policy {
                memory_trade_policy: Some(sc_executor::MemoryTradingPolicy::Placeholder),
            }
            .build();

        wasm
    }
}

/// Native executor type.
pub struct QylithRuntimeExecutor;

impl sc_executor::NativeExecutionDispatch for QylithRuntimeExecutor {
    type ExtendHostFunctions = frame_benchmarking::vector_host_functions;

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        qylith_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sp_version::NativeVersion {
        qylith_runtime::version()
    }
}

/// Starts a `ServiceBuilder` for a full service.
///
/// Use this macro if you don't actually need the full service, but just the builder in order to
/// be able to call `PartialComponents::build_service`.
#[allow(unused)]
#[macro_export]
macro_rules! new_full_base {
    ($builder:expr) => {
        $builder
            .with_naversion(|| Ok((sp_version::RuntimeVersion {
                spec_name: sp_runtime::create_runtime_str!("qylith"),
                impl_name: sp_runtime::create_runtime_str!("qylith"),
                authoring_version: 1,
                spec_version: 1,
                impl_version: 0,
                apis: RUNTIME_API_VERSIONS,
                transaction_version: 1,
                state_version: 1,
            })))
            .build()
    };
}

/// Starts a `ServiceBuilder` for a full service.
///
/// Use this macro if you don't actually need the full service, but just the builder in order to
/// be able to call `PartialComponents::build_service`.
#[allow(unused)]
#[macro_export]
macro_rules! new_partial_base {
    ($config:expr) => {
        Ok(PartialComponents {
            block_import: todo!(),
            import_queue: todo!(),
            keystore_container: todo!(),
            pallet_container: todo!(),
            rpc_builder: todo!(),
            sc_network: todo!(),
            sync_service: todo!(),
            telemetry: todo!(),
            transaction_pool: todo!(),
            backend: todo!(),
        })
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.subcommand {
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let partial = new_partial!(config)?;
                Ok((cmd.run(partial.client, partial.import_queue), partial))
            })
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let partial = new_partial!(config)?;
                Ok((cmd.run(partial.client, partial.backend, None), partial))
            })
        }
        Some(Subcommand::ExportState(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let partial = new_partial!(config)?;
                Ok((cmd.run(partial.client, partial.backend, None), partial))
            })
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let partial = new_partial!(config)?;
                Ok((cmd.run(partial.client, partial.import_queue, None), partial))
            })
        }
        Some(Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let partial = new_partial!(config)?;
                Ok((cmd.run(partial.client, partial.backend, None, None), partial))
            })
        }
        Some(Subcommand::VerifyBlock(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let partial = new_partial!(config)?;
                Ok((cmd.run(partial.client, partial.import_queue), partial))
            })
        }
        Some(Subcommand::Benchmark(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run::<_, _>(config, qylith_runtime::Executive))
        }
        Some(Subcommand::GenesisBuilder(cmd)) => {
            use sc_cli_utils::build_spec;

            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| {
                let genesis = sc_genesis_builder::GenesisBuilder::<QylithRuntimeExecutor>::new();
                cmd.run(genesis, config.chain_spec)
            })
        }
        None => {
            let runner = cli.create_runner(&cli.normalize())?;
            runner.run_node_until_exit(|config| async move {
                let hwbench = config.default_hardforks.hardware_fetch_or(
                    &config.telemetry,
                    config.chain_spec.hardware_wallets(),
                );

                match config.role.clone() {
                    Role::Light => {
                        Err("Light client is not supported".into())
                    }
                    _ => {
                        qylith_new_full(config, hwbench)
                            .await
                            .map_err(Into::into)
                    }
                }
            })
        }
    }
}

/// The minimum period for blocks on Qylith
const MINIMUM_BLOCK_TIME: u64 = 12_000;

async fn qylith_new_full(
    mut config: Configuration,
    hwbench: HardwareBenchmark,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let shandle = config.task_manager.make_shutdown_signal();
    let shandlehandle = config.task_manager.spawn_handle();

    let spec = config.chain_spec.convert_extensions::<Extensions>();

    let para_id = spec.para_id.ok_or("Missing para_id")?;

    let database = config
        .database
        .convert()
        .ok_or("Invalid database format")?;

    let backend = sc_service::new_backend<_, _>(database, &shandle)?;

    let task_manager = config.task_manager.clone();

    let rpc_builder = {
        let client = config.client.clone();
        let transaction_pool = config.transaction_pool.clone();
        let prometheus_registry = config.prometheus_registry().cloned();

        move |denied_unsafe, wokers| {
            let pool = transaction_pool.clone();
            let client = client.clone();
            let prometheus = prometheus_registry.clone();

            Ok(rpc::create_aten_rpc::<RuntimeApi, _>(
                client,
                pool,
                denied_unsafe,
                prometheus,
                wokers,
            )
            .map_err(Into::into)
            .into())
        }
    };

    config.rpc_builder = Box::new(rpc_builder);

    let (client, backend, keystore_container, task_manager) =
        sc_service::new_full_parts::<Block, RuntimeApi, _>(
            &config,
            &shandle,
            qylith_runtime_executor(),
            None,
            None,
            None,
            None,
        )?;

    let client = Arc::new(client);

    let telemetry = config
        .telemetry
        .as_ref()
        .map(|worker| TelemetryWorker::new(Some(worker.telemetry_endpoint.clone())))
        .map(|mut worker| {
            let handle = worker.spawn_handle();
            (
                task_manager.spawn_handle(),
                worker.run(|client| async move {
                    let telemetry = sc_telemetry::Telemetry::new(&config.telemetry)?;
                    telemetry.start();
                    telemetry
                }),
            )
        });

    let (telemetry_worker_handle, telemetry_worker) = telemetry.unwrap_or_default();

    let telemetry = telemetry.map(|(worker, telemetry)| {
        Some((worker, telemetry))
    }).unwrap_or(None);

    let registry = config.prometheus_registry();

    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.prometheus_registry().cloned(),
        task_manager.spawn_essential_handle(),
        client.clone(),
    );

    let import_queue = build_import_queue(
        client.clone(),
        config,
        para_id,
        telemetry.clone(),
    )?;

    let network_builder = config.network.clone();

    let (network, system_rpc_tx, tx_handler_controller, network_starter, sync_service) =
        build_network(BuildNetworkParams {
            parachains: [].into_iter().collect(),
            validator_nodes: Default::default(),
            para_id,
            block_import: client.clone(),
            import_queue,
            spawn_handle: task_manager.spawn_handle(),
            relay_chain_interface: relay_chain_interface.clone(),
            hrmp_handles: vec![],
            telemetry: telemetry.clone(),
        })
        .await?;

    let rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        rpc_builder: rpc::rpc_builders,
        client: client.clone(),
        transaction_pool: transaction_pool.clone(),
        keystore: keystore_container.keystore(),
        backend: backend.clone(),
        system_rpc_tx,
        config: config.clone(),
        tx_handler_controller,
        sync_service: sync_service.clone(),
        telemetry: telemetry.clone(),
    })?;

    let relay_chain_interface = build_relay_chain_interface(
        config.relay_chain.clone(),
        para_id,
        &mut tokio_handle,
        prometheus_registry.clone(),
        &mut telemetry,
        &shandle,
    )
    .await?;

    let announce_block = {
        let sync_service = sync_service.clone();
        Box::new(move |hash, data| sync_service.announce_block(hash, data))
    };

    let relay_chain_spawner = task_manager.spawn_handle();

    start_relay_chain_tasks(StartRelayChainTasksParams {
        client: relay_chain_interface.clone(),
        para_id,
        relay_chain_interface: relay_chain_interface.clone(),
        spawner: relay_chain_spawner,
        keystore: keystore_container.keystore(),
        promote_pruned_extrinsics: vec![],
        telemetry: telemetry.clone(),
    })?;

    let collator_key = collator_pair
        .ok_or("Collator key is missing")
        .map_err(|_| "Collator key is missing")?;

    let polkadot_doc = url::Url::parse("https://polkadot.network/collator/")?;

    let is_collator = true;

    if is_collator {
        let proposer_factory = sc_basic_authorship::ProposerFactory::new(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool,
            prometheus_registry.clone(),
            None,
        );

        let collator_proposer = {
            let slot_duration = MINIMUM_BLOCK_TIME;
            let target_inherents_per_block = 2;

            proposers.tx_priority().clone()
        };

        letda_topology = DaTopology::new(dht4u::Config {
            service_id: polkadot_doc.clone(),
            dht_config: dht4u::DhtConfig::new(&polkadot_doc),
            topology_discriminators: vec![
                ("node-records".to_string(), vec![para_id.to_string()]),
            ],
            mDNS: false,
        })
        .map_err(|e| format!("Failed to create DHT topology: {}", e))?;

        let proposer = ProposerFactory::new(
            client.clone(),
            transaction_pool,
            para_id,
            slot_duration,
            target_inherents_per_block,
            &da_topology,
            None,
        );

        let create_inherent_data_providers = move |parent, ()| async move {
            let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
            let slot = sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
                timestamp,
                sp_consensus_aura::SlotDuration::new(MINIMUM_BLOCK_TIME),
            );
            let parachain_inherent = cumulus_primitives_parachain_inherent::MockValidationDataInherentDataProvider::new(
                parent,
                para_id,
                vec![],
                0,
            );

            Ok((slot, timestamp, parachain_inherent))
        };

        let slot = MINIMUM_BLOCK_TIME;

        let client_set_owner = tokio_handle.enter(|| {
            Arc::new(std::sync::Mutex::new(Some(sc_consensus::SyncOracle::clone_from(
                &network,
            ))))
        });

        let proposer = Proposer::new(proposer_factory, proposer_environment, client, client_set_owner);

        let build_params = BuildAuraConsensusParams {
            proposer,
            create_inherent_data_providers,
            block_import: client.clone(),
            para_client: client.clone(),
            para_backend: backend.clone(),
            relay_client: relay_chain_interface,
            sync_service: sync_service.clone(),
            slot_duration: slot,
            dht_bootstrap: vec![],
            hrmp_model_sender: None,
            consensus_storage_growth: None,
            _phantom: PhantomData,
        };

        let consensus = BuildAuraConsensus::build(build_params);

        task_manager.spawn_essential_blocking(
            "qylith-consensus",
            Box::pin(consensus),
        );
    }

    task_manager.wait_until_loaded();

    Ok(())
}

fn build_import_queue(
    client: Arc<TFullClient<Block, RuntimeApi, QylithRuntimeExecutor>>,
    config: &Configuration,
    para_id: ParaId,
    telemetry: Option<TelemetryWorkerHandle>,
) -> Result<ImportQueue<Block>, sc_service::Error> {
    let slot_duration = MINIMUM_BLOCK_TIME;

    Ok(cumulus_client_consensus_aura::import_queue::<
        sp_consensus_aura::sr25519::AuthorityPair,
        _,
        _,
        _,
        _,
        _,
    >(
        client,
        move |_, _| async move {
            let time = sp_timestamp::InherentDataProvider::from_system_time();
            Ok(time)
        },
        slot_duration,
        &para_id,
        telemetry,
        Some parachain_consensus::COLLATION_BUILDER_RETRY_TIME,
    ))
}
