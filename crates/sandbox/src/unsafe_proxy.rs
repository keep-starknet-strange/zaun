use std::sync::Arc;
use ethers::abi::Tokenize;
use ethers::contract::ContractInstance;

use starknet_core_contract_client::clients::{StarknetSovereignContractClient, StarknetEthBridgeContractClient, StarknetTokenBridgeContractClient};

use crate::{deploy_contract, Error, LocalWalletSignerMiddleware};

const STARKNET_SOVEREIGN: &str = include_str!("../artifacts/Starknet.json");

const STARKNET_ETH_BRIDGE: &str = include_str!("../artifacts/StarknetEthBridge.json");

const STARKGATE_MANAGER: &str = include_str!("../artifacts/StarkgateManager.json");
const STARKGATE_REGISTRY: &str = include_str!("../artifacts/StarkgateRegistry.json");
const STARKNET_TOKEN_BRIDGE: &str = include_str!("../artifacts/StarknetTokenBridge.json");
const ERC20_TOKEN: &str = include_str!("../artifacts/Dai.json");

const UNSAFE_PROXY: &str = include_str!("../artifacts/UnsafeProxy.json");

/// Deploy Starknet sovereign contract and unsafe proxy for it.
/// Cached forge artifacts are used for deployment, make sure they are up to date.
pub async fn deploy_starknet_sovereign_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<StarknetSovereignContractClient, Error> {
    // First we deploy the Starknet core contract (no explicit contructor)
    let core_contract = deploy_contract_behind_unsafe_proxy(client.clone(), STARKNET_SOVEREIGN, ()).await?;

    Ok(StarknetSovereignContractClient::new(
        core_contract.address(),
        client.clone(),
    ))
}

pub async fn deploy_starknet_eth_bridge_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>
) -> Result<StarknetEthBridgeContractClient, Error> {
    // First we deploy the Eth Bridge contract (no explicit contructor)
    let eth_bridge_contract = deploy_contract_behind_unsafe_proxy(client.clone(), STARKNET_ETH_BRIDGE, ()).await?;

    Ok(StarknetEthBridgeContractClient::new(
        eth_bridge_contract.address(),
        client.clone(),
    ))
}

pub async fn deploy_starknet_token_bridge_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>
) -> Result<StarknetTokenBridgeContractClient, Error> {
    // Deploy the contracts required to bridge ERC20 token from L1
    let manager_contract = deploy_contract_behind_unsafe_proxy(client.clone(), STARKGATE_MANAGER, ()).await?;
    let registry_contract = deploy_contract_behind_unsafe_proxy(client.clone(), STARKGATE_REGISTRY, ()).await?;
    let token_bridge_contract = deploy_contract_behind_unsafe_proxy(client.clone(), STARKNET_TOKEN_BRIDGE, ()).await?;
    let token_contract = deploy_contract_behind_unsafe_proxy(client.clone(), ERC20_TOKEN, ()).await?;

    Ok(StarknetTokenBridgeContractClient::new(
        manager_contract.address(),
        registry_contract.address(),
        token_bridge_contract.address(),
        token_contract.address(),
        client.clone(),
    ))
}

pub async fn deploy_contract_behind_unsafe_proxy<T: Tokenize>(
    client: Arc<LocalWalletSignerMiddleware>,
    contract_path: &str,
    constructor_args: T,
) -> Result<ContractInstance<Arc<LocalWalletSignerMiddleware>, LocalWalletSignerMiddleware>, Error> {
    let contract = deploy_contract(client.clone(), contract_path, constructor_args).await?;

    // Once we know the main contract address (implementation address)
    // we can deploy and initialize our delegate proxy.
    // NOTE that real world proxies typically allow changing the implementation
    // address dynamically (this is basically how upgrades work). In our case,
    // for simplicity, the proxy is initialized only once during the deployment.
    let proxy_contract =
        deploy_contract(client.clone(), UNSAFE_PROXY, contract.address()).await?;

    return Ok(proxy_contract);
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
