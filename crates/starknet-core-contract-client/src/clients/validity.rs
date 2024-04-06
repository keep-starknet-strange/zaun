use std::sync::Arc;

use alloy::{
    primitives::Address,
    network::Ethereum,
    transports::http::Http,
};

use crate::{
    interfaces::{Operator, ProxySupport, StarknetMessaging, StarknetValidityContract},
    LocalWalletSignerMiddleware,
};

/// Client to interact with a Starknet core contract running in `Validity` mode
pub struct StarknetValidityContractClient {
    core_contract: StarknetValidityContract::StarknetValidityContractInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>,
    messaging: StarknetMessaging::StarknetMessagingInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>,
    operator: Operator::OperatorInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>,
    proxy_support: ProxySupport::ProxySupportInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>,
}

impl StarknetValidityContractClient {
    pub fn new(address: Address, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self {
            core_contract: StarknetValidityContract::new(address, client.clone()),
            messaging: StarknetMessaging::new(address, client.clone()),
            operator: Operator::new(address, client.clone()),
            proxy_support: ProxySupport::new(address, client.clone()),
        }
    }
}

impl AsRef<StarknetValidityContract::StarknetValidityContractInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>>
    for StarknetValidityContractClient
{
    fn as_ref(&self) -> &StarknetValidityContract::StarknetValidityContractInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>> {
        &self.core_contract
    }
}
impl AsRef<StarknetMessaging::StarknetMessagingInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>> for StarknetValidityContractClient {
    fn as_ref(&self) -> &StarknetMessaging::StarknetMessagingInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>> {
        &self.messaging
    }
}
impl AsRef<ProxySupport::ProxySupportInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>> for StarknetValidityContractClient {
    fn as_ref(&self) -> &ProxySupport::ProxySupportInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>> {
        &self.proxy_support
    }
}
impl AsRef<Operator::OperatorInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>> for StarknetValidityContractClient {
    fn as_ref(&self) -> &Operator::OperatorInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>> {
        &self.operator
    }
}
