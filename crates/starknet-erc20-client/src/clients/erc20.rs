use std::sync::Arc;

use utils::{ LocalWalletSignerMiddleware, StarknetContractClient };
use crate::interfaces::erc20::ERC20Token;

use ethers::types::Address;

pub struct ERC20ContractClient {
    erc20_token: ERC20Token<LocalWalletSignerMiddleware>,
}

impl ERC20ContractClient {
    pub fn new(address: Address, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self {
            erc20_token: ERC20Token::new(address, client.clone()),
        }
    }
}

impl AsRef<ERC20Token<LocalWalletSignerMiddleware>> for ERC20ContractClient {
    fn as_ref(&self) -> &ERC20Token<LocalWalletSignerMiddleware> {
        &self.erc20_token
    }
}

impl StarknetContractClient for ERC20ContractClient {
    fn address(&self) -> Address {
        self.erc20_token.address()
    }

    fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.erc20_token.client()
    }
}
