use std::sync::Arc;

use crate::clients::StarknetValidityContractClient;
use clients::StarknetSovereignContractClient;
use starknet_proxy_client::deploy::{deploy_contract_behind_proxy, Error, ProxyVersion};
use utils::{LocalWalletSignerMiddleware, NO_CONSTRUCTOR_ARG};

pub mod clients;
pub mod interfaces;

const STARKNET_SOVEREIGN: &str = include_str!("artifacts/StarknetSovereign.json");
const STARKNET: &str = include_str!("artifacts/Starknet.json");

/// Deploy Starknet Sovereign contract and unsafe proxy for it.
/// Cached forge artifacts are used for deployment, make sure they are up to date.
pub async fn deploy_starknet_sovereign_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<StarknetSovereignContractClient, Error> {
    // Deploy the Starknet Core contract (no explicit constructor)
    let core_contract = deploy_contract_behind_proxy(
        client.clone(),
        STARKNET_SOVEREIGN,
        NO_CONSTRUCTOR_ARG,
        ProxyVersion::UnsafeProxy,
    )
    .await?;

    Ok(StarknetSovereignContractClient::new(
        core_contract.0.address(),
        client.clone(),
        core_contract.1.address(),
    ))
}

/// Deploy Starknet Sovereign contract and safe proxy for it.
/// Implementation of starknet contract is added which is used in prod by starknet
pub async fn deploy_starknet_sovereign_behind_safe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<StarknetSovereignContractClient, Error> {
    // Deploy the Starknet Core contract (no explicit constructor)
    let (core_contract, core_contract_implementation) = deploy_contract_behind_proxy(
        client.clone(),
        STARKNET_SOVEREIGN,
        NO_CONSTRUCTOR_ARG,
        ProxyVersion::SafeProxy3_0_2,
    )
    .await?;

    Ok(StarknetSovereignContractClient::new(
        core_contract.address(),
        client.clone(),
        core_contract_implementation.address(),
    ))
}

/// Deploy Starknet Validity contract and unsafe proxy for it.
/// Cached forge artifacts are used for deployment, make sure they are up to date.
pub async fn deploy_starknet_validity_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<StarknetValidityContractClient, Error> {
    // Deploy the Starknet Core contract (no explicit constructor)
    let core_contract = deploy_contract_behind_proxy(
        client.clone(),
        STARKNET,
        NO_CONSTRUCTOR_ARG,
        ProxyVersion::UnsafeProxy,
    )
    .await?;

    Ok(StarknetValidityContractClient::new(
        core_contract.0.address(),
        client.clone(),
        core_contract.1.address(),
    ))
}

/// Deploy Starknet Validity contract and safe proxy for it.
/// Implementation of starknet contract is added which is used in prod by starknet
pub async fn deploy_starknet_validity_behind_safe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<StarknetValidityContractClient, Error> {
    // Deploy the Starknet Core contract (no explicit constructor)
    let (core_contract, core_contract_implementation) = deploy_contract_behind_proxy(
        client.clone(),
        STARKNET,
        NO_CONSTRUCTOR_ARG,
        ProxyVersion::SafeProxy3_0_2,
    )
    .await?;

    Ok(StarknetValidityContractClient::new(
        core_contract.address(),
        client.clone(),
        core_contract_implementation.address(),
    ))
}
