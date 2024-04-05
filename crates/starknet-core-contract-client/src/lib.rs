pub mod clients;
mod error;
pub mod interfaces;

use std::sync::Arc;

// pub use error::Error;

use alloy::{
    network::{Ethereum, EthereumSigner}, primitives::Address, providers::{layers::{GasEstimatorProvider, ManagedNonceProvider, SignerProvider}, Provider, RootProvider}, transports::{http::Http, RpcError, TransportErrorKind}
};

pub type LocalWalletSignerMiddleware = SignerProvider<Ethereum, Http<reqwest::Client>, RootProvider<Ethereum, Http<reqwest::Client>>, EthereumSigner>;

pub trait StarknetCoreContractClient {
    fn address(&self) -> Address;
    fn client(&self) -> Arc<LocalWalletSignerMiddleware>;
}
