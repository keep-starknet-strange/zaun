use ethers::abi::Tokenize;
use ethers::contract::ContractError;
use ethers::prelude::SignerMiddleware;
use ethers::prelude::{ContractFactory, ContractInstance};
use ethers::providers::{Http, Provider, ProviderError};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::Bytes;
use ethers::utils::hex::FromHex;
use ethers::utils::{Anvil, AnvilInstance};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

pub mod unsafe_proxy;

pub use starknet_core_contract_client::LocalWalletSignerMiddleware;

/// Sandbox is typically used for E2E scenarious so we need to speed things up
const POLLING_INTERVAL_MS: u64 = 10;
const ANVIL_DEFAULT_ENDPOINT: &str = "http://127.0.0.1:8545";
const ANVIL_DEFAULT_CHAIN_ID: u64 = 31337;
const ANVIL_DEFAULT_PRIVATE_KEY: &str =
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

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
}

/// A convenient wrapper over an already running or spawned Anvil local devnet
pub struct EthereumSandbox {
    /// If initialized keeps an Anvil instance to properly shutdown it at the end
    _anvil: Option<AnvilInstance>,
    /// Pre-configured local signer
    client: Arc<LocalWalletSignerMiddleware>,
}

impl EthereumSandbox {
    /// Creates a new sandbox instance.
    /// Will try to attach to already running Anvil instance using one
    /// of the following endpoints:
    ///     - `anvil_endpoint` parameter (if specified)
    ///     - ${ANVIL_ENDPOINT} environment variable (if set)
    ///     - http://127.0.0.1:8545 (default)
    /// Also default values for chain ID and private keys will be used.
    pub fn attach(anvil_endpoint: Option<String>) -> Result<Self, Error> {
        let anvil_endpoint = anvil_endpoint.unwrap_or_else(|| {
            std::env::var("ANVIL_ENDPOINT")
                .map(Into::into)
                .ok()
                .unwrap_or_else(|| ANVIL_DEFAULT_ENDPOINT.into())
        });

        let provider = Provider::<Http>::try_from(anvil_endpoint)
            .map_err(|_| Error::UrlParser)?
            .interval(Duration::from_millis(POLLING_INTERVAL_MS));

        let wallet: LocalWallet = ANVIL_DEFAULT_PRIVATE_KEY
            .parse()
            .expect("Failed to parse private key");
        let client = SignerMiddleware::new(
            provider.clone(),
            wallet.with_chain_id(ANVIL_DEFAULT_CHAIN_ID),
        );

        Ok(Self {
            _anvil: None,
            client: Arc::new(client),
        })
    }

    /// Creates a new sandbox instance.
    /// A new Anvil instance will be spawned using binary located at:
    ///     - `anvil_path` parameter (if specified)
    ///     - ${ANVIL_PATH} environment variable (if set)
    ///     - ~/.foundry/bin/anvil (default)
    pub fn spawn(anvil_path: Option<PathBuf>) -> Self {
        let anvil_path: PathBuf = anvil_path.unwrap_or_else(|| {
            std::env::var("ANVIL_PATH")
                .map(Into::into)
                .ok()
                .unwrap_or_else(|| dirs::home_dir().unwrap().join(".foundry/bin/anvil"))
        });

        // Will panic if invalid path
        let anvil = Anvil::at(anvil_path).spawn();

        let provider = Provider::<Http>::try_from(anvil.endpoint())
            .expect("Failed to connect to Anvil")
            .interval(Duration::from_millis(POLLING_INTERVAL_MS));

        let wallet: LocalWallet = anvil.keys()[0].clone().into();
        let client =
            SignerMiddleware::new(provider.clone(), wallet.with_chain_id(anvil.chain_id()));

        Self {
            _anvil: Some(anvil),
            client: Arc::new(client),
        }
    }

    /// Returns local client configured for the running Anvil instance
    pub fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.client.clone()
    }
}

/// Deploys new smart contract using:
///     - Forge build artifacts (JSON file contents)
///     - Constructor args (use () if no args expected)
pub async fn deploy_contract<T: Tokenize>(
    client: Arc<LocalWalletSignerMiddleware>,
    contract_build_artifacts: &str,
    contructor_args: T,
) -> Result<ContractInstance<Arc<LocalWalletSignerMiddleware>, LocalWalletSignerMiddleware>, Error>
{
    let (abi, bytecode) = {
        let mut artifacts: serde_json::Value = serde_json::from_str(contract_build_artifacts)?;
        let abi = serde_json::from_value(artifacts["abi"].take())?;
        let bytecode = Bytes::from_hex(
            artifacts["bytecode"]["object"]
                .as_str()
                .ok_or(Error::BytecodeObject)?,
        )?;
        (abi, bytecode)
    };

    let factory = ContractFactory::new(abi, bytecode, client.clone());

    Ok(factory
        .deploy(contructor_args)
        .map_err(Into::<ContractError<LocalWalletSignerMiddleware>>::into)?
        .send()
        .await
        .map_err(Into::<ContractError<LocalWalletSignerMiddleware>>::into)?)
}
