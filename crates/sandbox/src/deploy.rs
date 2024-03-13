use std::sync::Arc;
use starknet_core_contract_client::clients::{StarknetSovereignContractClient, StarknetEthBridgeContractClient, StarknetTokenBridgeContractClient, StarkgateManagerContractClient, StarkgateRegistryContractClient, DaiERC20ContractClient};

use crate::{deploy_contract, deploy_contract_behind_unsafe_proxy, Error, LocalWalletSignerMiddleware};

const STARKNET_SOVEREIGN: &str = include_str!("../artifacts/Starknet.json");

const STARKNET_ETH_BRIDGE: &str = include_str!("../artifacts/StarknetLegacyBridge.json");

const STARKGATE_MANAGER: &str = include_str!("../artifacts/StarkgateManager.json");
const STARKGATE_REGISTRY: &str = include_str!("../artifacts/StarkgateRegistry.json");
const STARKNET_TOKEN_BRIDGE: &str = include_str!("../artifacts/StarknetTokenBridge.json");
const DAI_ERC20_TOKEN: &str = include_str!("../artifacts/DaiERC20Token.json");


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

pub async fn deploy_starkgate_manager_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>
) -> Result<StarkgateManagerContractClient, Error> {
    // Deploy the Starkgate Manager contract (no explicit constructor)
    let manager_contract = deploy_contract_behind_unsafe_proxy(client.clone(), STARKGATE_MANAGER, ()).await?;

    Ok(StarkgateManagerContractClient::new(
        manager_contract.address(),
        client.clone(),
    ))
}

pub async fn deploy_starkgate_registry_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>
) -> Result<StarkgateRegistryContractClient, Error> {
    // Deploy the Starkgate Registry contract (no explicit constructor)
    let registry_contract = deploy_contract_behind_unsafe_proxy(client.clone(), STARKGATE_REGISTRY, ()).await?;

    Ok(StarkgateRegistryContractClient::new(
        registry_contract.address(),
        client.clone(),
    ))
}

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

pub async fn deploy_dai_erc20_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>
) -> Result<DaiERC20ContractClient, Error> {
    // Deploy the Dai ERC20 Token contract (no explicit constructor)
    let contract = deploy_contract(client.clone(), DAI_ERC20_TOKEN, ()).await?;

    Ok(DaiERC20ContractClient::new(
        contract.address(),
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
        StarknetContractClient,
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

        // Register as operator
        starknet
            .register_operator(starknet.client().address())
            .await
            .expect("Failed to register as operator");

        // Check that contract is initialized
        let program_hash = starknet
            .program_hash()
            .await
            .expect("Failed to query program hash");
        assert_eq!(program_hash, 1u64.into());
    }
}
