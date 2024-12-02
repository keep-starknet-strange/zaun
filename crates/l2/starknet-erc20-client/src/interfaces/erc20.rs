use async_trait::async_trait;
use ethers::contract::ContractError;
use ethers::prelude::TransactionReceipt;
use ethers::{
    prelude::abigen,
    providers::Middleware,
    types::{Address, U256},
};

use utils::errors::Error;

abigen!(
    ERC20Token,
    "../../../artifacts/starkgate-contracts/ERC20.json",
);

#[async_trait]
pub trait ERC20TokenTrait<M: Middleware> {
    async fn name(&self) -> Result<String, Error<M>>;
    async fn symbol(&self) -> Result<String, Error<M>>;
    async fn total_supply(&self) -> Result<U256, Error<M>>;
    async fn balance_of(&self, address: Address) -> Result<U256, Error<M>>;
    async fn allowance(&self, owner: Address, spender: Address) -> Result<U256, Error<M>>;
    async fn approve(&self, address: Address, value: U256) -> Result<bool, Error<M>>;
}

#[async_trait]
impl<T, M: Middleware> ERC20TokenTrait<M> for T
where
    T: AsRef<ERC20Token<M>> + Send + Sync,
{
    async fn name(&self) -> Result<String, Error<M>> {
        self.as_ref().name().call().await.map_err(Into::into)
    }

    async fn symbol(&self) -> Result<String, Error<M>> {
        self.as_ref().symbol().call().await.map_err(Into::into)
    }

    async fn total_supply(&self) -> Result<U256, Error<M>> {
        self.as_ref()
            .total_supply()
            .call()
            .await
            .map_err(Into::into)
    }

    async fn balance_of(&self, address: Address) -> Result<U256, Error<M>> {
        self.as_ref()
            .balance_of(address)
            .call()
            .await
            .map_err(Into::into)
    }

    async fn allowance(&self, owner: Address, spender: Address) -> Result<U256, Error<M>> {
        self.as_ref()
            .allowance(owner, spender)
            .call()
            .await
            .map_err(Into::into)
    }

    async fn approve(&self, address: Address, value: U256) -> Result<bool, Error<M>> {
        let txn: Result<Option<TransactionReceipt>, Error<M>> = self
            .as_ref()
            .approve(address, value)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into);

        match txn {
            Ok(receipt) => {
                if receipt.is_some() {
                    return Ok(true);
                }
                Ok(false)
            }
            Err(err) => Err(err),
        }
    }
}
