pub mod clients;
mod error;
pub mod interfaces;

use std::sync::Arc;

pub use error::Error;

use alloy::{
    primitives::Address,
    network::{Ethereum, EthereumSigner},
    providers::{layers::SignerProvider, RootProvider},
    transports::BoxTransport
};

pub type LocalWalletSignerMiddleware = SignerProvider<Ethereum, BoxTransport, RootProvider<Ethereum, BoxTransport>, EthereumSigner>;

pub trait StarknetCoreContractClient {
    fn address(&self) -> Address;
    fn client(&self) -> Arc<LocalWalletSignerMiddleware>;
}
