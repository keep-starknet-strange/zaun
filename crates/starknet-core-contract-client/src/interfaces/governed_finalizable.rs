use async_trait::async_trait;

use crate::Error;

use alloy::{
    network::Ethereum,
    providers::Provider,
    rpc::types::eth::TransactionReceipt,
    sol,
    sol_types::ContractError,
};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    interface GovernedFinalizable {
        function isFinalized() public view returns (bool);
        function finalize() external onlyGovernance notFinalized;
    }
);

#[async_trait]
pub trait GovernedFinalizableTrait<P: Provider<Ethereum>> {
    async fn is_finalized(&self) -> Result<bool, Error<P>>;
    async fn finalize(&self) -> Result<Option<TransactionReceipt>, Error<P>>;
}

#[async_trait]
impl<T, P: Provider<Ethereum>> GovernedFinalizableTrait<P> for T
where
    T: AsRef<GovernedFinalizable::GovernedFinalizableInstance<Ethereum, T, P>> + Send + Sync,
{
    async fn is_finalized(&self) -> Result<bool, Error<P>> {
        self.as_ref()
            .is_finalized()
            .call()
            .await
            .map_err(Into::into)
    }

    async fn finalize(&self) -> Result<Option<TransactionReceipt>, Error<P>> {
        self.as_ref()
            .finalize()
            .send()
            .await
            .map_err(Into::<ContractError<P>>::into)?
            .await
            .map_err(Into::into)
    }
}
