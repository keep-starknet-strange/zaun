use std::sync::Arc;

use crate::interfaces::eth_bridge::StarknetEthBridge;
use utils::{LocalWalletSignerMiddleware, StarknetContractClient};

use ethers::types::Address;
use starknet_proxy_client::clients::proxy_3_0_2::ProxySupport3_0_2;

/// Client to interact with a Starknet Eth Bridge
pub struct StarknetEthBridgeContractClient {
    eth_bridge: StarknetEthBridge<LocalWalletSignerMiddleware>,
    proxy_support: ProxySupport3_0_2<LocalWalletSignerMiddleware>,
    eth_bridge_implementation: Address,
}

impl StarknetEthBridgeContractClient {
    pub fn new(
        address: Address,
        client: Arc<LocalWalletSignerMiddleware>,
        implementation_address: Address,
    ) -> Self {
        Self {
            eth_bridge: StarknetEthBridge::new(address, client.clone()),
            proxy_support: ProxySupport3_0_2::new(address, client.clone()),
            eth_bridge_implementation: implementation_address,
        }
    }
}

impl AsRef<StarknetEthBridge<LocalWalletSignerMiddleware>> for StarknetEthBridgeContractClient {
    fn as_ref(&self) -> &StarknetEthBridge<LocalWalletSignerMiddleware> {
        &self.eth_bridge
    }
}

impl AsRef<ProxySupport3_0_2<LocalWalletSignerMiddleware>> for StarknetEthBridgeContractClient {
    fn as_ref(&self) -> &ProxySupport3_0_2<LocalWalletSignerMiddleware> {
        &self.proxy_support
    }
}

impl StarknetContractClient for StarknetEthBridgeContractClient {
    fn address(&self) -> Address {
        self.eth_bridge.address()
    }
    fn implementation_address(&self) -> Address {
        self.eth_bridge_implementation
    }
    fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.eth_bridge.client()
    }
}
