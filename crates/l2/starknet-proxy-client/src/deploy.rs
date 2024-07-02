use ethereum_instance::deploy_contract;
use ethers::abi::{Token, Tokenize};
use ethers::contract::ContractError;
use ethers::prelude::ContractInstance;
use ethers::providers::ProviderError;
use ethers::types::U256;
use ethers::utils::hex::{self};
use std::sync::Arc;
use utils::LocalWalletSignerMiddleware;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("['bytecode']['object'] is not a string")]
    BytecodeObject,
    #[error(transparent)]
    Hex(#[from] hex::FromHexError),
    #[error("Failed to parse URL")]
    UrlParser,
    #[error(transparent)]
    EthersContract(#[from] ContractError<LocalWalletSignerMiddleware>),
    #[error(transparent)]
    EthersProvider(#[from] ProviderError),
    #[error("Invalid contract build artifacts: missing field `{0}`")]
    ContractBuildArtifacts(&'static str),
    #[error("Failed to deploy the contract : {0}")]
    DeployContract(#[from] ethereum_instance::Error),
}

const UNSAFE_PROXY: &str = include_str!("artifacts/UnsafeProxy.json");
const SAFE_PROXY: &str = include_str!("artifacts/Proxy.json");
const SAFE_PROXY_3_0_2: &str = include_str!("artifacts/Proxy_3_0_2.json");
const SAFE_PROXY_5_0_0: &str = include_str!("artifacts/Proxy_5_0_0.json");

/// Deploys new unsafe proxy contract:
///     - Implementation can be set only once at initialization
///     - Traditional (Safe) proxies can be upgraded multiple times
pub async fn deploy_contract_behind_unsafe_proxy<T: Tokenize>(
    client: Arc<LocalWalletSignerMiddleware>,
    contract_path: &str,
    constructor_args: T,
) -> Result<ContractInstance<Arc<LocalWalletSignerMiddleware>, LocalWalletSignerMiddleware>, Error>
{
    let contract = deploy_contract(client.clone(), contract_path, constructor_args).await?;

    let proxy_contract = deploy_contract(client.clone(), UNSAFE_PROXY, contract.address()).await?;

    Ok(proxy_contract)
}

/// Here we have three deploy behind safe proxy functions :
/// -------------------------------------------------------
///
/// - `deploy_contract_behind_safe_proxy` :
///     This function will deploy the contract behind the proxy that is used by starknet in deployment of the proxies.
/// - `deploy_contract_behind_safe_proxy_3_0_2` :
///     This function will deploy the contract behind the proxy that is being used by chains such as paradex and kakarot.
/// - `deploy_contract_behind_safe_proxy_5_0_0` :
///     This function will deploy the contract behind the latest version of the starknet proxy currently used in the token bridge : `0xF5b6Ee2CAEb6769659f6C091D209DfdCaF3F69Eb`
/// ---------------------------------------------------------

pub async fn deploy_contract_behind_safe_proxy<T: Tokenize>(
    client: Arc<LocalWalletSignerMiddleware>,
    contract_path: &str,
    constructor_args: T,
) -> Result<
    (
        ContractInstance<Arc<LocalWalletSignerMiddleware>, LocalWalletSignerMiddleware>,
        ContractInstance<Arc<LocalWalletSignerMiddleware>, LocalWalletSignerMiddleware>,
    ),
    Error,
> {
    let contract = deploy_contract(client.clone(), contract_path, constructor_args).await?;

    log::debug!("ℹ️  Contract deployed : {:?}", contract.address().clone());

    let proxy_contract =
        deploy_contract(client.clone(), SAFE_PROXY, Token::Uint(U256::from(0))).await?;

    log::debug!(
        "ℹ️  Proxy for contract [{:?}] deployed : {:?}",
        contract.address().clone(),
        proxy_contract.address()
    );

    Ok((proxy_contract, contract))
}

pub async fn deploy_contract_behind_safe_proxy_3_0_2<T: Tokenize>(
    client: Arc<LocalWalletSignerMiddleware>,
    contract_path: &str,
    constructor_args: T,
) -> Result<
    (
        ContractInstance<Arc<LocalWalletSignerMiddleware>, LocalWalletSignerMiddleware>,
        ContractInstance<Arc<LocalWalletSignerMiddleware>, LocalWalletSignerMiddleware>,
    ),
    Error,
> {
    let contract = deploy_contract(client.clone(), contract_path, constructor_args).await?;

    log::debug!("ℹ️  Contract deployed : {:?}", contract.address().clone());

    let proxy_contract =
        deploy_contract(client.clone(), SAFE_PROXY_3_0_2, Token::Uint(U256::from(0))).await?;

    log::debug!(
        "ℹ️  Proxy for contract [{:?}] deployed : {:?}",
        contract.address().clone(),
        proxy_contract.address()
    );

    Ok((proxy_contract, contract))
}

pub async fn deploy_contract_behind_safe_proxy_5_0_0<T: Tokenize>(
    client: Arc<LocalWalletSignerMiddleware>,
    contract_path: &str,
    constructor_args: T,
) -> Result<
    (
        ContractInstance<Arc<LocalWalletSignerMiddleware>, LocalWalletSignerMiddleware>,
        ContractInstance<Arc<LocalWalletSignerMiddleware>, LocalWalletSignerMiddleware>,
    ),
    Error,
> {
    let contract = deploy_contract(client.clone(), contract_path, constructor_args).await?;

    log::debug!("ℹ️  Contract deployed : {:?}", contract.address().clone());

    let proxy_contract =
        deploy_contract(client.clone(), SAFE_PROXY_5_0_0, Token::Uint(U256::from(0))).await?;

    log::debug!(
        "ℹ️  Proxy for contract [{:?}] deployed : {:?}",
        contract.address().clone(),
        proxy_contract.address()
    );

    Ok((proxy_contract, contract))
}
