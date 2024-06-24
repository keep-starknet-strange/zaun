use std::sync::Arc;

use crate::interfaces::eth_bridge::StarknetEthBridge;
use starknet_proxy_client::proxy_support::ProxySupport;
use utils::{LocalWalletSignerMiddleware, StarknetContractClient};

use ethers::types::Address;

/// Client to interact with a Starknet Eth Bridge
pub struct StarknetEthBridgeContractClient {
    eth_bridge: StarknetEthBridge<LocalWalletSignerMiddleware>,
    proxy_support: ProxySupport<LocalWalletSignerMiddleware>,
    eth_bridge_implementation: StarknetEthBridge<LocalWalletSignerMiddleware>,
}

impl StarknetEthBridgeContractClient {
    pub fn new(
        address: Address,
        client: Arc<LocalWalletSignerMiddleware>,
        implementation_address: Address,
    ) -> Self {
        Self {
            eth_bridge: StarknetEthBridge::new(address, client.clone()),
            proxy_support: ProxySupport::new(address, client.clone()),
            eth_bridge_implementation: StarknetEthBridge::new(
                implementation_address,
                client.clone(),
            ),
        }
    }
}

impl AsRef<StarknetEthBridge<LocalWalletSignerMiddleware>> for StarknetEthBridgeContractClient {
    fn as_ref(&self) -> &StarknetEthBridge<LocalWalletSignerMiddleware> {
        &self.eth_bridge
    }
}

impl AsRef<ProxySupport<LocalWalletSignerMiddleware>> for StarknetEthBridgeContractClient {
    fn as_ref(&self) -> &ProxySupport<LocalWalletSignerMiddleware> {
        &self.proxy_support
    }
}

impl StarknetContractClient for StarknetEthBridgeContractClient {
    fn address(&self) -> Address {
        self.eth_bridge.address()
    }
    fn implementation_address(&self) -> Address {
        self.eth_bridge_implementation.address()
    }
    fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.eth_bridge.client()
    }
}
