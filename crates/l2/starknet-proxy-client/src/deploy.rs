use ethereum_instance::deploy_contract;
use ethers::abi::{Token, Tokenize};
use ethers::contract::ContractError;
use ethers::prelude::ContractInstance;
use ethers::providers::ProviderError;
use ethers::types::U256;
use ethers::utils::hex::{self};
use std::cmp::PartialEq;
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
const SAFE_PROXY_3_0_2: &str = include_str!("artifacts/Proxy_3_0_2.json");
const SAFE_PROXY_5_0_0: &str = include_str!("artifacts/Proxy_5_0_0.json");

#[derive(PartialEq)]
pub enum ProxyVersion {
    /// deploys unsafe proxy.
    UnsafeProxy,
    /// deploys safe proxy (starknet proxy version 3.0.2)
    SafeProxy3_0_2,
    /// deploys safe proxy (starknet proxy version 5.0.0)
    SafeProxy5_0_0,
}

pub async fn deploy_contract_behind_proxy<T: Tokenize>(
    client: Arc<LocalWalletSignerMiddleware>,
    contract_path: &str,
    constructor_args: T,
    proxy_type: ProxyVersion,
) -> Result<
    (
        ContractInstance<Arc<LocalWalletSignerMiddleware>, LocalWalletSignerMiddleware>,
        ContractInstance<Arc<LocalWalletSignerMiddleware>, LocalWalletSignerMiddleware>,
    ),
    Error,
> {
    let contract = deploy_contract(client.clone(), contract_path, constructor_args).await?;

    log::debug!("ℹ️  Contract deployed : {:?}", contract.address().clone());

    let proxy_code = match proxy_type {
        ProxyVersion::SafeProxy3_0_2 => SAFE_PROXY_3_0_2,
        ProxyVersion::SafeProxy5_0_0 => SAFE_PROXY_5_0_0,
        ProxyVersion::UnsafeProxy => UNSAFE_PROXY,
    };

    let proxy_contract = match proxy_type {
        ProxyVersion::UnsafeProxy => {
            deploy_contract(client.clone(), proxy_code, contract.address()).await?
        }
        _ => deploy_contract(client.clone(), proxy_code, Token::Uint(U256::from(0))).await?,
    };

    log::debug!(
        "ℹ️  Proxy for contract [{:?}] deployed : {:?}",
        contract.address().clone(),
        proxy_contract.address()
    );

    Ok((proxy_contract, contract))
}
