use ethers::abi::Tokenize;
use ethers::contract::{ContractError, ContractFactory, ContractInstance};
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Provider, ProviderError};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::Bytes;
use ethers::utils::{Anvil, AnvilInstance};
use hex::FromHex;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

/// Ethers library allows multiple signer backends and transports.
/// For simplicity we use local wallet (basically private key) and
/// HTTP transport in this crate.
pub use utils::LocalWalletSignerMiddleware;

/// Sandbox is typically used for E2E scenarios so we need to speed things up
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
    #[error("Invalid contract build artifacts: missing field `{0}`")]
    ContractBuildArtifacts(&'static str),
}

/// A convenient wrapper over an already running or spawned Anvil local devnet or ethereum
#[allow(dead_code)]
pub struct EthereumClient {
    /// If initialized keeps an Anvil instance to properly shutdown it at the end
    client: Option<AnvilInstance>,
    /// Pre-configured local signer
    signer: Arc<LocalWalletSignerMiddleware>,
}

impl EthereumClient {
    /// Creates a new sandbox instance.
    /// Will try to attach to already running Anvil instance or custom rpc and private key provided to the function.
    /// if not provided any argument it will attack to a default anvil instance with default anvil params.
    pub fn attach(
        rpc_endpoint: Option<String>,
        priv_key: Option<String>,
        chain_id: Option<u64>,
    ) -> Result<Self, Error> {
        let rpc_endpoint = rpc_endpoint.unwrap_or_else(|| {
            std::env::var("ETH_RPC_ENDPOINT")
                .map(Into::into)
                .ok()
                .unwrap_or_else(|| ANVIL_DEFAULT_ENDPOINT.into())
        });

        let provider = Provider::<Http>::try_from(rpc_endpoint)
            .map_err(|_| Error::UrlParser)?
            .interval(Duration::from_millis(POLLING_INTERVAL_MS));

        let priv_key = priv_key.unwrap_or_else(|| ANVIL_DEFAULT_PRIVATE_KEY.to_owned());

        let wallet: LocalWallet = priv_key.parse().expect("Failed to parse private key");

        let chain_id = chain_id.unwrap_or(ANVIL_DEFAULT_CHAIN_ID);

        let client = SignerMiddleware::new(provider.clone(), wallet.with_chain_id(chain_id));

        Ok(Self {
            client: None,
            signer: Arc::new(client),
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
            client: Some(anvil),
            signer: Arc::new(client),
        }
    }

    /// Returns local client configured for the running Anvil instance
    pub fn signer(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.signer.clone()
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

        let abi_value = artifacts
            .get_mut("abi")
            .ok_or_else(|| Error::ContractBuildArtifacts("abi"))?
            .take();
        let bytecode_value = artifacts
            .get_mut("bytecode")
            .ok_or_else(|| Error::ContractBuildArtifacts("bytecode"))?
            .get_mut("object")
            .ok_or_else(|| Error::ContractBuildArtifacts("bytecode.object"))?
            .take();

        let abi = serde_json::from_value(abi_value)?;
        let bytecode = Bytes::from_hex(bytecode_value.as_str().ok_or(Error::BytecodeObject)?)?;
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
