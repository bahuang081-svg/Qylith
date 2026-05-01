//! Qylith Chain Specification
//!
//! This module defines the chain specification for the Qylith network,
//! including genesis configuration and initial validators.

use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use sc_chain_spec::{ChainExtension, ChainSpec, ChainType};
use sc_service::Properties;
use serde::{Deserialize, Serialize};
use sp_core::{crypto::UncheckedFrom, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

use qylith_runtime::{
    AccountId, AuraId, Balance, BalancesConfig, GenesisAccount, GenesisBuild,
    ParachainInfoConfig, SessionConfig, SessionKeys, SudoConfig, SystemConfig, WASM_BINARY,
};

/// Specialized `ChainSpec` for Qylith
pub type ChainSpec = sc_service::GenericChainSpec<QylithGenesisExt>;

/// Qylith Chain Properties
pub fn qylith_properties() -> Properties {
    let mut props = Properties::new();
    props.insert("tokenSymbol".into(), "QYL".into());
    props.insert("tokenDecimals".into(), 12u32.into());
    props.insert("ss58Format".into(), 42u32.into());
    props.insert("chainType".into(), "Qylith".into());
    props
}

/// Qylith Development Chain Spec
pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available")?;

    Ok(ChainSpec::from_genesis(
        "Qylith Development",
        "qylith_dev",
        ChainType::Development,
        move || testnet_genesis(wasm_binary, vec![], vec![], vec![], vec![], 1000.into()),
        vec![],
        None,
        None,
        Some(qylith_properties()),
        ChainExtension::None,
        Extensions {
            relay_chain: "rococo-local".into(),
            para_id: 1000,
        },
    ))
}

/// Qylith Local Testnet Chain Spec
pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Local testnet wasm binary not available")?;

    Ok(ChainSpec::from_genesis(
        "Qylith Local Testnet",
        "qylith_local_testnet",
        ChainType::Local,
        move || testnet_genesis(wasm_binary, vec![], vec![], vec![], vec![], 1000.into()),
        vec![],
        None,
        None,
        Some(qylith_properties()),
        ChainExtension::None,
        Extensions {
            relay_chain: "rococo-local".into(),
            para_id: 1000,
        },
    ))
}

/// Qylith Staging Chain Spec
pub fn staging_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Staging wasm binary not available")?;

    Ok(ChainSpec::from_genesis(
        "Qylith Staging",
        "qylith_staging",
        ChainType::Live,
        move || {
            staging_genesis(
                wasm_binary,
                // Initial validators
                vec![
                    (
                        // Validator 1 - Alice
                        sr25519::Public::unchecked_from(hex!("9effc1668ca621c5b23f3c5cf3f4ea5f98e6777"),
                        AuraId::unchecked_from(hex!("9effc1668ca621c5b23f3c5cf3f4ea5f98e6777")),
                    ),
                    (
                        // Validator 2 - Bob
                        sr25519::Public::unchecked_from(hex!("75f03990a47e4c7c4a46c6d8d8e0a7c9d15c1e3f"),
                        AuraId::unchecked_from(hex!("75f03990a47e4c7c4a46c6d8d8e0a7c9d15c1e3f")),
                    ),
                ],
            )
        },
        vec![],
        None,
        None,
        Some(qylith_properties()),
        ChainExtension::None,
        Extensions {
            relay_chain: "rococo".into(),
            para_id: 1000,
        },
    ))
}

/// Qylith Production Chain Spec
pub fn production_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Production wasm binary not available")?;

    Ok(ChainSpec::from_genesis(
        "Qylith",
        "qylith",
        ChainType::Live,
        move || {
            production_genesis(
                wasm_binary,
                // Initial validators will be configured for mainnet launch
                vec![
                    (
                        sr25519::Public::unchecked_from(hex!("")),
                        AuraId::unchecked_from(hex!("")),
                    ),
                ],
            )
        },
        vec![],
        None,
        None,
        Some(qylith_properties()),
        ChainExtension::None,
        Extensions {
            relay_chain: "polkadot".into(),
            para_id: 1000,
        },
    ))
}

/// Extensions for the chain spec
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Extensions {
    pub relay_chain: String,
    pub para_id: u32,
}

impl sc_chain_spec::Extensions for Extensions {
    fn relay_chain(&self) -> Option<&str> {
        Some(&self.relay_chain)
    }

    fn para_id(&self) -> Option<u32> {
        Some(self.para_id)
    }
}

/// Qylith Genesis Ext
#[derive(Debug, Serialize, Deserialize)]
pub struct QylithGenesisExt {
    #[serde(flatten)]
    pub base: qylith_runtime::GenesisConfig,
    pub para_id: u32,
}

impl<Q: sc_service::ChainType + sp_runtime::traits::ChainTypeExtension<QylithGenesisExt>>
    sc_service::ChainSpecExtension<Q> for Extensions
{
    type Extension = Extensions;
    fn override_extension(
        &self,
        _chain_spec: &Q,
        _extensions: &mut std::collections::HashMap<String, serde_json::Value>,
    ) -> Result<(), String> {
        Ok(())
    }
}

