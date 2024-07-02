use crate::clients::proxy::{ProxyInitializeData, ProxySupportTrait};
use async_trait::async_trait;
use ethers::addressbook::Address;
use ethers::contract::{abigen, ContractError};
use ethers::middleware::Middleware;
use ethers::prelude::{Bytes, TransactionReceipt};
use utils::errors::Error;

abigen!(
    ProxySupport,
    r#"[
        function isFrozen() external view virtual returns (bool)
        function initialize(bytes calldata data) external notCalledDirectly
        function upgradeTo(address newImplementation, bytes calldata data, bool finalize) external payable onlyGovernance notFinalized notFrozen
        function addImplementation(address newImplementation, bytes calldata data, bool finalize) external onlyGovernance
        function proxyNominateNewGovernor(address newGovernor) external
        function proxyRemoveGovernor(address governorForRemoval) external
        function proxyAcceptGovernance() external
    ]"#,
);

#[async_trait]
impl<T, M: Middleware> ProxySupportTrait<M> for T
where
    T: AsRef<ProxySupport<M>> + Send + Sync,
{
    async fn is_frozen(&self) -> Result<bool, Error<M>> {
        self.as_ref().is_frozen().call().await.map_err(Into::into)
    }

    async fn initialize(&self, data: Bytes) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .initialize(data)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn initialize_with<const N: usize>(
        &self,
        data: ProxyInitializeData<N>,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.initialize(data.into()).await
    }

    async fn upgrade_to(
        &self,
        data: Bytes,
        implementation_address: Address,
        finalized: bool,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .upgrade_to(implementation_address, data, finalized)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn add_implementation(
        &self,
        data: Bytes,
        implementation_address: Address,
        finalized: bool,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .add_implementation(implementation_address, data, finalized)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn proxy_nominate_new_governor(
        &self,
        new_governor: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .proxy_nominate_new_governor(new_governor)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn proxy_remove_governance(
        &self,
        governor: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .proxy_remove_governor(governor)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn proxy_accept_governance(&self) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .proxy_accept_governance()
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }
}
