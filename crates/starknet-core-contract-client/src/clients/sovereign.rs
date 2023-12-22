use std::sync::Arc;

use crate::{
    interfaces::{Operator, ProxySupport, StarknetMessaging, StarknetSovereignContract},
    LocalMiddleware,
};
use ethers::types::Address;

/// Client to interact with a Starknet core contract running in `Sovereign` mode
pub struct StarknetSovereignContractClient {
    core_contract: StarknetSovereignContract<LocalMiddleware>,
    messaging: StarknetMessaging<LocalMiddleware>,
    operator: Operator<LocalMiddleware>,
    proxy_support: ProxySupport<LocalMiddleware>,
}

impl StarknetSovereignContractClient {
    pub fn new(address: Address, client: Arc<LocalMiddleware>) -> Self {
        Self {
            core_contract: StarknetSovereignContract::new(address, client.clone()),
            messaging: StarknetMessaging::new(address, client.clone()),
            operator: Operator::new(address, client.clone()),
            proxy_support: ProxySupport::new(address, client),
        }
    }
}

impl AsRef<StarknetSovereignContract<LocalMiddleware>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &StarknetSovereignContract<LocalMiddleware> {
        &self.core_contract
    }
}
impl AsRef<StarknetMessaging<LocalMiddleware>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &StarknetMessaging<LocalMiddleware> {
        &self.messaging
    }
}
impl AsRef<ProxySupport<LocalMiddleware>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &ProxySupport<LocalMiddleware> {
        &self.proxy_support
    }
}
impl AsRef<Operator<LocalMiddleware>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &Operator<LocalMiddleware> {
        &self.operator
    }
}
