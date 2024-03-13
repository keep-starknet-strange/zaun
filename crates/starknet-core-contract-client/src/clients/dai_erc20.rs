use std::sync::Arc;

use crate::{interfaces::{
    DaiERC20Token,
}, LocalWalletSignerMiddleware, StarknetContractClient};

use ethers::types::Address;

pub struct DaiERC20ContractClient {
    erc20_token: DaiERC20Token<LocalWalletSignerMiddleware>,
}

impl DaiERC20ContractClient {
    pub fn new(address: Address, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self {
            erc20_token: DaiERC20Token::new(address, client.clone()),
        }
    }
}

impl AsRef<DaiERC20Token<LocalWalletSignerMiddleware>> for DaiERC20ContractClient {
    fn as_ref(&self) -> &DaiERC20Token<LocalWalletSignerMiddleware> {
        &self.erc20_token
    }
}

impl StarknetContractClient for DaiERC20ContractClient {
    fn address(&self) -> Address {
        self.erc20_token.address()
    }

    fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.erc20_token.client()
    }
}
