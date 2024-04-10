use std::sync::Arc;

use starknet_proxy_client::proxy_support::ProxySupport;
use utils::{ LocalWalletSignerMiddleware, StarknetContractClient };
use crate::interfaces::token_bridge::StarknetTokenBridge;

use ethers::types::Address;

/// Client to interact with a Token Bridge (ERC20)
pub struct StarknetTokenBridgeContractClient {
    token_bridge: StarknetTokenBridge<LocalWalletSignerMiddleware>,
    proxy_support: ProxySupport<LocalWalletSignerMiddleware>,
}

impl StarknetTokenBridgeContractClient {
    pub fn new(address: Address, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self {
            token_bridge: StarknetTokenBridge::new(address, client.clone()),
            proxy_support: ProxySupport::new(address, client.clone()),
        }
    }
}

impl AsRef<StarknetTokenBridge<LocalWalletSignerMiddleware>> for StarknetTokenBridgeContractClient {
    fn as_ref(&self) -> &StarknetTokenBridge<LocalWalletSignerMiddleware> {
        &self.token_bridge
    }
}

impl AsRef<ProxySupport<LocalWalletSignerMiddleware>> for StarknetTokenBridgeContractClient {
    fn as_ref(&self) -> &ProxySupport<LocalWalletSignerMiddleware> {
        &self.proxy_support
    }
}

impl StarknetContractClient for StarknetTokenBridgeContractClient {
    fn address(&self) -> ethers::abi::Address {
        self.token_bridge.address()
    }

    fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.token_bridge.client()
    }
}