use std::sync::Arc;

use clients::eth_bridge::StarknetEthBridgeContractClient;
use starknet_proxy_client::deploy::{
    deploy_contract_behind_safe_proxy, deploy_contract_behind_unsafe_proxy, Error,
};
use utils::{LocalWalletSignerMiddleware, NO_CONSTRUCTOR_ARG};
pub mod clients;
pub mod interfaces;

const STARKNET_ETH_BRIDGE: &str = include_str!("./artifacts/StarknetLegacyBridge.json");

pub async fn deploy_starknet_eth_bridge_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<StarknetEthBridgeContractClient, Error> {
    // Deploy the Eth Bridge contract (no explicit constructor)
    let eth_bridge_contract = deploy_contract_behind_unsafe_proxy(
        client.clone(),
        STARKNET_ETH_BRIDGE,
        NO_CONSTRUCTOR_ARG,
    )
    .await?;

    Ok(StarknetEthBridgeContractClient::new(
        eth_bridge_contract.address(),
        client.clone(),
    ))
}

pub async fn deploy_starknet_eth_bridge_behind_safe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<StarknetEthBridgeContractClient, Error> {
    // Deploy the Eth Bridge contract (no explicit constructor)
    let eth_bridge_contract =
        deploy_contract_behind_safe_proxy(client.clone(), STARKNET_ETH_BRIDGE, NO_CONSTRUCTOR_ARG)
            .await?;

    Ok(StarknetEthBridgeContractClient::new(
        eth_bridge_contract.address(),
        client.clone(),
    ))
}
