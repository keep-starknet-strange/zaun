use std::sync::Arc;

use clients::token_bridge::StarknetTokenBridgeContractClient;
use starknet_proxy_client::deploy::{deploy_contract_behind_proxy, Error, ProxyVersion};
use utils::{LocalWalletSignerMiddleware, NO_CONSTRUCTOR_ARG};

pub mod clients;
pub mod interfaces;

const STARKNET_TOKEN_BRIDGE: &str =
    include_str!("../../../../artifacts/starkgate-contracts/StarknetTokenBridge.json");

pub async fn deploy_starknet_token_bridge_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<StarknetTokenBridgeContractClient, Error> {
    // Deploy the Starknet Token Bridge contract (no explicit constructor)
    let token_bridge_contract = deploy_contract_behind_proxy(
        client.clone(),
        STARKNET_TOKEN_BRIDGE,
        NO_CONSTRUCTOR_ARG,
        ProxyVersion::UnsafeProxy,
    )
    .await?;

    Ok(StarknetTokenBridgeContractClient::new(
        token_bridge_contract.0.address(),
        client.clone(),
        token_bridge_contract.1.address(),
    ))
}

pub async fn deploy_starknet_token_bridge_behind_safe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<StarknetTokenBridgeContractClient, Error> {
    // Deploy the Starknet Token Bridge contract (no explicit constructor)
    let (token_bridge_contract, token_bridge_contract_implementation) =
        deploy_contract_behind_proxy(
            client.clone(),
            STARKNET_TOKEN_BRIDGE,
            NO_CONSTRUCTOR_ARG,
            ProxyVersion::SafeProxy5_0_0,
        )
        .await?;

    Ok(StarknetTokenBridgeContractClient::new(
        token_bridge_contract.address(),
        client.clone(),
        token_bridge_contract_implementation.address(),
    ))
}
