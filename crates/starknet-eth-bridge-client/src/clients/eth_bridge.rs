use std::sync::Arc;

use starknet_proxy_client::proxy_support::ProxySupport;
use utils::{ LocalWalletSignerMiddleware, StarknetContractClient };
use crate::interfaces::eth_bridge::StarknetEthBridge;

use ethers::types::Address;

/// Client to interact with a Starknet Eth Bridge
pub struct StarknetEthBridgeContractClient {
    eth_bridge: StarknetEthBridge<LocalWalletSignerMiddleware>,
    proxy_support: ProxySupport<LocalWalletSignerMiddleware>,
}

impl StarknetEthBridgeContractClient {
    pub fn new(address: Address, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self{
            eth_bridge: StarknetEthBridge::new(address, client.clone()),
            proxy_support: ProxySupport::new(address, client.clone()),
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

    fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.eth_bridge.client()
    }
}