use std::sync::Arc;

use ethers::abi::Address;

use crate::{
    interfaces::{Operator, ProxySupport, StarknetMessaging, StarknetValidityContract},
    LocalMiddleware,
};

/// Client to interact with a Starknet core contract running in `Validity` mode
pub struct StarknetValidityContractClient {
    core_contract: StarknetValidityContract<LocalMiddleware>,
    messaging: StarknetMessaging<LocalMiddleware>,
    operator: Operator<LocalMiddleware>,
    proxy_support: ProxySupport<LocalMiddleware>,
}

impl StarknetValidityContractClient {
    pub fn new(address: Address, client: Arc<LocalMiddleware>) -> Self {
        Self {
            core_contract: StarknetValidityContract::new(address, client.clone()),
            messaging: StarknetMessaging::new(address, client.clone()),
            operator: Operator::new(address, client.clone()),
            proxy_support: ProxySupport::new(address, client),
        }
    }
}

impl AsRef<StarknetValidityContract<LocalMiddleware>> for StarknetValidityContractClient {
    fn as_ref(&self) -> &StarknetValidityContract<LocalMiddleware> {
        &self.core_contract
    }
}
impl AsRef<StarknetMessaging<LocalMiddleware>> for StarknetValidityContractClient {
    fn as_ref(&self) -> &StarknetMessaging<LocalMiddleware> {
        &self.messaging
    }
}
impl AsRef<ProxySupport<LocalMiddleware>> for StarknetValidityContractClient {
    fn as_ref(&self) -> &ProxySupport<LocalMiddleware> {
        &self.proxy_support
    }
}
impl AsRef<Operator<LocalMiddleware>> for StarknetValidityContractClient {
    fn as_ref(&self) -> &Operator<LocalMiddleware> {
        &self.operator
    }
}
