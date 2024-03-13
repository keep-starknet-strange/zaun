use std::sync::Arc;

use crate::{interfaces::{
    StarkgateRegistry, ProxySupport
}, LocalWalletSignerMiddleware, StarknetContractClient};

use ethers::types::Address;

pub struct StarkgateRegistryContractClient {
    registry: StarkgateRegistry<LocalWalletSignerMiddleware>,
    proxy_support: ProxySupport<LocalWalletSignerMiddleware>,
}

impl StarkgateRegistryContractClient {
    pub fn new(address: Address, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self {
            registry: StarkgateRegistry::new(address, client.clone()),
            proxy_support: ProxySupport::new(address, client.clone()),
        }
    }
}
impl AsRef<StarkgateRegistry<LocalWalletSignerMiddleware>> for StarkgateRegistryContractClient {
    fn as_ref(&self) -> &StarkgateRegistry<LocalWalletSignerMiddleware> {
        &self.registry
    }
}

impl AsRef<ProxySupport<LocalWalletSignerMiddleware>> for StarkgateRegistryContractClient {
    fn as_ref(&self) -> &ProxySupport<LocalWalletSignerMiddleware> {
        &self.proxy_support
    }
}

impl StarknetContractClient for StarkgateRegistryContractClient {
    fn address(&self) -> Address {
        self.registry.address()
    }

    fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.registry.client()
    }
}