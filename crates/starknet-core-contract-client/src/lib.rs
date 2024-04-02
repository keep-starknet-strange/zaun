use std::sync::Arc;

use clients::StarknetSovereignContractClient;
use starknet_proxy_client::deploy::{deploy_contract_behind_unsafe_proxy, Error};
use utils::LocalWalletSignerMiddleware;

pub mod clients;
pub mod interfaces;

const STARKNET_SOVEREIGN: &str = include_str!("./artifacts/Starknet.json");

/// Deploy Starknet sovereign contract and unsafe proxy for it.
/// Cached forge artifacts are used for deployment, make sure they are up to date.
pub async fn deploy_starknet_sovereign_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<StarknetSovereignContractClient, Error> {
    // Deploy the Starknet Core contract (no explicit constructor)
    let core_contract = deploy_contract_behind_unsafe_proxy(client.clone(), STARKNET_SOVEREIGN, ()).await?;

    Ok(StarknetSovereignContractClient::new(
        core_contract.address(),
        client.clone(),
    ))
}