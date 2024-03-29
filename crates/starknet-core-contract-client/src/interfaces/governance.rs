use async_trait::async_trait;

use crate::Error;

use alloy::{
    primitives::Address,
    network::Ethereum,
    providers::Provider,
    rpc::types::eth::TransactionReceipt,
    sol,
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
pub trait StarknetGovernanceTrait<P: Provider<Ethereum>> {
    async fn starknet_is_governor(&self, user: Address) -> Result<bool, Error<P>>;
    async fn starknet_nominate_new_governor(
        &self,
        new_governor: Address,
    ) -> Result<Option<TransactionReceipt>, Error<P>>;
    async fn starknet_remove_governor(
        &self,
        governor_for_removal: Address,
    ) -> Result<Option<TransactionReceipt>, Error<P>>;
    async fn starknet_accept_governance(&self) -> Result<Option<TransactionReceipt>, Error<P>>;
    async fn starknet_cancel_nomination(&self) -> Result<Option<TransactionReceipt>, Error<P>>;
}

#[async_trait]
impl<T, P: Provider<Ethereum>> StarknetGovernanceTrait<P> for T
where
    T: AsRef<StarknetGovernance::StarknetGovernanceInstance<Ethereum, T, P>> + Send + Sync,
{
    async fn starknet_is_governor(&self, user: Address) -> Result<bool, Error<P>> {        
        self.starknet_is_governor(user)
            .await
            .map_err(Into::into)
    }

    async fn starknet_nominate_new_governor(
        &self,
        new_governor: Address,
    ) -> Result<Option<TransactionReceipt>, Error<P>> {
        self
            .starknet_nominate_new_governor(new_governor)
            .await
            .map_err(Into::into)
    }

    async fn starknet_remove_governor(
        &self,
        governor_for_removal: Address,
    ) -> Result<Option<TransactionReceipt>, Error<P>> {
        self
            .starknet_remove_governor(governor_for_removal)
            .await
            .map_err(Into::into)
    }

    async fn starknet_accept_governance(&self) -> Result<Option<TransactionReceipt>, Error<P>> {
        self
            .starknet_accept_governance()
            .await
            .map_err(Into::into)
    }

    async fn starknet_cancel_nomination(&self) -> Result<Option<TransactionReceipt>, Error<P>> {
        self
            .starknet_cancel_nomination()
            .await
            .map_err(Into::into)
    }
}
