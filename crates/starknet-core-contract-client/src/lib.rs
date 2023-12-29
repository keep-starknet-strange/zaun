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
