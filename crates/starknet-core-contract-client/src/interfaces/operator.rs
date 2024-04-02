use async_trait::async_trait;
use ethers::{
    contract::ContractError,
    prelude::abigen,
    providers::Middleware,
    types::{Address, TransactionReceipt},
};

use utils::errors::Error;

abigen!(
    Operator,
    r#"[
        function registerOperator(address newOperator) external override onlyGovernance
        function unregisterOperator(address removedOperator) external override onlyGovernance

        function isOperator(address user) public view override returns (bool)
    ]"#,
);

#[async_trait]
pub trait OperatorTrait<M: Middleware> {
    async fn register_operator(
        &self,
        new_operator: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn unregister_operator(
        &self,
        removed_operator: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn is_operator(&self, user: Address) -> Result<bool, Error<M>>;
}
#[async_trait]
impl<T, M: Middleware> OperatorTrait<M> for T
where
    T: AsRef<Operator<M>> + Send + Sync,
{
    async fn register_operator(
        &self,
        new_operator: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .register_operator(new_operator)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }
    async fn unregister_operator(
        &self,
        removed_operator: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .unregister_operator(removed_operator)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }
    async fn is_operator(&self, user: Address) -> Result<bool, Error<M>> {
        self.as_ref()
            .is_operator(user)
            .call()
            .await
            .map_err(Into::into)
    }
}
