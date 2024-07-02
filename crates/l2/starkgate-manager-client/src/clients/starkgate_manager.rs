use std::sync::Arc;

use crate::interfaces::manager::StarkgateManager;
use utils::{LocalWalletSignerMiddleware, StarknetContractClient};

use ethers::types::Address;
use starknet_proxy_client::clients::proxy_5_0_0::ProxySupportLatest;

pub struct StarkgateManagerContractClient {
    manager: StarkgateManager<LocalWalletSignerMiddleware>,
    proxy_support: ProxySupportLatest<LocalWalletSignerMiddleware>,
    manager_implementation: Address,
}

impl StarkgateManagerContractClient {
    pub fn new(
        address: Address,
        client: Arc<LocalWalletSignerMiddleware>,
        implementation_address: Address,
    ) -> Self {
        Self {
            manager: StarkgateManager::new(address, client.clone()),
            proxy_support: ProxySupportLatest::new(address, client.clone()),
            manager_implementation: implementation_address,
        }
    }
}

impl AsRef<StarkgateManager<LocalWalletSignerMiddleware>> for StarkgateManagerContractClient {
    fn as_ref(&self) -> &StarkgateManager<LocalWalletSignerMiddleware> {
        &self.manager
    }
}

impl AsRef<ProxySupportLatest<LocalWalletSignerMiddleware>> for StarkgateManagerContractClient {
    fn as_ref(&self) -> &ProxySupportLatest<LocalWalletSignerMiddleware> {
        &self.proxy_support
    }
}

impl StarknetContractClient for StarkgateManagerContractClient {
    fn address(&self) -> Address {
        self.manager.address()
    }
    fn implementation_address(&self) -> Address {
        self.manager_implementation
    }
    fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.manager.client()
    }
}
