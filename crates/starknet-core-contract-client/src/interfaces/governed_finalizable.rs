use std::sync::Arc;

use async_trait::async_trait;

use crate::LocalWalletSignerMiddleware;

use alloy::{
    contract::Error, network::Ethereum, providers::Provider, rpc::types::eth::TransactionReceipt, sol, transports::{http::Http, RpcError, TransportErrorKind}
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
    async fn finalize(&self) -> Result<TransactionReceipt, RpcError<TransportErrorKind>>;
}

#[async_trait]
impl<T> GovernedFinalizableTrait for T
where
    T: AsRef<GovernedFinalizable::GovernedFinalizableInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>> + Send + Sync,
{
    async fn is_finalized(&self) -> Result<bool, Error> {
        Ok(self.as_ref().isFinalized().call().await?._0)
    }

    async fn finalize(&self) -> Result<TransactionReceipt, RpcError<TransportErrorKind>> {
        let base_fee = self.as_ref().provider().as_ref().get_gas_price().await.unwrap();
        let from_address = self.as_ref().provider().as_ref().get_accounts().await.unwrap()[0];
        let gas = self.as_ref().finalize().from(from_address).estimate_gas().await.unwrap();
        let builder = self.as_ref().finalize();
        builder
            .from(from_address)
            .nonce(2)
            .gas(gas)
            .gas_price(base_fee)
            .send()
            .await.unwrap()
            .get_receipt()
            .await
    }
}
