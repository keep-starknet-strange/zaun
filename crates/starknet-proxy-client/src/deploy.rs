use std::sync::Arc;
use ethers::abi::Tokenize;
use ethers::contract::ContractError;
use ethers::prelude::ContractInstance;
use ethers::providers::ProviderError;
use ethers::utils::hex::{self};
use utils::LocalWalletSignerMiddleware;
use ethereum_instance::deploy_contract;

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
}

const UNSAFE_PROXY: &str = include_str!("./artifacts/UnsafeProxy.json");

/// Deploys new unsafe proxy contract:
///     - Implementation can be set only once at initialization
///     - Traditional (Safe) proxies can be upgraded multiple times
pub async fn deploy_contract_behind_unsafe_proxy<T: Tokenize>(
    client: Arc<LocalWalletSignerMiddleware>,
    contract_path: &str,
    constructor_args: T,
) -> Result<ContractInstance<Arc<LocalWalletSignerMiddleware>, LocalWalletSignerMiddleware>, Error> {
    let contract = deploy_contract(client.clone(), contract_path, constructor_args).await.unwrap();

    let proxy_contract =
        deploy_contract(client.clone(), UNSAFE_PROXY, contract.address()).await.unwrap();

    return Ok(proxy_contract);
}