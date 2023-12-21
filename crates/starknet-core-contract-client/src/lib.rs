mod error;
pub mod interfaces;

use interfaces::*;

pub use error::Error;
use std::sync::Arc;

use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use ethers::types::Address;

pub type LocalMiddleware = SignerMiddleware<Provider<Http>, LocalWallet>;

pub struct StarknetContractClient {
    core_contract: StarknetContract<LocalMiddleware>,
    messaging: StarknetMessaging<LocalMiddleware>,
    operator: Operator<LocalMiddleware>,
    proxy_support: ProxySupport<LocalMiddleware>,
}

impl StarknetContractClient {
    pub fn new(address: Address, client: Arc<LocalMiddleware>) -> Self {
        Self {
            core_contract: StarknetContract::new(address, client.clone()),
            messaging: StarknetMessaging::new(address, client.clone()),
            operator: Operator::new(address, client.clone()),
            proxy_support: ProxySupport::new(address, client),
        }
    }
}

impl AsRef<StarknetContract<LocalMiddleware>> for StarknetContractClient {
    fn as_ref(&self) -> &StarknetContract<LocalMiddleware> {
        &self.core_contract
    }
}
impl AsRef<StarknetMessaging<LocalMiddleware>> for StarknetContractClient {
    fn as_ref(&self) -> &StarknetMessaging<LocalMiddleware> {
        &self.messaging
    }
}
impl AsRef<ProxySupport<LocalMiddleware>> for StarknetContractClient {
    fn as_ref(&self) -> &ProxySupport<LocalMiddleware> {
        &self.proxy_support
    }
}
impl AsRef<Operator<LocalMiddleware>> for StarknetContractClient {
    fn as_ref(&self) -> &Operator<LocalMiddleware> {
        &self.operator
    }
}
