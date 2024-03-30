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
    core_contract: StarknetSovereignContract::StarknetSovereignContractInstance<Ethereum, BoxTransport, Arc<LocalWalletSignerMiddleware>>,
    messaging: StarknetMessaging::StarknetMessagingInstance<Ethereum, BoxTransport, Arc<LocalWalletSignerMiddleware>>,
    operator: Operator::OperatorInstance<Ethereum, BoxTransport, Arc<LocalWalletSignerMiddleware>>,
    proxy_support: ProxySupport::ProxySupportInstance<Ethereum, BoxTransport, Arc<LocalWalletSignerMiddleware>>,
    governance: StarknetGovernance::StarknetGovernanceInstance<Ethereum, BoxTransport, Arc<LocalWalletSignerMiddleware>>,
    governed_finalizable: GovernedFinalizable::GovernedFinalizableInstance<Ethereum, BoxTransport, Arc<LocalWalletSignerMiddleware>>,
}

impl StarknetSovereignContractClient {
    pub fn new(address: Address, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self {
            core_contract: StarknetSovereignContract::new(address, client.clone()),
            messaging: StarknetMessaging::new(address, client.clone()),
            operator: Operator::new(address, client.clone()),
            proxy_support: ProxySupport::new(address, client.clone()),
            governance: StarknetGovernance::new(address, client.clone()),
            governed_finalizable: GovernedFinalizable::new(address, client.clone()),
        }
    }
}

impl AsRef<StarknetSovereignContract::StarknetSovereignContractInstance<Ethereum, BoxTransport, Arc<LocalWalletSignerMiddleware>>>
    for StarknetSovereignContractClient
{
    fn as_ref(&self) -> &StarknetSovereignContract::StarknetSovereignContractInstance<Ethereum, BoxTransport, Arc<LocalWalletSignerMiddleware>> {
        &self.core_contract
    }
}
impl AsRef<StarknetMessaging::StarknetMessagingInstance<Ethereum, BoxTransport, Arc<LocalWalletSignerMiddleware>>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &StarknetMessaging::StarknetMessagingInstance<Ethereum, BoxTransport, Arc<LocalWalletSignerMiddleware>> {
        &self.messaging
    }
}
impl AsRef<ProxySupport::ProxySupportInstance<Ethereum, BoxTransport, Arc<LocalWalletSignerMiddleware>>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &ProxySupport::ProxySupportInstance<Ethereum, BoxTransport, Arc<LocalWalletSignerMiddleware>> {
        &self.proxy_support
    }
}
impl AsRef<Operator::OperatorInstance<Ethereum, BoxTransport, Arc<LocalWalletSignerMiddleware>>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &Operator::OperatorInstance<Ethereum, BoxTransport, Arc<LocalWalletSignerMiddleware>> {
        &self.operator
    }
}
impl AsRef<StarknetGovernance::StarknetGovernanceInstance<Ethereum, BoxTransport, Arc<LocalWalletSignerMiddleware>>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &StarknetGovernance::StarknetGovernanceInstance<Ethereum, BoxTransport, Arc<LocalWalletSignerMiddleware>> {
        &self.governance
    }
}
impl AsRef<GovernedFinalizable::GovernedFinalizableInstance<Ethereum, BoxTransport, Arc<LocalWalletSignerMiddleware>>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &GovernedFinalizable::GovernedFinalizableInstance<Ethereum, BoxTransport, Arc<LocalWalletSignerMiddleware>> {
        &self.governed_finalizable
    }
}

impl StarknetCoreContractClient for StarknetSovereignContractClient {
    fn address(&self) -> Address {
        *self.core_contract.address()
    }

    fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.core_contract.provider().clone()
    }
}
