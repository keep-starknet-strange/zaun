use std::sync::Arc;

use async_trait::async_trait;

use crate::{LocalWalletSignerMiddleware};

use alloy::{
    network::Ethereum,
    providers::Provider,
    rpc::types::eth::TransactionReceipt,
    sol, transports::http::Http,
    contract::Error
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
pub trait GovernedFinalizableTrait {
    async fn is_finalized(&self) -> Result<bool, Error>;
    async fn finalize(&self) -> Result<Option<TransactionReceipt>, Error>;
}

#[async_trait]
impl<T> GovernedFinalizableTrait for T
where
    T: AsRef<GovernedFinalizable::GovernedFinalizableInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>> + Send + Sync,
{
    async fn is_finalized(&self) -> Result<bool, Error> {
        self
            .is_finalized()
            .await
            .map_err(Into::into)
    }

    async fn finalize(&self) -> Result<Option<TransactionReceipt>, Error> {
        self
            .finalize()
            .await
            .map_err(Into::into)
    }
}
