use std::sync::Arc;

use crate::{
    interfaces::{
        GovernedFinalizable, Operator, ProxySupport, StarknetGovernance, StarknetMessaging,
        StarknetSovereignContract,
    },
    LocalWalletSignerMiddleware, StarknetCoreContractClient,
};
use alloy::{
    network::Ethereum, primitives::Address, transports::http::Http
};
/// Client to interact with a Starknet core contract running in `Sovereign` mode
pub struct StarknetSovereignContractClient {
    core_contract: StarknetSovereignContract::StarknetSovereignContractInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>,
    messaging: StarknetMessaging::StarknetMessagingInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>,
    operator: Operator::OperatorInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>,
    proxy_support: ProxySupport::ProxySupportInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>,
    governance: StarknetGovernance::StarknetGovernanceInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>,
    governed_finalizable: GovernedFinalizable::GovernedFinalizableInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>,
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

impl AsRef<StarknetSovereignContract::StarknetSovereignContractInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>>
    for StarknetSovereignContractClient
{
    fn as_ref(&self) -> &StarknetSovereignContract::StarknetSovereignContractInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>> {
        &self.core_contract
    }
}
impl AsRef<StarknetMessaging::StarknetMessagingInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &StarknetMessaging::StarknetMessagingInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>> {
        &self.messaging
    }
}
impl AsRef<ProxySupport::ProxySupportInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &ProxySupport::ProxySupportInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>> {
        &self.proxy_support
    }
}
impl AsRef<Operator::OperatorInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &Operator::OperatorInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>> {
        &self.operator
    }
}
impl AsRef<StarknetGovernance::StarknetGovernanceInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &StarknetGovernance::StarknetGovernanceInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>> {
        &self.governance
    }
}
impl AsRef<GovernedFinalizable::GovernedFinalizableInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &GovernedFinalizable::GovernedFinalizableInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>> {
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
