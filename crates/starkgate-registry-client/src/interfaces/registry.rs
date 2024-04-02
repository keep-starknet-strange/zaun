use async_trait::async_trait;
use ethers::{
    contract::ContractError,
    prelude::abigen,
    providers::Middleware,
    types::{TransactionReceipt, H160},
};

use utils::errors::Error;

type Address = H160;

abigen!(
    StarkgateRegistry,
    r#"[
        function enlistToken(address token, address bridge) external onlyManager
        function blockToken(address token) external onlyManager

        function selfRemove(address token) external

        function identify() external pure override returns (string memory)
        function getBridge(address token) external view returns (address)
    ]"#,
);

#[async_trait]
pub trait StarkgateRegistryTrait<M: Middleware> {
    async fn enlist_token(&self, token: Address, bridge: Address) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn block_token(&self, token: Address) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn self_remove(&self, token: Address) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn identify(&self) -> Result<String, Error<M>>;
    async fn get_bridge(&self, token: Address) -> Result<Address, Error<M>>;
}

#[async_trait]
impl<T, M: Middleware> StarkgateRegistryTrait<M> for T
where
    T: AsRef<StarkgateRegistry<M>> + Send + Sync,
{
    async fn enlist_token(&self, token: Address, bridge: Address) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .enlist_token(token, bridge)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn block_token(&self, token: Address) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .block_token(token)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn self_remove(&self, token: Address) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .self_remove(token)
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

    async fn get_bridge(&self, token: Address) -> Result<Address, Error<M>> {
        self.as_ref()
            .get_bridge(token)
            .call()
            .await
            .map_err(Into::into)
    }
}