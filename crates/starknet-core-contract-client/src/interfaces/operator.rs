use std::sync::Arc;

use async_trait::async_trait;

use crate::LocalWalletSignerMiddleware;

use alloy::{
    contract::Error, network::Ethereum, primitives::Address, providers::Provider, rpc::types::eth::TransactionReceipt, sol, transports::{http::Http, RpcError, TransportErrorKind}
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
pub trait OperatorTrait {
    async fn register_operator(
        &self,
        new_operator: Address,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>>;
    async fn unregister_operator(
        &self,
        removed_operator: Address,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>>;
    async fn is_operator(&self, user: Address) -> Result<bool, Error>;
}
#[async_trait]
impl<T> OperatorTrait for T
where
    T: AsRef<Operator::OperatorInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>> + Send + Sync,
{
    async fn register_operator(
        &self,
        new_operator: Address,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>> {
        let base_fee = self.as_ref().provider().as_ref().get_gas_price().await.unwrap();
        let from_address = self.as_ref().provider().as_ref().get_accounts().await.unwrap()[0];
        let gas = self.as_ref().registerOperator(new_operator).from(from_address).estimate_gas().await;
        let builder = self.as_ref().registerOperator(new_operator);
        builder
            .from(from_address)
            .nonce(3)
            .gas(gas.unwrap())
            .gas_price(base_fee)
            .send()
            .await.unwrap()
            .get_receipt()
            .await
    }
    async fn unregister_operator(
        &self,
        removed_operator: Address,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>> {
        let base_fee = self.as_ref().provider().as_ref().get_gas_price().await.unwrap();
        let from_address = self.as_ref().provider().as_ref().get_accounts().await.unwrap()[0];
        let gas = self.as_ref().unregisterOperator(removed_operator).from(from_address).estimate_gas().await;
        let builder = self.as_ref().unregisterOperator(removed_operator);
        builder
            .from(from_address)
            .nonce(3)
            .gas(gas.unwrap())
            .gas_price(base_fee)
            .send()
            .await.unwrap()
            .get_receipt()
            .await
    }
    async fn is_operator(&self, user: Address) -> Result<bool, Error> {
        Ok(self.as_ref().isOperator(user).call().await?._0)
    }
}
