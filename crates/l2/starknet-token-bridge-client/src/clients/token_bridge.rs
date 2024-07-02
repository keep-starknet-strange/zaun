use std::sync::Arc;

use crate::interfaces::token_bridge::StarknetTokenBridge;
use utils::{LocalWalletSignerMiddleware, StarknetContractClient};

use ethers::types::Address;
use starknet_proxy_client::clients::proxy_5_0_0::ProxySupportLatest;

/// Client to interact with a Token Bridge (ERC20)
pub struct StarknetTokenBridgeContractClient {
    token_bridge: StarknetTokenBridge<LocalWalletSignerMiddleware>,
    proxy_support: ProxySupportLatest<LocalWalletSignerMiddleware>,
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
            proxy_support: ProxySupportLatest::new(address, client.clone()),
            token_bridge_implementation: implementation_address,
        }
    }
}

impl AsRef<StarknetTokenBridge<LocalWalletSignerMiddleware>> for StarknetTokenBridgeContractClient {
    fn as_ref(&self) -> &StarknetTokenBridge<LocalWalletSignerMiddleware> {
        &self.token_bridge
    }
}

impl AsRef<ProxySupportLatest<LocalWalletSignerMiddleware>> for StarknetTokenBridgeContractClient {
    fn as_ref(&self) -> &ProxySupportLatest<LocalWalletSignerMiddleware> {
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
