use async_trait::async_trait;
use ethers::{
    contract::ContractError,
    prelude::abigen,
    providers::Middleware,
    types::{TransactionReceipt, H160, U256},
};

use utils::errors::Error;

type Address = H160;

abigen!(
    StarkgateManager,
    r#"[
        function addExistingBridge(address token, address bridge_) external onlyTokenAdmin
        function deactivateToken(address token) external onlyTokenAdmin
        function blockToken(address token) external onlyTokenAdmin

        function enrollTokenBridge(address token) external payable

        function getRegistry() external view returns (address)
        function identify() external pure override returns (string memory)
    ]"#,
);

#[async_trait]
pub trait StarkgateManagerTrait<M: Middleware> {
    async fn add_existing_bridge(&self, token: Address, bridge: Address) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn deactivate_token(&self, token: Address) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn block_token(&self, token: Address) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn enroll_token_bridge(&self, token: Address, fee: U256) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn get_registry(&self) -> Result<Address, Error<M>>;
    async fn identify(&self) -> Result<String, Error<M>>;
}

#[async_trait]
impl<T, M: Middleware> StarkgateManagerTrait<M> for T
where
    T: AsRef<StarkgateManager<M>> + Send + Sync,
{
    async fn add_existing_bridge(&self, token: Address, bridge: Address) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .add_existing_bridge(token, bridge)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn deactivate_token(&self, token: Address) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .deactivate_token(token)
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

    async fn enroll_token_bridge(&self, token: Address, fee: U256) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .enroll_token_bridge(token)
            .value(fee)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn get_registry(&self) -> Result<Address, Error<M>> {
        self.as_ref()
            .get_registry()
            .call()
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