/// Helper function to generate a session key from seed
pub fn get_from_seed<T Public: Public>(seed: &str) -> T {
    T::Public::from unchecked_from(
        sp_core::sr25519::Public::from_seed_string(seed).unwrap().to_raw_vec()
    )
}

/// Generate genesis configuration for testnet
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AccountId, AuraId)>,
    _root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _workers: Vec<GenesisWorker>,
    parachain_id: ParaId,
) -> QylithGenesisExt {
    use qylith_runtime::{
        AdminOriginConfig, AssetConfig, AuthorshipConfig, AylumConfig, BalancesConfig,
        CollatorSelectionConfig, CouncilConfig, DemocracyConfig, ElectionsConfig,
        GenesisBuild, GovernanceConfig, IdentityConfig, NposConfig, ParachainInfoConfig,
        PrecompilesValue, RegistryConfig, RuntimeGenesisConfig, SessionConfig, SessionKeys,
        SudoConfig, SystemConfig, TechnicalCommitteeConfig, TreasuryConfig, VestingConfig,
        WASM_BINARY,
    };

    let mut endowed_accounts: Vec<AccountId> = endowed_accounts;

    // Initialize endowed accounts if empty
    if endowed_accounts.is_empty() {
        endowed_accounts = vec![
            get_account_id_from_seed("Alice"),
            get_account_id_from_seed("Bob"),
            get_account_id_from_seed("Charlie"),
            get_account_id_from_seed("Dave"),
            get_account_id_from_seed("Eve"),
            get_account_id_from_seed("Ferdie"),
        ];
    }

    // Initial authorities
    if initial_authorities.is_empty() {
        initial_authorities = vec![
            (get_account_id_from_seed("Alice"), get_aura_id_from_seed("Alice")),
            (get_account_id_from_seed("Bob"), get_aura_id_from_seed("Bob")),
        ];
    }

    // Root key
    let root_key = if _root_key == Default::default() {
        get_account_id_from_seed("Alice")
    } else {
        _root_key
    };

    const ENDOWMENT: Balance = 10_000_000 * qylith_runtime::DOLLARS;
    const STASH: Balance = 100 * qylith_runtime::DOLLARS;

    QylithGenesisExt {
        base: RuntimeGenesisConfig {
            system: SystemConfig {
                code: wasm_binary.to_vec(),
                ..Default::default()
            },
            balances: BalancesConfig {
                balances: endowed_accounts
                    .iter()
                    .cloned()
                    .map(|k| (k, ENDOWMENT))
                    .chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
                    .collect(),
            },
            session: SessionConfig {
                keys: initial_authorities
                    .iter()
                    .map(|x| (x.0.clone(), x.0.clone(), SessionKeys { aura: x.1.clone() }))
                    .collect(),
            },
            collator_selection: CollatorSelectionConfig {
                invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
                candidacy_bond: qylith_runtime::MIN_BOND * 16,
                ..Default::default()
            },
            sudo: SudoConfig {
                key: Some(root_key),
            },
            parachain_info: ParachainInfoConfig {
                parachain_id,
                ..Default::default()
            },
            ..Default::default()
        },
        para_id: parachain_id.into(),
    }
}

/// Generate genesis configuration for staging
fn staging_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AccountId, AuraId)>,
) -> RuntimeGenesisConfig {
    RuntimeGenesisConfig {
        system: SystemConfig {
            code: wasm_binary.to_vec(),
            ..Default::default()
        },
        balances: BalancesConfig {
            balances: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), 1_000_000 * qylith_runtime::DOLLARS))
                .collect(),
        },
        session: SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.0.clone(), SessionKeys { aura: x.1.clone() }))
                .collect(),
        },
        ..Default::default()
    }
}

/// Generate genesis configuration for production
fn production_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AccountId, AuraId)>,
) -> RuntimeGenesisConfig {
    RuntimeGenesisConfig {
        system: SystemConfig {
            code: wasm_binary.to_vec(),
            ..Default::default()
        },
        balances: BalancesConfig {
            balances: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), 1_000_000 * qylith_runtime::DOLLARS))
                .collect(),
        },
        session: SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.0.clone(), SessionKeys { aura: x.1.clone() }))
                .collect(),
        },
        ..Default::default()
    }
}

/// Helper function to get account ID from seed
fn get_account_id_from_seed(seed: &str) -> AccountId {
    sr25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
        .into_account()
}

/// Helper function to get aura ID from seed
fn get_aura_id_from_seed(seed: &str) -> AuraId {
    sr25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
        .into()
}

/// Placeholder worker type for genesis
#[derive(Debug, Serialize, Deserialize)]
pub struct GenesisWorker {
    pub id: String,
    pub worker: AccountId,
    pub cgit_id: String,
}
