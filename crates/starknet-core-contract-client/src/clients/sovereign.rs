use std::sync::Arc;

use crate::interfaces::{
    GovernedFinalizable, Operator, StarknetGovernance, StarknetMessaging, StarknetSovereignContract,
};
use ethers::types::Address;
use starknet_proxy_client::proxy_support::ProxySupport;
use utils::{LocalWalletSignerMiddleware, StarknetContractClient};

/// Client to interact with a Starknet core contract running in `Sovereign` mode
pub struct StarknetSovereignContractClient {
    core_contract: StarknetSovereignContract<LocalWalletSignerMiddleware>,
    messaging: StarknetMessaging<LocalWalletSignerMiddleware>,
    operator: Operator<LocalWalletSignerMiddleware>,
    proxy_support: ProxySupport<LocalWalletSignerMiddleware>,
    governance: StarknetGovernance<LocalWalletSignerMiddleware>,
    governed_finalizable: GovernedFinalizable<LocalWalletSignerMiddleware>,
    core_contract_implementation: StarknetSovereignContract<LocalWalletSignerMiddleware>,
}

impl StarknetSovereignContractClient {
    pub fn new(
        address: Address,
        client: Arc<LocalWalletSignerMiddleware>,
        implementation_address: Address,
    ) -> Self {
        Self {
            core_contract: StarknetSovereignContract::new(address, client.clone()),
            messaging: StarknetMessaging::new(address, client.clone()),
            operator: Operator::new(address, client.clone()),
            proxy_support: ProxySupport::new(address, client.clone()),
            governance: StarknetGovernance::new(address, client.clone()),
            governed_finalizable: GovernedFinalizable::new(address, client.clone()),
            core_contract_implementation: StarknetSovereignContract::new(
                implementation_address,
                client.clone(),
            ),
        }
    }
}

impl AsRef<StarknetSovereignContract<LocalWalletSignerMiddleware>>
    for StarknetSovereignContractClient
{
    fn as_ref(&self) -> &StarknetSovereignContract<LocalWalletSignerMiddleware> {
        &self.core_contract
    }
}
impl AsRef<StarknetMessaging<LocalWalletSignerMiddleware>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &StarknetMessaging<LocalWalletSignerMiddleware> {
        &self.messaging
    }
}
impl AsRef<ProxySupport<LocalWalletSignerMiddleware>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &ProxySupport<LocalWalletSignerMiddleware> {
        &self.proxy_support
    }
}
impl AsRef<Operator<LocalWalletSignerMiddleware>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &Operator<LocalWalletSignerMiddleware> {
        &self.operator
    }
}
impl AsRef<StarknetGovernance<LocalWalletSignerMiddleware>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &StarknetGovernance<LocalWalletSignerMiddleware> {
        &self.governance
    }
}
impl AsRef<GovernedFinalizable<LocalWalletSignerMiddleware>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &GovernedFinalizable<LocalWalletSignerMiddleware> {
        &self.governed_finalizable
    }
}

impl StarknetContractClient for StarknetSovereignContractClient {
    fn address(&self) -> Address {
        self.core_contract.address()
    }
    fn implementation_address(&self) -> Address {
        self.core_contract_implementation.address()
    }
    fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.core_contract.client()
    }
}
