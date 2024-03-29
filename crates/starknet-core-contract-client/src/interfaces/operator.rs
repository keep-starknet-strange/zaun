use async_trait::async_trait;

use crate::Error;

use alloy::{
    network::Ethereum,
    primitives::Address,
    providers::Provider,
    rpc::types::eth::TransactionReceipt,
    sol,
    sol_types::ContractError,
};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    interface Operator {
        function registerOperator(address newOperator) external override onlyGovernance;
        function unregisterOperator(address removedOperator) external override onlyGovernance;

        function isOperator(address user) public view override returns (bool);
    }
);

#[async_trait]
pub trait OperatorTrait<P: Provider<Ethereum>> {
    async fn register_operator(
        &self,
        new_operator: Address,
    ) -> Result<Option<TransactionReceipt>, Error<P>>;
    async fn unregister_operator(
        &self,
        removed_operator: Address,
    ) -> Result<Option<TransactionReceipt>, Error<P>>;
    async fn is_operator(&self, user: Address) -> Result<bool, Error<P>>;
}
#[async_trait]
impl<T, P: Provider<Ethereum>> OperatorTrait<P> for T
where
    T: AsRef<Operator::OperatorInstance<Ethereum, T, P>> + Send + Sync,
{
    async fn register_operator(
        &self,
        new_operator: Address,
    ) -> Result<Option<TransactionReceipt>, Error<P>> {
        self
            .register_operator(new_operator)
            .await
            .map_err(Into::into)
    }
    async fn unregister_operator(
        &self,
        removed_operator: Address,
    ) -> Result<Option<TransactionReceipt>, Error<P>> {
        self
            .unregister_operator(removed_operator)
            .await
            .map_err(Into::into)
    }
    async fn is_operator(&self, user: Address) -> Result<bool, Error<P>> {
        self
            .is_operator(user)
            .await
            .map_err(Into::into)
    }
}
