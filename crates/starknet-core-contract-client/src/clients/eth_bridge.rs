use std::sync::Arc;

use crate::{interfaces::StarknetEthBridge, LocalWalletSignerMiddleware, StarknetLegacyBridgeContractClient};

use ethers::abi::Address;
use crate::clients::StarknetSovereignContractClient;

/// Client to interact with a Starknet Eth Bridge
pub struct StarknetEthBridgeContractClient {
    eth_bridge: StarknetEthBridge<LocalWalletSignerMiddleware>,
    proxy_support: ProxySupport<LocalWalletSignerMiddleware>,
}

impl StarknetEthBridgeContractClient {
    pub fn new(eth_bridge: Address, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self{
            eth_bridge: StarknetEthBridge::new(eth_bridge, client.clone()),
            proxy_support: ProxySupport::new(address, client.clone()),
        }
    }
}

impl AsRef<StarknetEthBridge<LocalWalletSignerMiddleware>> for StarknetEthBridgeContractClient {
    fn as_ref(&self) -> &StarknetEthBridge<LocalWalletSignerMiddleware> {
        &self.eth_bridge
    }
}

impl AsRef<ProxySupport<LocalWalletSignerMiddleware>> for StarknetSovereignContractClient {
    fn as_ref(&self) -> &ProxySupport<LocalWalletSignerMiddleware> {
        &self.proxy_support
    }
}

impl StarknetLegacyBridgeContractClient for StarknetEthBridgeContractClient {
    fn address(&self) -> Address {
        self.eth_bridge.address()
    }

    fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.eth_bridge.client()
    }
}