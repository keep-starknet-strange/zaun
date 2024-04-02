use std::sync::Arc;

use clients::token_bridge::StarknetTokenBridgeContractClient;
use starknet_proxy_client::deploy::{deploy_contract_behind_unsafe_proxy, Error};
use utils::LocalWalletSignerMiddleware;

pub mod clients;
pub mod interfaces;

const STARKNET_TOKEN_BRIDGE: &str = include_str!("./artifacts/StarknetTokenBridge.json");

pub async fn deploy_starknet_token_bridge_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>
) -> Result<StarknetTokenBridgeContractClient, Error> {
    // Deploy the Starknet Token Bridge contract (no explicit constructor)
    let token_bridge_contract = deploy_contract_behind_unsafe_proxy(client.clone(), STARKNET_TOKEN_BRIDGE, ()).await?;

    Ok(StarknetTokenBridgeContractClient::new(
        token_bridge_contract.address(),
        client.clone(),
    ))
}