use std::sync::Arc;

use crate::{
    interfaces::{StarknetEthBridge, StarknetMessaging}, LocalWalletSignerMiddleware,
};

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

