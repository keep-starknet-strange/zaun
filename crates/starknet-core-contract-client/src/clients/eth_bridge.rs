use std::sync::Arc;

use crate::{interfaces::StarknetEthBridge, LocalWalletSignerMiddleware, StarknetLegacyBridgeContractClient};

use ethers::abi::Address;

/// Client to interact with a Starknet Eth Bridge
pub struct StarknetEthBridgeContractClient {
    eth_bridge: StarknetEthBridge<LocalWalletSignerMiddleware>,
}

impl StarknetEthBridgeContractClient {
    pub fn new(eth_bridge: Address, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self{
            eth_bridge: StarknetEthBridge::new(eth_bridge, client.clone()),
        }
    }
}

impl AsRef<StarknetEthBridge<LocalWalletSignerMiddleware>> for StarknetEthBridgeContractClient {
    fn as_ref(&self) -> &StarknetEthBridge<LocalWalletSignerMiddleware> {
        &self.eth_bridge
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