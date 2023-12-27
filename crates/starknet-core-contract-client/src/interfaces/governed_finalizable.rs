use async_trait::async_trait;
use ethers::{prelude::abigen, providers::Middleware};

use crate::Error;

abigen!(
    GovernedFinalizable,
    r#"[
        function isFinalized() public view returns (bool)
        function finalize() external onlyGovernance notFinalized
    ]"#,
);

#[async_trait]
pub trait GovernedFinalizableTrait<M: Middleware> {
    async fn is_finalized(&self) -> Result<bool, Error<M>>;
    async fn finalize(&self) -> Result<(), Error<M>>;
}

#[async_trait]
impl<T, M: Middleware> GovernedFinalizableTrait<M> for T
where
    T: AsRef<GovernedFinalizable<M>> + Send + Sync,
{
    async fn is_finalized(&self) -> Result<bool, Error<M>> {
        self.as_ref()
        .is_finalized()
        .call()
        .await
        .map_err(Into::into)
    }

    async fn finalize(&self) -> Result<(), Error<M>> {
        self.as_ref()
        .finalize()
        .call()
        .await
        .map_err(Into::into)
    }
}
