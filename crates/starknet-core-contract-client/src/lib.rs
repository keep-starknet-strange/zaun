pub mod clients;
pub mod interfaces;

use std::sync::Arc;

use alloy::{
    network::{Ethereum, EthereumSigner}, primitives::Address, providers::{layers::SignerProvider, RootProvider}, transports::http::Http
};

pub type LocalWalletSignerMiddleware = SignerProvider<Ethereum, Http<reqwest::Client>, RootProvider<Ethereum, Http<reqwest::Client>>, EthereumSigner>;

pub trait StarknetCoreContractClient {
    fn address(&self) -> Address;
    fn client(&self) -> Arc<LocalWalletSignerMiddleware>;
}
