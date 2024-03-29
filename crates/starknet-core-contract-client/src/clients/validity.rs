use std::sync::Arc;

use alloy::{
    primitives::Address,
    network::Ethereum,
    transports::http::Http
};

use crate::{
    interfaces::{Operator, ProxySupport, StarknetMessaging, StarknetValidityContract},
    LocalWalletSignerMiddleware,
};

/// Client to interact with a Starknet core contract running in `Validity` mode
pub struct StarknetValidityContractClient {
    core_contract: StarknetValidityContract::StarknetValidityContractInstance<Ethereum, Http<reqwest::Client>, LocalWalletSignerMiddleware>,
    messaging: StarknetMessaging::StarknetMessagingInstance<Ethereum, Http<reqwest::Client>, LocalWalletSignerMiddleware>,
    operator: Operator::OperatorInstance<Ethereum, Http<reqwest::Client>, LocalWalletSignerMiddleware>,
    proxy_support: ProxySupport::ProxySupportInstance<Ethereum, Http<reqwest::Client>, LocalWalletSignerMiddleware>,
}

impl StarknetValidityContractClient {
    pub fn new(address: Address, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self {
            core_contract: StarknetValidityContract::StarknetValidityContractInstance::new(address, client.clone()),
            messaging: StarknetMessaging::StarknetMessagingInstance::new(address, client.clone()),
            operator: Operator::OperatorInstance::new(address, client.clone()),
            proxy_support: ProxySupport::ProxySupportInstance::new(address, client),
        }
    }
}

impl AsRef<StarknetValidityContract::StarknetValidityContractInstance<Ethereum, Http<reqwest::Client>, LocalWalletSignerMiddleware>>
    for StarknetValidityContractClient
{
    fn as_ref(&self) -> &StarknetValidityContract::StarknetValidityContractInstance<Ethereum, Http<reqwest::Client>, LocalWalletSignerMiddleware> {
        &self.core_contract
    }
}
impl AsRef<StarknetMessaging::StarknetMessagingInstance<Ethereum, Http<reqwest::Client>, LocalWalletSignerMiddleware>> for StarknetValidityContractClient {
    fn as_ref(&self) -> &StarknetMessaging::StarknetMessagingInstance<Ethereum, Http<reqwest::Client>, LocalWalletSignerMiddleware> {
        &self.messaging
    }
}
impl AsRef<ProxySupport::ProxySupportInstance<Ethereum, Http<reqwest::Client>, LocalWalletSignerMiddleware>> for StarknetValidityContractClient {
    fn as_ref(&self) -> &ProxySupport::ProxySupportInstance<Ethereum, Http<reqwest::Client>, LocalWalletSignerMiddleware> {
        &self.proxy_support
    }
}
impl AsRef<Operator::OperatorInstance<Ethereum, Http<reqwest::Client>, LocalWalletSignerMiddleware>> for StarknetValidityContractClient {
    fn as_ref(&self) -> &Operator::OperatorInstance<Ethereum, Http<reqwest::Client>, LocalWalletSignerMiddleware> {
        &self.operator
    }
}
