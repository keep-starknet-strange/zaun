use async_trait::async_trait;
use ethers::{
    contract::ContractError,
    prelude::abigen,
    providers::Middleware,
    types::{TransactionReceipt, H160},
};

use utils::errors::Error;

type Address = H160;

abigen!(
    StarknetGovernance,
    r#"[
        function starknetIsGovernor(address user) external view returns (bool)
        function starknetNominateNewGovernor(address newGovernor) external
        function starknetRemoveGovernor(address governorForRemoval) external
        function starknetAcceptGovernance() external
        function starknetCancelNomination() external
    ]"#,
);

#[async_trait]
pub trait StarknetGovernanceTrait<M: Middleware> {
    async fn starknet_is_governor(&self, user: Address) -> Result<bool, Error<M>>;
    async fn starknet_nominate_new_governor(
        &self,
        new_governor: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn starknet_remove_governor(
        &self,
        governor_for_removal: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn starknet_accept_governance(&self) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn starknet_cancel_nomination(&self) -> Result<Option<TransactionReceipt>, Error<M>>;
}

#[async_trait]
impl<T, M: Middleware> StarknetGovernanceTrait<M> for T
where
    T: AsRef<StarknetGovernance<M>> + Send + Sync,
{
    async fn starknet_is_governor(&self, user: Address) -> Result<bool, Error<M>> {
        self.as_ref()
            .starknet_is_governor(user)
            .call()
            .await
            .map_err(Into::into)
    }

    async fn starknet_nominate_new_governor(
        &self,
        new_governor: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .starknet_nominate_new_governor(new_governor)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn starknet_remove_governor(
        &self,
        governor_for_removal: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .starknet_remove_governor(governor_for_removal)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn starknet_accept_governance(&self) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .starknet_accept_governance()
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn starknet_cancel_nomination(&self) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .starknet_cancel_nomination()
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }
}
