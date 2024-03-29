use async_trait::async_trait;

use crate::Error;

use alloy::{
    primitives::Address,
    network::Ethereum,
    providers::Provider,
    rpc::types::eth::TransactionReceipt,
    sol,
    sol_types::ContractError,
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
        self.as_ref()
            .starknet_is_governor(user)
            .call()
            .await
            .map_err(Into::into)
    }

    async fn starknet_nominate_new_governor(
        &self,
        new_governor: Address,
    ) -> Result<Option<TransactionReceipt>, Error<P>> {
        self.as_ref()
            .starknet_nominate_new_governor(new_governor)
            .send()
            .await
            .map_err(Into::<ContractError<P>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn starknet_remove_governor(
        &self,
        governor_for_removal: Address,
    ) -> Result<Option<TransactionReceipt>, Error<P>> {
        self.as_ref()
            .starknet_remove_governor(governor_for_removal)
            .send()
            .await
            .map_err(Into::<ContractError<P>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn starknet_accept_governance(&self) -> Result<Option<TransactionReceipt>, Error<P>> {
        self.as_ref()
            .starknet_accept_governance()
            .send()
            .await
            .map_err(Into::<ContractError<P>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn starknet_cancel_nomination(&self) -> Result<Option<TransactionReceipt>, Error<P>> {
        self.as_ref()
            .starknet_cancel_nomination()
            .send()
            .await
            .map_err(Into::<ContractError<P>>::into)?
            .await
            .map_err(Into::into)
    }
}
