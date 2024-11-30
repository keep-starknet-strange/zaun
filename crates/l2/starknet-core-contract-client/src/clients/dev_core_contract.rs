use std::sync::Arc;

use ethers::abi::Address;
use starknet_proxy_client::clients::proxy_3_0_2::ProxySupport3_0_2;

use crate::interfaces::{
    GovernedFinalizable, Operator, StarknetDevCoreContract, StarknetGovernance, StarknetMessaging,
};
use utils::{LocalWalletSignerMiddleware, StarknetContractClient};

/// Client to interact with a Starknet core contract running in `Validity` mode
#[derive(Clone)]
pub struct StarknetDevCoreContractClient {
    core_contract: StarknetDevCoreContract<LocalWalletSignerMiddleware>,
    messaging: StarknetMessaging<LocalWalletSignerMiddleware>,
    operator: Operator<LocalWalletSignerMiddleware>,
    proxy_support: ProxySupport3_0_2<LocalWalletSignerMiddleware>,
    governance: StarknetGovernance<LocalWalletSignerMiddleware>,
    governed_finalizable: GovernedFinalizable<LocalWalletSignerMiddleware>,
    core_contract_implementation: ethers::addressbook::Address,
}

impl StarknetDevCoreContractClient {
    pub fn new(
        address: Address,
        client: Arc<LocalWalletSignerMiddleware>,
        implementation_address: Address,
    ) -> Self {
        Self {
            core_contract: StarknetDevCoreContract::new(address, client.clone()),
            messaging: StarknetMessaging::new(address, client.clone()),
            operator: Operator::new(address, client.clone()),
            proxy_support: ProxySupport3_0_2::new(address, client.clone()),
            governance: StarknetGovernance::new(address, client.clone()),
            governed_finalizable: GovernedFinalizable::new(address, client.clone()),
            core_contract_implementation: implementation_address,
        }
    }
}

impl AsRef<StarknetDevCoreContract<LocalWalletSignerMiddleware>> for StarknetDevCoreContractClient {
    fn as_ref(&self) -> &StarknetDevCoreContract<LocalWalletSignerMiddleware> {
        &self.core_contract
    }
}
impl AsRef<StarknetMessaging<LocalWalletSignerMiddleware>> for StarknetDevCoreContractClient {
    fn as_ref(&self) -> &StarknetMessaging<LocalWalletSignerMiddleware> {
        &self.messaging
    }
}
impl AsRef<ProxySupport3_0_2<LocalWalletSignerMiddleware>> for StarknetDevCoreContractClient {
    fn as_ref(&self) -> &ProxySupport3_0_2<LocalWalletSignerMiddleware> {
        &self.proxy_support
    }
}
impl AsRef<Operator<LocalWalletSignerMiddleware>> for StarknetDevCoreContractClient {
    fn as_ref(&self) -> &Operator<LocalWalletSignerMiddleware> {
        &self.operator
    }
}
impl AsRef<StarknetGovernance<LocalWalletSignerMiddleware>> for StarknetDevCoreContractClient {
    fn as_ref(&self) -> &StarknetGovernance<LocalWalletSignerMiddleware> {
        &self.governance
    }
}
impl AsRef<GovernedFinalizable<LocalWalletSignerMiddleware>> for StarknetDevCoreContractClient {
    fn as_ref(&self) -> &GovernedFinalizable<LocalWalletSignerMiddleware> {
        &self.governed_finalizable
    }
}

impl StarknetContractClient for StarknetDevCoreContractClient {
    fn address(&self) -> ethers::addressbook::Address {
        self.core_contract.address()
    }
    fn implementation_address(&self) -> ethers::addressbook::Address {
        self.core_contract_implementation
    }
    fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.core_contract.client()
    }
}
