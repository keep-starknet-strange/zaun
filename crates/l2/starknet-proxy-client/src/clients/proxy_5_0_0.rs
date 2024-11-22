use crate::interfaces::proxy::{ProxyInitializeData, ProxySupport5_0_0Trait};
use async_trait::async_trait;
use ethers::addressbook::Address;
use ethers::contract::{abigen, ContractError};
use ethers::middleware::Middleware;
use ethers::prelude::{Bytes, TransactionReceipt};
use utils::errors::Error;

abigen!(
    ProxySupport5_0_0,
    "../../../artifacts/starkgate-contracts/Proxy_5_0_0.json",
);

#[async_trait]
impl<T, M: Middleware> ProxySupport5_0_0Trait<M> for T
where
    T: AsRef<ProxySupport5_0_0<M>> + Send + Sync,
{
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
            .register_app_governor(new_governor)
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
            .revoke_app_governor(governor)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn register_app_governor(
        &self,
        account: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .register_app_governor(account)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn register_app_role_admin(
        &self,
        account: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .register_app_role_admin(account)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn register_governance_admin(
        &self,
        account: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .register_governance_admin(account)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn register_operator(
        &self,
        account: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .register_operator(account)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn register_security_admin(
        &self,
        account: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .register_security_admin(account)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn register_security_agent(
        &self,
        account: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .register_security_agent(account)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn register_token_admin(
        &self,
        account: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .register_token_admin(account)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn register_upgrade_governor(
        &self,
        account: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .register_upgrade_governor(account)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }
}
