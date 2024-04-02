pub mod errors;
use std::sync::Arc;
use ethers::types::Address;
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;

pub type LocalWalletSignerMiddleware = SignerMiddleware<Provider<Http>, LocalWallet>;

pub trait StarknetContractClient {
    fn address(&self) -> Address;
    fn client(&self) -> Arc<LocalWalletSignerMiddleware>;
}