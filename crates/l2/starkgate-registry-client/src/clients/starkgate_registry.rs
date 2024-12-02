use std::sync::Arc;

use crate::interfaces::registry::StarkgateRegistry;
use utils::{LocalWalletSignerMiddleware, StarknetContractClient};

use ethers::types::Address;
use starknet_proxy_client::clients::proxy_5_0_0::ProxySupport5_0_0;

#[derive(Clone)]
pub struct StarkgateRegistryContractClient {
    registry: StarkgateRegistry<LocalWalletSignerMiddleware>,
    proxy_support: ProxySupport5_0_0<LocalWalletSignerMiddleware>,
    registry_implementation: Address,
}

impl StarkgateRegistryContractClient {
    pub fn new(
        address: Address,
        client: Arc<LocalWalletSignerMiddleware>,
        implementation_address: Address,
    ) -> Self {
        Self {
            registry: StarkgateRegistry::new(address, client.clone()),
            proxy_support: ProxySupport5_0_0::new(address, client.clone()),
            registry_implementation: implementation_address,
        }
    }
}
impl AsRef<StarkgateRegistry<LocalWalletSignerMiddleware>> for StarkgateRegistryContractClient {
    fn as_ref(&self) -> &StarkgateRegistry<LocalWalletSignerMiddleware> {
        &self.registry
    }
}

impl AsRef<ProxySupport5_0_0<LocalWalletSignerMiddleware>> for StarkgateRegistryContractClient {
    fn as_ref(&self) -> &ProxySupport5_0_0<LocalWalletSignerMiddleware> {
        &self.proxy_support
    }
}

impl StarknetContractClient for StarkgateRegistryContractClient {
    fn address(&self) -> Address {
        self.registry.address()
    }
    fn implementation_address(&self) -> Address {
        self.registry_implementation
    }
    fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.registry.client()
    }
}
