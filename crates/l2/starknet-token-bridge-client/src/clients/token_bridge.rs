use std::sync::Arc;

use crate::interfaces::token_bridge::StarknetTokenBridge;
use utils::{LocalWalletSignerMiddleware, StarknetContractClient};

use ethers::types::Address;
use starknet_proxy_client::clients::proxy_5_0_0::ProxySupport5_0_0;

/// Client to interact with a Token Bridge (ERC20)
#[derive(Clone)]
pub struct StarknetTokenBridgeContractClient {
    token_bridge: StarknetTokenBridge<LocalWalletSignerMiddleware>,
    proxy_support: ProxySupport5_0_0<LocalWalletSignerMiddleware>,
    token_bridge_implementation: Address,
}

impl StarknetTokenBridgeContractClient {
    pub fn new(
        address: Address,
        client: Arc<LocalWalletSignerMiddleware>,
        implementation_address: Address,
    ) -> Self {
        Self {
            token_bridge: StarknetTokenBridge::new(address, client.clone()),
            proxy_support: ProxySupport5_0_0::new(address, client.clone()),
            token_bridge_implementation: implementation_address,
        }
    }
}

impl AsRef<StarknetTokenBridge<LocalWalletSignerMiddleware>> for StarknetTokenBridgeContractClient {
    fn as_ref(&self) -> &StarknetTokenBridge<LocalWalletSignerMiddleware> {
        &self.token_bridge
    }
}

impl AsRef<ProxySupport5_0_0<LocalWalletSignerMiddleware>> for StarknetTokenBridgeContractClient {
    fn as_ref(&self) -> &ProxySupport5_0_0<LocalWalletSignerMiddleware> {
        &self.proxy_support
    }
}

impl StarknetContractClient for StarknetTokenBridgeContractClient {
    fn address(&self) -> ethers::abi::Address {
        self.token_bridge.address()
    }

    fn implementation_address(&self) -> Address {
        self.token_bridge_implementation
    }

    fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.token_bridge.client()
    }
}
