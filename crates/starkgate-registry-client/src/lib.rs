use std::sync::Arc;

use clients::starkgate_registry::StarkgateRegistryContractClient;
use starknet_proxy_client::deploy::{
    deploy_contract_behind_safe_proxy, deploy_contract_behind_unsafe_proxy, Error,
};
use utils::{LocalWalletSignerMiddleware, NO_CONSTRUCTOR_ARG};

pub mod clients;
pub mod interfaces;

const STARKGATE_REGISTRY: &str = include_str!("./artifacts/StarkgateRegistry.json");

pub async fn deploy_starkgate_registry_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<StarkgateRegistryContractClient, Error> {
    // Deploy the Starkgate Registry contract (no explicit constructor)
    let registry_contract =
        deploy_contract_behind_unsafe_proxy(client.clone(), STARKGATE_REGISTRY, NO_CONSTRUCTOR_ARG)
            .await?;

    Ok(StarkgateRegistryContractClient::new(
        registry_contract.address(),
        client.clone(),
    ))
}

pub async fn deploy_starkgate_registry_behind_safe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<StarkgateRegistryContractClient, Error> {
    // Deploy the Starkgate Registry contract (no explicit constructor)
    let registry_contract =
        deploy_contract_behind_safe_proxy(client.clone(), STARKGATE_REGISTRY, NO_CONSTRUCTOR_ARG)
            .await?;

    Ok(StarkgateRegistryContractClient::new(
        registry_contract.address(),
        client.clone(),
    ))
}
