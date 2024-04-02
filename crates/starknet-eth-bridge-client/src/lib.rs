use std::sync::Arc;

use clients::eth_bridge::StarknetEthBridgeContractClient;
use starknet_proxy_client::deploy::{deploy_contract_behind_unsafe_proxy, Error};
use utils::LocalWalletSignerMiddleware;
pub mod interfaces;
pub mod clients;

const STARKNET_ETH_BRIDGE: &str = include_str!("./artifacts/StarknetLegacyBridge.json");

pub async fn deploy_starknet_eth_bridge_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>
) -> Result<StarknetEthBridgeContractClient, Error> {
    // Deploy the Eth Bridge contract (no explicit constructor)
    let eth_bridge_contract = deploy_contract_behind_unsafe_proxy(client.clone(), STARKNET_ETH_BRIDGE, ()).await?;

    Ok(StarknetEthBridgeContractClient::new(
        eth_bridge_contract.address(),
        client.clone(),
    ))
}