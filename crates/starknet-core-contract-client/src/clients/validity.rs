use std::sync::Arc;

use alloy::{
    primitives::Address,
    network::Ethereum,
    transports::BoxTransport
};

use crate::{
    interfaces::{Operator, ProxySupport, StarknetMessaging, StarknetValidityContract},
    LocalWalletSignerMiddleware,
};

/// Client to interact with a Starknet core contract running in `Validity` mode
pub struct StarknetValidityContractClient {
    core_contract: StarknetValidityContract::StarknetValidityContractInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware>,
    messaging: StarknetMessaging::StarknetMessagingInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware>,
    operator: Operator::OperatorInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware>,
    proxy_support: ProxySupport::ProxySupportInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware>,
}

impl StarknetValidityContractClient {
    pub fn new(address: Address, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        // let client = *client;
        Self {
            core_contract: StarknetValidityContract::StarknetValidityContractInstance::new(address, client),
            messaging: StarknetMessaging::StarknetMessagingInstance::new(address, client),
            operator: Operator::OperatorInstance::new(address, client),
            proxy_support: ProxySupport::ProxySupportInstance::new(address, client),
        }
    }
}

impl AsRef<StarknetValidityContract::StarknetValidityContractInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware>>
    for StarknetValidityContractClient
{
    fn as_ref(&self) -> &StarknetValidityContract::StarknetValidityContractInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware> {
        &self.core_contract
    }
}
impl AsRef<StarknetMessaging::StarknetMessagingInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware>> for StarknetValidityContractClient {
    fn as_ref(&self) -> &StarknetMessaging::StarknetMessagingInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware> {
        &self.messaging
    }
}
impl AsRef<ProxySupport::ProxySupportInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware>> for StarknetValidityContractClient {
    fn as_ref(&self) -> &ProxySupport::ProxySupportInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware> {
        &self.proxy_support
    }
}
impl AsRef<Operator::OperatorInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware>> for StarknetValidityContractClient {
    fn as_ref(&self) -> &Operator::OperatorInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware> {
        &self.operator
    }
}
