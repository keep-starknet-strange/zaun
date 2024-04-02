use std::sync::Arc;

use ethers::abi::Address;

use crate::interfaces::{Operator, StarknetMessaging, StarknetValidityContract};
use starknet_proxy_client::proxy_support::ProxySupport;
use utils::LocalWalletSignerMiddleware;


/// Client to interact with a Starknet core contract running in `Validity` mode
pub struct StarknetValidityContractClient {
    core_contract: StarknetValidityContract<LocalWalletSignerMiddleware>,
    messaging: StarknetMessaging<LocalWalletSignerMiddleware>,
    operator: Operator<LocalWalletSignerMiddleware>,
    proxy_support: ProxySupport<LocalWalletSignerMiddleware>,
}

impl StarknetValidityContractClient {
    pub fn new(address: Address, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self {
            core_contract: StarknetValidityContract::new(address, client.clone()),
            messaging: StarknetMessaging::new(address, client.clone()),
            operator: Operator::new(address, client.clone()),
            proxy_support: ProxySupport::new(address, client),
        }
    }
}

impl AsRef<StarknetValidityContract<LocalWalletSignerMiddleware>>
    for StarknetValidityContractClient
{
    fn as_ref(&self) -> &StarknetValidityContract<LocalWalletSignerMiddleware> {
        &self.core_contract
    }
}
impl AsRef<StarknetMessaging<LocalWalletSignerMiddleware>> for StarknetValidityContractClient {
    fn as_ref(&self) -> &StarknetMessaging<LocalWalletSignerMiddleware> {
        &self.messaging
    }
}
impl AsRef<ProxySupport<LocalWalletSignerMiddleware>> for StarknetValidityContractClient {
    fn as_ref(&self) -> &ProxySupport<LocalWalletSignerMiddleware> {
        &self.proxy_support
    }
}
impl AsRef<Operator<LocalWalletSignerMiddleware>> for StarknetValidityContractClient {
    fn as_ref(&self) -> &Operator<LocalWalletSignerMiddleware> {
        &self.operator
    }
}
