use std::sync::Arc;

use crate::interfaces::{
    GovernedFinalizable, Operator, StarknetCoreContract, StarknetGovernance, StarknetMessaging,
};
use ethers::types::Address;
use starknet_proxy_client::clients::proxy_3_0_2::ProxySupport3_0_2;
use utils::{LocalWalletSignerMiddleware, StarknetContractClient};

/// Client to interact with a Starknet core contract running in `Sovereign` mode
#[derive(Clone)]
pub struct StarknetCoreContractClient {
    core_contract: StarknetCoreContract<LocalWalletSignerMiddleware>,
    messaging: StarknetMessaging<LocalWalletSignerMiddleware>,
    operator: Operator<LocalWalletSignerMiddleware>,
    proxy_support: ProxySupport3_0_2<LocalWalletSignerMiddleware>,
    governance: StarknetGovernance<LocalWalletSignerMiddleware>,
    governed_finalizable: GovernedFinalizable<LocalWalletSignerMiddleware>,
    core_contract_implementation: Address,
}

impl StarknetCoreContractClient {
    pub fn new(
        address: Address,
        client: Arc<LocalWalletSignerMiddleware>,
        implementation_address: Address,
    ) -> Self {
        Self {
            core_contract: StarknetCoreContract::new(address, client.clone()),
            messaging: StarknetMessaging::new(address, client.clone()),
            operator: Operator::new(address, client.clone()),
            proxy_support: ProxySupport3_0_2::new(address, client.clone()),
            governance: StarknetGovernance::new(address, client.clone()),
            governed_finalizable: GovernedFinalizable::new(address, client.clone()),
            core_contract_implementation: implementation_address,
        }
    }
}

impl AsRef<StarknetCoreContract<LocalWalletSignerMiddleware>> for StarknetCoreContractClient {
    fn as_ref(&self) -> &StarknetCoreContract<LocalWalletSignerMiddleware> {
        &self.core_contract
    }
}
impl AsRef<StarknetMessaging<LocalWalletSignerMiddleware>> for StarknetCoreContractClient {
    fn as_ref(&self) -> &StarknetMessaging<LocalWalletSignerMiddleware> {
        &self.messaging
    }
}
impl AsRef<ProxySupport3_0_2<LocalWalletSignerMiddleware>> for StarknetCoreContractClient {
    fn as_ref(&self) -> &ProxySupport3_0_2<LocalWalletSignerMiddleware> {
        &self.proxy_support
    }
}
impl AsRef<Operator<LocalWalletSignerMiddleware>> for StarknetCoreContractClient {
    fn as_ref(&self) -> &Operator<LocalWalletSignerMiddleware> {
        &self.operator
    }
}
impl AsRef<StarknetGovernance<LocalWalletSignerMiddleware>> for StarknetCoreContractClient {
    fn as_ref(&self) -> &StarknetGovernance<LocalWalletSignerMiddleware> {
        &self.governance
    }
}
impl AsRef<GovernedFinalizable<LocalWalletSignerMiddleware>> for StarknetCoreContractClient {
    fn as_ref(&self) -> &GovernedFinalizable<LocalWalletSignerMiddleware> {
        &self.governed_finalizable
    }
}

impl StarknetContractClient for StarknetCoreContractClient {
    fn address(&self) -> Address {
        self.core_contract.address()
    }
    fn implementation_address(&self) -> Address {
        self.core_contract_implementation
    }
    fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.core_contract.client()
    }
}
