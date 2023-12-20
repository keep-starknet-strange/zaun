mod error;
mod starknet_core_contract;
// mod starknet_messaging;

pub use error::Error;
pub use starknet_core_contract::StarknetContract;
use std::sync::Arc;

use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use ethers::types::Address;

pub type LocalMiddleware = SignerMiddleware<Provider<Http>, LocalWallet>;

pub struct StarknetContractClient {
    core_contract: StarknetContract<LocalMiddleware>,
}

impl StarknetContractClient {
    pub fn new(address: Address, client: Arc<LocalMiddleware>) -> Self {
        Self {
            core_contract: StarknetContract::new(address, client),
        }
    }
}

impl AsRef<StarknetContract<LocalMiddleware>> for StarknetContractClient {
    fn as_ref(&self) -> &StarknetContract<LocalMiddleware> {
        &self.core_contract
    }
}
