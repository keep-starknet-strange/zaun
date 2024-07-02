use std::sync::Arc;

use clients::starkgate_registry::StarkgateRegistryContractClient;
use starknet_proxy_client::deploy::{deploy_contract_behind_proxy, Error, ProxyVersion};
use utils::{LocalWalletSignerMiddleware, NO_CONSTRUCTOR_ARG};

pub mod clients;
pub mod interfaces;

const STARKGATE_REGISTRY: &str = include_str!("artifacts/StarkgateRegistry.json");

pub async fn deploy_starkgate_registry_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<StarkgateRegistryContractClient, Error> {
    // Deploy the Starkgate Registry contract (no explicit constructor)
    let registry_contract = deploy_contract_behind_proxy(
        client.clone(),
        STARKGATE_REGISTRY,
        NO_CONSTRUCTOR_ARG,
        ProxyVersion::UnsafeProxy,
    )
    .await?;

    Ok(StarkgateRegistryContractClient::new(
        registry_contract.0.address(),
        client.clone(),
        registry_contract.1.address(),
    ))
}

pub async fn deploy_starkgate_registry_behind_safe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<StarkgateRegistryContractClient, Error> {
    // Deploy the Starkgate Registry contract (no explicit constructor)
    let (registry_contract, registry_contract_implementation) = deploy_contract_behind_proxy(
        client.clone(),
        STARKGATE_REGISTRY,
        NO_CONSTRUCTOR_ARG,
        ProxyVersion::SafeProxyLatest,
    )
    .await?;

    Ok(StarkgateRegistryContractClient::new(
        registry_contract.address(),
        client.clone(),
        registry_contract_implementation.address(),
    ))
}
