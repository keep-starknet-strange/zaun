use url::Url;
use alloy::{
    network::{Ethereum, EthereumSigner},
    node_bindings::{Anvil, AnvilInstance},
    providers::ProviderBuilder,
    signers::{
        wallet::{LocalWallet, WalletError},
        Signer,
    },
    transports::TransportError,
    rpc::client::RpcClient,
};

use std::path::PathBuf;
use std::sync::Arc;

/// Unsafe proxy is a straightforward implementation of the delegate proxy contract
/// that is used to make Starknet core contract upgradeable.
/// This implementation DOES NOT restrict who can invoke the core contract.
/// For more information see https://l2beat.com/scaling/projects/starknet#contracts
pub mod unsafe_proxy;

/// Ethers library allows multiple signer backends and transports.
/// For simplicity we use local wallet (basically private key) and
/// HTTP transport in this crate.
pub use starknet_core_contract_client::LocalWalletSignerMiddleware;

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
    #[error("Failed to parse provider URL: {0}")]
    ProviderUrlParse(#[source] url::ParseError),
    #[error(transparent)]
    EthersProvider(#[from] TransportError),
    #[error("Invalid contract build artifacts: missing field `{0}`")]
    ContractBuildArtifacts(&'static str),
    #[error("Failed to parse private key: {0}")]
    PrivateKeyParse(#[source] WalletError),
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

        let wallet: LocalWallet = String::from(ANVIL_DEFAULT_PRIVATE_KEY).parse::<LocalWallet>().map_err(Error::PrivateKeyParse)?;
        let wallet = wallet.with_chain_id(Some(ANVIL_DEFAULT_CHAIN_ID));
        let rpc_client = RpcClient::new_http(Url::parse(&anvil_endpoint).map_err(Error::ProviderUrlParse)?);
        // let http_provider = RootProvider::<Ethereum, BoxTransport>::connect_builtin(anvil_endpoint.as_str()).await?;
        let provider_with_signer = ProviderBuilder::<_, Ethereum>::new()
            .signer(EthereumSigner::from(wallet))
            .on_client(rpc_client);
            // .provider(http_provider);

        Ok(Self {
            _anvil: None,
            client: Arc::new(provider_with_signer),
        })
    }

    /// Creates a new sandbox instance.
    /// A new Anvil instance will be spawned using binary located at:
    ///     - `anvil_path` parameter (if specified)
    ///     - ${ANVIL_PATH} environment variable (if set)
    ///     - ~/.foundry/bin/anvil (default)
    pub fn spawn(anvil_path: Option<PathBuf>) -> Result<Self, Error> {
        let anvil_path: PathBuf = anvil_path.unwrap_or_else(|| {
            std::env::var("ANVIL_PATH")
                .map(Into::into)
                .ok()
                .unwrap_or_else(|| dirs::home_dir().unwrap().join(".foundry/bin/anvil"))
        });

        // Will panic if invalid path
        // let anvil = Anvil::at(anvil_path).spawn();
        let anvil = Anvil::at(anvil_path).spawn();

        let wallet: LocalWallet = anvil.keys()[0].clone().try_into().expect("Failed to parse private key");
        let rpc_client = RpcClient::new_http(Url::parse(&anvil.endpoint()).map_err(Error::ProviderUrlParse)?);
        // let http_provider = RootProvider::<Ethereum, BoxTransport>::connect_builtin(anvil.endpoint().as_str()).await?;
        let provider_with_signer = ProviderBuilder::<_, Ethereum>::new()
            .signer(EthereumSigner::from(wallet))
            .on_client(rpc_client);
            // .provider(http_provider);

        Ok(Self {
            _anvil: Some(anvil),
            client: Arc::new(provider_with_signer),
        })
    }

    /// Returns local client configured for the running Anvil instance
    pub fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.client.clone()
    }
}
