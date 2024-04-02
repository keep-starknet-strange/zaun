use std::sync::Arc;

use clients::starkgate_manager::StarkgateManagerContractClient;
use starknet_proxy_client::deploy::{deploy_contract_behind_unsafe_proxy, Error};
use utils::LocalWalletSignerMiddleware;
pub mod clients;
pub mod interfaces;

const STARKGATE_MANAGER: &str = include_str!("./artifacts/StarkgateManager.json");

pub async fn deploy_starkgate_manager_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>
) -> Result<StarkgateManagerContractClient, Error> {
    // Deploy the Starkgate Manager contract (no explicit constructor)
    let manager_contract = deploy_contract_behind_unsafe_proxy(client.clone(), STARKGATE_MANAGER, ()).await?;

    Ok(StarkgateManagerContractClient::new(
        manager_contract.address(),
        client.clone(),
    ))
}