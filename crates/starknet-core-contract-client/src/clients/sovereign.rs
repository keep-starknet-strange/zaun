use std::sync::Arc;

use crate::{
    interfaces::{
        GovernedFinalizable, Operator, ProxySupport, StarknetGovernance, StarknetMessaging,
        StarknetSovereignContract,
    },
    LocalWalletSignerMiddleware, StarknetCoreContractClient,
};
use alloy::{
    network::Ethereum, primitives::Address, transports::BoxTransport
};
/// Client to interact with a Starknet core contract running in `Sovereign` mode
pub struct StarknetSovereignContractClient {
    core_contract: StarknetSovereignContract::StarknetSovereignContractInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware>,
    messaging: StarknetMessaging::StarknetMessagingInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware>,
    operator: Operator::OperatorInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware>,
    proxy_support: ProxySupport::ProxySupportInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware>,
    governance: StarknetGovernance::StarknetGovernanceInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware>,
    governed_finalizable: GovernedFinalizable::GovernedFinalizableInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware>,
}

impl StarknetSovereignContractClient {
    pub fn new(address: Address, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        let client = *client;
        Self {
            core_contract: StarknetSovereignContract::StarknetSovereignContractInstance::new(address, client),
            messaging: StarknetMessaging::StarknetMessagingInstance::new(address, client),
            operator: Operator::OperatorInstance::new(address, client),
            proxy_support: ProxySupport::ProxySupportInstance::new(address, client),
            governance: StarknetGovernance::StarknetGovernanceInstance::new(address, client),
            governed_finalizable: GovernedFinalizable::GovernedFinalizableInstance::new(address, client),
        }
    }
}

impl AsRef<StarknetSovereignContract::StarknetSovereignContractInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware>>
    for StarknetSovereignContractClient
{
    fn as_ref(&self) -> &StarknetSovereignContract::StarknetSovereignContractInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware> {
        &self.core_contract
    }
}
impl AsRef<StarknetMessaging::StarknetMessagingInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &StarknetMessaging::StarknetMessagingInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware> {
        &self.messaging
    }
}
impl AsRef<ProxySupport::ProxySupportInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &ProxySupport::ProxySupportInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware> {
        &self.proxy_support
    }
}
impl AsRef<Operator::OperatorInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &Operator::OperatorInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware> {
        &self.operator
    }
}
impl AsRef<StarknetGovernance::StarknetGovernanceInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &StarknetGovernance::StarknetGovernanceInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware> {
        &self.governance
    }
}
impl AsRef<GovernedFinalizable::GovernedFinalizableInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &GovernedFinalizable::GovernedFinalizableInstance<Ethereum, BoxTransport, LocalWalletSignerMiddleware> {
        &self.governed_finalizable
    }
}

impl StarknetCoreContractClient for StarknetSovereignContractClient {
    fn address(&self) -> Address {
        *self.core_contract.address()
    }

    fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        Arc::new(*self.core_contract.provider().clone())
    }
}
