use std::sync::Arc;

use clients::starkgate_manager::StarkgateManagerContractClient;
use starknet_proxy_client::deploy::{deploy_contract_behind_proxy, Error, ProxyVersion};
use utils::{LocalWalletSignerMiddleware, NO_CONSTRUCTOR_ARG};
pub mod clients;
pub mod interfaces;

const STARKGATE_MANAGER: &str =
    include_str!("../../../../artifacts/starkgate-contracts/StarkgateManager.json");

pub async fn deploy_starkgate_manager_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<StarkgateManagerContractClient, Error> {
    // Deploy the Starkgate Manager contract (no explicit constructor)
    let manager_contract = deploy_contract_behind_proxy(
        client.clone(),
        STARKGATE_MANAGER,
        NO_CONSTRUCTOR_ARG,
        ProxyVersion::UnsafeProxy,
    )
    .await?;

    Ok(StarkgateManagerContractClient::new(
        manager_contract.0.address(),
        client.clone(),
        manager_contract.1.address(),
    ))
}

pub async fn deploy_starkgate_manager_behind_safe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<StarkgateManagerContractClient, Error> {
    // Deploy the Starkgate Manager contract (no explicit constructor)
    let (manager_contract, manager_contract_implementation) = deploy_contract_behind_proxy(
        client.clone(),
        STARKGATE_MANAGER,
        NO_CONSTRUCTOR_ARG,
        ProxyVersion::SafeProxy5_0_0,
    )
    .await?;

    Ok(StarkgateManagerContractClient::new(
        manager_contract.address(),
        client.clone(),
        manager_contract_implementation.address(),
    ))
}
