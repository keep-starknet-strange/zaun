pub mod clients;
mod error;
pub mod interfaces;

use std::sync::Arc;

pub use error::Error;

use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use ethers::types::Address;

pub type LocalWalletSignerMiddleware = SignerMiddleware<Provider<Http>, LocalWallet>;

pub trait StarknetCoreContractClient {
    fn address(&self) -> Address;
    fn client(&self) -> Arc<LocalWalletSignerMiddleware>;
}

pub trait StarknetLegacyBridgeContractClient {
    fn address(&self) -> Address;

    fn client(&self) -> Arc<LocalWalletSignerMiddleware>;
}

pub trait StarknetBridgeContractClient {
    fn address(&self) -> Address;

    fn manager(&self) -> Address;

    fn registry(&self) -> Address;

    fn token(&self) -> Address;

    fn manager_client(&self) -> Arc<LocalWalletSignerMiddleware>;
    fn registry_client(&self) -> Arc<LocalWalletSignerMiddleware>;
    fn bridge_client(&self) -> Arc<LocalWalletSignerMiddleware>;
    fn token_client(&self) -> Arc<LocalWalletSignerMiddleware>;

}