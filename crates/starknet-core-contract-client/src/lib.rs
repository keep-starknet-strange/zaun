pub mod clients;
mod error;
pub mod interfaces;

use std::sync::Arc;

pub use error::Error;

use alloy::{
    primitives::Address,
    network::{Ethereum, EthereumSigner},
    providers::{layers::SignerProvider, RootProvider},
    transports::http::Http
};

pub type LocalWalletSignerMiddleware = SignerProvider<Ethereum, Http<reqwest::Client>, RootProvider<Ethereum, Http<reqwest::Client>>, EthereumSigner>;

pub trait StarknetCoreContractClient {
    fn address(&self) -> Address;
    fn client(&self) -> Arc<LocalWalletSignerMiddleware>;
}
