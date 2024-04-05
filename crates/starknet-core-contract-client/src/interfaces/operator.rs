use std::sync::Arc;

use async_trait::async_trait;

use crate::{LocalWalletSignerMiddleware};

use alloy::{
    network::Ethereum,
    primitives::{Address, U256},
    providers::Provider,
    rpc::types::eth::TransactionReceipt,
    sol, transports::{http::Http, RpcError, TransportErrorKind},
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
    async fn is_operator(&self, user: Address) -> bool;
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
        log::debug!("check ocntract address - {:?}", self.as_ref().address());
        let base_fee = self.as_ref().provider().as_ref().get_gas_price().await.unwrap();
        let initialize_builder = self.as_ref().registerOperator(new_operator);
        let initialize_gas = initialize_builder.estimate_gas().await.unwrap();
        initialize_builder
            .from(self.as_ref().provider().as_ref().get_accounts().await.unwrap()[0])
            .nonce(3)
            .gas(initialize_gas)
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
        self
            .unregister_operator(removed_operator)
            .await
            .map_err(Into::into)
    }
    async fn is_operator(&self, user: Address) -> bool {
        self
            .is_operator(user)
            .await
    }
}
