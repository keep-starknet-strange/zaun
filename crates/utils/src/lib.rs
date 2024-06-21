pub mod errors;
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use ethers::types::Address;
use std::sync::Arc;

pub type LocalWalletSignerMiddleware = SignerMiddleware<Provider<Http>, LocalWallet>;

pub const NO_CONSTRUCTOR_ARG: () = ();

pub trait StarknetContractClient {
    fn address(&self) -> Address;
    fn client(&self) -> Arc<LocalWalletSignerMiddleware>;
}
