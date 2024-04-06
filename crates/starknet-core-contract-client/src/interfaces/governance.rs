use std::sync::Arc;

use async_trait::async_trait;

use crate::LocalWalletSignerMiddleware;

use alloy::{
    contract::Error, network::Ethereum, primitives::Address, providers::Provider, rpc::types::eth::TransactionReceipt, sol, transports::{http::Http, RpcError, TransportErrorKind}
};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    interface StarknetGovernance {
        function starknetIsGovernor(address user) external view returns (bool);
        function starknetNominateNewGovernor(address newGovernor) external;
        function starknetRemoveGovernor(address governorForRemoval) external;
        function starknetAcceptGovernance() external;
        function starknetCancelNomination() external;
    }
);

#[async_trait]
pub trait StarknetGovernanceTrait {
    async fn starknet_is_governor(&self, user: Address) -> Result<bool, Error>;
    async fn starknet_nominate_new_governor(
        &self,
        new_governor: Address,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>>;
    async fn starknet_remove_governor(
        &self,
        governor_for_removal: Address,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>>;
    async fn starknet_accept_governance(&self) -> Result<TransactionReceipt, RpcError<TransportErrorKind>>;
    async fn starknet_cancel_nomination(&self) -> Result<TransactionReceipt, RpcError<TransportErrorKind>>;
}

#[async_trait]
impl<T> StarknetGovernanceTrait for T
where
    T: AsRef<StarknetGovernance::StarknetGovernanceInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>> + Send + Sync,
{
    async fn starknet_is_governor(&self, user: Address) -> Result<bool, Error> {        
        Ok(self.as_ref().starknetIsGovernor(user).call().await?._0)
    }

    async fn starknet_nominate_new_governor(
        &self,
        new_governor: Address,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>> {
        let base_fee = self.as_ref().provider().as_ref().get_gas_price().await.unwrap();
        let builder = self.as_ref().starknetNominateNewGovernor(new_governor);
        let gas = builder.estimate_gas().await.unwrap();
        builder
            .from(self.as_ref().provider().as_ref().get_accounts().await.unwrap()[0])
            .nonce(2)
            .gas(gas)
            .gas_price(base_fee)
            .send()
            .await.unwrap()
            .get_receipt()
            .await
    }

    async fn starknet_remove_governor(
        &self,
        governor_for_removal: Address,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>> {
        let base_fee = self.as_ref().provider().as_ref().get_gas_price().await.unwrap();
        let builder = self.as_ref().starknetRemoveGovernor(governor_for_removal);
        let gas = builder.estimate_gas().await.unwrap();
        builder
            .from(self.as_ref().provider().as_ref().get_accounts().await.unwrap()[0])
            .nonce(2)
            .gas(gas)
            .gas_price(base_fee)
            .send()
            .await.unwrap()
            .get_receipt()
            .await
    }

    async fn starknet_accept_governance(&self) -> Result<TransactionReceipt, RpcError<TransportErrorKind>> {
        let base_fee = self.as_ref().provider().as_ref().get_gas_price().await.unwrap();
        let builder = self.as_ref().starknetAcceptGovernance();
        let gas = builder.estimate_gas().await.unwrap();
        builder
            .from(self.as_ref().provider().as_ref().get_accounts().await.unwrap()[0])
            .nonce(2)
            .gas(gas)
            .gas_price(base_fee)
            .send()
            .await.unwrap()
            .get_receipt()
            .await
    }

    async fn starknet_cancel_nomination(&self) -> Result<TransactionReceipt, RpcError<TransportErrorKind>> {
        let base_fee = self.as_ref().provider().as_ref().get_gas_price().await.unwrap();
        let builder = self.as_ref().starknetCancelNomination();
        let gas = builder.estimate_gas().await.unwrap();
        builder
            .from(self.as_ref().provider().as_ref().get_accounts().await.unwrap()[0])
            .nonce(2)
            .gas(gas)
            .gas_price(base_fee)
            .send()
            .await.unwrap()
            .get_receipt()
            .await
    }
}
