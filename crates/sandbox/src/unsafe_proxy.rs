use std::sync::Arc;

use starknet_core_contract_client::clients::StarknetSovereignContractClient;

use crate::{deploy_contract, Error, LocalWalletSignerMiddleware};

const STARKNET_SOVEREIGN: &str = include_str!("../artifacts/Starknet.json");
const UNSAFE_PROXY: &str = include_str!("../artifacts/UnsafeProxy.json");

/// Deploy Starknet sovereign contract and unsafe proxy for it.
/// Cached forge atrifacts are used for deployment, make sure they are up to date.
pub async fn deploy_starknet_sovereign_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<StarknetSovereignContractClient, Error> {
    // First we deploy the Starknet contract (no explicit contructor)
    let core_contract = deploy_contract(client.clone(), STARKNET_SOVEREIGN, ()).await?;

    // Then we deploy a simple delegate proxy to interact with Starknet contract (initialized with its
    // address)
    let proxy_contract =
        deploy_contract(client.clone(), UNSAFE_PROXY, core_contract.address()).await?;

    Ok(StarknetSovereignContractClient::new(
        proxy_contract.address(),
        client.clone(),
    ))
}

#[cfg(test)]
mod tests {
    use super::deploy_starknet_sovereign_behind_unsafe_proxy;
    use crate::EthereumSandbox;
    use starknet_core_contract_client::{
        interfaces::{
            CoreContractInitData, OperatorTrait, ProxyInitializeData, ProxySupportTrait,
            StarknetSovereignContractTrait,
        },
        StarknetCoreContractClient,
    };

    #[tokio::test]
    async fn test_starknet_sovereign_contract_initialized_in_anvil() {
        let sandbox = EthereumSandbox::spawn(None);
        let starknet = deploy_starknet_sovereign_behind_unsafe_proxy(sandbox.client())
            .await
            .expect("Failed to deploy");

        let data = ProxyInitializeData::<0> {
            sub_contract_addresses: [],
            eic_address: Default::default(),
            init_data: CoreContractInitData {
                program_hash: 1u64.into(), // zero program hash would be deemed invalid
                ..Default::default()
            },
        };

        // Initialize state & governance
        starknet
            .initialize_with(data)
            .await
            .expect("Failed to initialize");

        // Second attempt to register as operator should be skipped
        starknet
            .register_operator(starknet.client().address())
            .await
            .expect("Should not fail");

        // Check that contract is initialized
        let program_hash = starknet
            .program_hash()
            .await
            .expect("Failed to query program hash");
        assert_eq!(program_hash, 1u64.into());
    }
}
