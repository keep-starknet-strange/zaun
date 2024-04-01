use std::sync::Arc;

use async_trait::async_trait;

use crate::{Error, LocalWalletSignerMiddleware};

use alloy::{
    network::Ethereum,
    providers::Provider,
    rpc::types::eth::TransactionReceipt,
    sol, transports::http::Http,
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
    T: AsRef<GovernedFinalizable::GovernedFinalizableInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>> + Send + Sync,
{
    async fn is_finalized(&self) -> Result<bool, Error<P>> {
        self
            .is_finalized()
            .await
            .map_err(Into::into)
    }

    async fn finalize(&self) -> Result<Option<TransactionReceipt>, Error<P>> {
        self
            .finalize()
            .await
            .map_err(Into::into)
    }
}
