use async_trait::async_trait;
use ethers::{
    contract::ContractError,
    prelude::abigen,
    providers::Middleware,
    types::{TransactionReceipt, U256},
};

use crate::Error;

abigen!(
    StarknetEthBridge,
    r#"[
        function deposit(uint256 amount, uint256 l2Recipient) public payable override

        function identify() external pure override returns (string memory)
    ]"#,
);

#[async_trait]
pub trait StarknetEthBridgeTrait<M: Middleware> {
    async fn deposit(&self, amount: U256, l2recipient: U256) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn identify(&self) -> Result<String, Error<M>>;
}

#[async_trait]
impl<T, M: Middleware> StarknetEthBridgeTrait<M> for T
    where
        T: AsRef<StarknetEthBridge<M>> + Send + Sync,
{
    async fn deposit(&self, amount: U256, l2recipient: U256) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .deposit(amount, l2recipient)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)

    }

    async fn identify(&self) -> Result<String, Error<M>> {
        self.as_ref()
            .identify()
            .call()
            .await
            .map_err(Into::into)
    }
}
