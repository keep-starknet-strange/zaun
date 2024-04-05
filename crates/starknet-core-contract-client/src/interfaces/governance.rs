use std::sync::Arc;

use async_trait::async_trait;

use crate::{LocalWalletSignerMiddleware};

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
        self
            .starknet_nominate_new_governor(new_governor)
            .await
            .map_err(Into::into)
    }

    async fn starknet_remove_governor(
        &self,
        governor_for_removal: Address,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>> {
        self
            .starknet_remove_governor(governor_for_removal)
            .await
            .map_err(Into::into)
    }

    async fn starknet_accept_governance(&self) -> Result<TransactionReceipt, RpcError<TransportErrorKind>> {
        self
            .starknet_accept_governance()
            .await
            .map_err(Into::into)
    }

    async fn starknet_cancel_nomination(&self) -> Result<TransactionReceipt, RpcError<TransportErrorKind>> {
        self
            .starknet_cancel_nomination()
            .await
            .map_err(Into::into)
    }
}
