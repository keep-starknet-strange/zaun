use std::sync::Arc;
use starknet_core_contract_client::clients::StarknetSovereignContractClient;
use crate::{Error, LocalWalletSignerMiddleware};
use alloy::{
    providers::Provider, sol,
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

/// Deploy Starknet sovereign contract and unsafe proxy for it.
/// Cached forge atrifacts are used for deployment, make sure they are up to date.
pub async fn deploy_starknet_sovereign_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<StarknetSovereignContractClient, Error> {
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

    Ok(StarknetSovereignContractClient::new(
        proxy_contract_address.unwrap(),
        client.clone(),
    ))
}

#[cfg(test)]
mod tests {
    use super::deploy_starknet_sovereign_behind_unsafe_proxy;
    use crate::EthereumSandbox;
    use alloy::{contract::Error, primitives::U256, providers::Provider, rpc::types::eth::TransactionReceipt, transports::{RpcError, TransportErrorKind}};
    use starknet_core_contract_client::{
        interfaces::{
            CoreContractInitData, OperatorTrait, ProxyInitializeData, ProxySupportTrait, StarknetSovereignContractTrait
        }, StarknetCoreContractClient
    };

    #[tokio::test]
    async fn test_starknet_sovereign_contract_initialized_in_anvil() {
        let sandbox = EthereumSandbox::spawn(None);
        let sandbox_ref = sandbox.as_ref().clone();
        let starknet = deploy_starknet_sovereign_behind_unsafe_proxy(sandbox_ref.unwrap().client())
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

        let _init: Result<TransactionReceipt, RpcError<TransportErrorKind>> = starknet.initialize(data.into()).await;
        
        let _register: Result<TransactionReceipt, RpcError<TransportErrorKind>> = starknet.register_operator(starknet.client().get_accounts().await.unwrap()[0]).await;
        
        let program_hash: Result<U256, Error> = starknet.program_hash().await;
        
        assert_eq!(program_hash.unwrap(), U256::from(1_u64));
    }
}
