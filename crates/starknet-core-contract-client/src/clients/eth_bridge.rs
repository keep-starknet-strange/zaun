use std::sync::Arc;

use crate::{interfaces::{StarknetEthBridge, StarknetMessaging}, LocalWalletSignerMiddleware, StarknetLegacyBridgeContractClient};

use ethers::abi::Address;

/// Client to interact with a Starknet Eth Bridge
pub struct StarknetEthBridgeContractClient {
    messaging: StarknetMessaging<LocalWalletSignerMiddleware>,
    eth_bridge: StarknetEthBridge<LocalWalletSignerMiddleware>,
}

impl StarknetEthBridgeContractClient {
    pub fn new(messaging: Address, eth_bridge: Address, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self{
            messaging: StarknetMessaging::new(messaging, client.clone()),
            eth_bridge: StarknetEthBridge::new(eth_bridge, client.clone()),
        }
    }
}

impl AsRef<StarknetMessaging<LocalWalletSignerMiddleware>> for StarknetEthBridgeContractClient {
    fn as_ref(&self) -> &StarknetMessaging<LocalWalletSignerMiddleware> {
        &self.messaging
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

    fn messaging(&self) -> Address {
        self.messaging.address()
    }

    fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.eth_bridge.client()
    }
}