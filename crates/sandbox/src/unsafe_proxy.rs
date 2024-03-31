use std::sync::Arc;

use crate::{Error, LocalWalletSignerMiddleware};
use alloy::{
    network::{Ethereum, EthereumSigner}, providers::{layers::SignerProvider, Provider, RootProvider}, sol, transports::http::Http
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

type StarknetSovereignContractInstance = StarknetSovereign::StarknetSovereignInstance<Ethereum, Http<reqwest::Client>, Arc<SignerProvider<Ethereum, Http<reqwest::Client>, RootProvider<Ethereum, Http<reqwest::Client>>, EthereumSigner>>>;

/// Deploy Starknet sovereign contract and unsafe proxy for it.
/// Cached forge atrifacts are used for deployment, make sure they are up to date.
pub async fn deploy_starknet_sovereign_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<StarknetSovereignContractInstance, Error> {
    let base_fee = client.as_ref().get_gas_price().await?;

    // First we deploy the Starknet core contract (no explicit contructor)
    let core_contract_builder = StarknetSovereign::deploy_builder(&client);
    let estimate = core_contract_builder.estimate_gas().await.unwrap();
    let core_contract_address = core_contract_builder.gas_price(base_fee).gas(estimate).nonce(0).deploy().await;
    // Once we know the Starknet core contract address (implementation address)
    // we can deploy and initialize our delegate proxy.
    // NOTE that real world proxies typically allow changing the implementation
    // address dynamically (this is basically how upgrades work). In our case,
    // for simplicity, the proxy is initialized only once during the deployment.
    let proxy_contract_builder = UnsafeProxy::deploy_builder(&client, core_contract_address.unwrap());
    let estimate = proxy_contract_builder.estimate_gas().await.unwrap();
    let proxy_contract_address = proxy_contract_builder.gas_price(base_fee).gas(estimate).nonce(1).deploy().await;

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
    use test_log::test;

    #[test(tokio::test)]
    async fn test_starknet_sovereign_contract_initialized_in_anvil() {
        let sandbox = EthereumSandbox::spawn(None).await;
        let sandbox_ref = sandbox.as_ref().clone();
        let starknet = deploy_starknet_sovereign_behind_unsafe_proxy(sandbox_ref.unwrap().client())
            .await
            .expect("Failed to deploy");
        let base_fee = sandbox_ref.unwrap().client().as_ref().get_gas_price().await.unwrap();
        let data = ProxyInitializeData::<0> {
            sub_contract_addresses: [],
            eic_address: Default::default(),
            init_data: CoreContractInitData {
                program_hash: U256::from(1_u64), // zero program hash would be deemed invalid
                ..Default::default()
            },
        };

        // Initialize state & governance
        let initialize_builder = starknet.initialize(data.into());
        let initialize_gas = initialize_builder.estimate_gas().await.unwrap();
        let _ = initialize_builder
            .nonce(2)
            .gas(initialize_gas)
            .gas_price(base_fee)
            .send()
            .await
            .expect("Failed to initialize");

        // Register as operator
        let register_operator_builder = starknet
            .registerOperator(starknet.provider().get_accounts().await.unwrap()[0]);
        let register_operator_gas = register_operator_builder.estimate_gas().await.unwrap();
        let _ = register_operator_builder
            .nonce(3)
            .gas(register_operator_gas)
            .gas_price(base_fee)
            .send()
            .await
            .expect("Failed to register as operator");

        // Check that contract is initialized
        let program_hash = starknet
            .programHash()
            .call()
            .await
            .unwrap()._0;
        assert_eq!(program_hash, U256::from(1_u64));

    }
}
