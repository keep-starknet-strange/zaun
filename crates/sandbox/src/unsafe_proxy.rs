use std::sync::Arc;

use crate::{Error, LocalWalletSignerMiddleware};
use alloy::{
    primitives::U256,
    network::{Ethereum, EthereumSigner},
    providers::{layers::SignerProvider, RootProvider},
    transports::BoxTransport,
    sol
};

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    StarknetSovereign,
    "artifacts/Starknet.json"
}

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    UnsafeProxy,
    "artifacts/UnsafeProxy.json"
}

type StarknetSovereignContractInstance = StarknetSovereign::StarknetSovereignInstance<Ethereum, BoxTransport, Arc<SignerProvider<Ethereum, BoxTransport, RootProvider<Ethereum, BoxTransport>, EthereumSigner>>>;

/// Deploy Starknet sovereign contract and unsafe proxy for it.
/// Cached forge atrifacts are used for deployment, make sure they are up to date.
pub async fn deploy_starknet_sovereign_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<StarknetSovereignContractInstance, Error> {
    // First we deploy the Starknet core contract (no explicit contructor)
    let core_contract_address = StarknetSovereign::deploy_builder(&client).nonce(0).gas_price(U256::from(1e9)).gas(U256::from(21000)).deploy().await;
    // Once we know the Starknet core contract address (implementation address)
    // we can deploy and initialize our delegate proxy.
    // NOTE that real world proxies typically allow changing the implementation
    // address dynamically (this is basically how upgrades work). In our case,
    // for simplicity, the proxy is initialized only once during the deployment.
    let proxy_contract_address = UnsafeProxy::deploy_builder(&client, core_contract_address.unwrap()).nonce(1).gas_price(U256::from(1e9)).gas(U256::from(21000)).deploy().await;
    
    Ok(StarknetSovereign::new(
        proxy_contract_address.unwrap(),
        client.clone(),
    ))
}

#[cfg(test)]
mod tests {
    use super::deploy_starknet_sovereign_behind_unsafe_proxy;
    use crate::EthereumSandbox;
    use alloy::{primitives::U256, providers::Provider};
    use starknet_core_contract_client::interfaces::{CoreContractInitData, ProxyInitializeData};

    #[tokio::test]
    async fn test_starknet_sovereign_contract_initialized_in_anvil() {
        let sandbox = EthereumSandbox::spawn(None).await;
        let starknet = deploy_starknet_sovereign_behind_unsafe_proxy(sandbox.unwrap().client())
            .await
            .expect("Failed to deploy");

        let data = ProxyInitializeData::<0> {
            sub_contract_addresses: [],
            eic_address: Default::default(),
            init_data: CoreContractInitData {
                program_hash: U256::from(1_u64), // zero program hash would be deemed invalid
                ..Default::default()
            },
        };

        // Initialize state & governance
        let _ = starknet
            .initialize(data.into())
            .send()
            .await
            .expect("Failed to initialize");

        // Register as operator
        let _ = starknet
            .registerOperator(starknet.provider().get_accounts().await.unwrap()[0])
            .send()
            .await
            .expect("Failed to register as operator");

        // Check that contract is initialized
        let program_hash = starknet
            .programHash()
            .call()
            .await
            .unwrap()._0;
        assert_eq!(program_hash, U256::from(0_u64));

    }
}
