use async_trait::async_trait;
use ethers::{prelude::abigen, providers::Middleware, types::Bytes};

use crate::Error;

abigen!(
    ProxySupport,
    r#"[
        function isFrozen() external view virtual returns (bool)
        function initialize(bytes calldata data) external notCalledDirectly
    ]"#,
);

#[async_trait]
pub trait ProxySupportTrait<M: Middleware> {
    async fn is_frozen(&self) -> Result<bool, Error<M>>;
    async fn initialize(&self, data: Bytes) -> Result<(), Error<M>>;
}

#[async_trait]
impl<T, M: Middleware> ProxySupportTrait<M> for T
where
    T: AsRef<ProxySupport<M>> + Send + Sync,
{
    async fn is_frozen(&self) -> Result<bool, Error<M>> {
        self.as_ref().is_frozen().call().await.map_err(Into::into)
    }

    async fn initialize(&self, data: Bytes) -> Result<(), Error<M>> {
        self.as_ref()
            .initialize(data)
            .call()
            .await
            .map_err(Into::into)
    }
}
