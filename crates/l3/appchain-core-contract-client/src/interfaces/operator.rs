use async_trait::async_trait;
use starknet_core::types::FieldElement;
use starknet_core::types::TransactionReceipt;
use starknet_providers::{Provider, ProviderError};
use common::errors::Error;
use starknet_instance::invoke_contract;
use starknet_core::types::StarknetError;
use std::sync::Arc;

// Operator accepts LocalWalletSignerMiddleware as argument
pub struct Operator<M>
where M: Provider,
{
    client: M,
    address: FieldElement,
}

impl<M> Operator<M>
where M: Provider,
{
    pub fn new(address: FieldElement, client: Arc<M>) -> Self {
        Self {
            client: client,
            address: address,
        }
    }
    pub fn register_operator(&self, new_operator: FieldElement) -> Result<Option<TransactionReceipt>, Error> {
        invoke_contract(
            &self.client,
            &self.address,
            "register_operator",
            vec![new_operator.into()],
        )
    }
}


#[async_trait]
pub trait OperatorTrait<M: Provider + Send> {
    async fn register_operator(
        &self, new_operator: FieldElement
    ) -> Result<Option<TransactionReceipt>, Error>;
    // unregister_operator - address
    // is_operator - address -> Returns(bool)
    // set_program_info - program_hash, config_hash
    // get_program_info -> Returns(felt252, felt252)
    // set_facts_registry - address
    // get_facts_registry -> Returns(address)
}

#[async_trait]
impl<T, M> OperatorTrait<M> for T
where
    T: AsRef<Operator<M>> + Send + Sync,
    M: Provider + Send,
{
    async fn register_operator(
        &self, new_operator: FieldElement
    ) -> Result<Option<TransactionReceipt>, Error> {
        self.as_ref()
            .register_operator(new_operator).await
            .map_err(Into::<StarknetError>::into)?
            .await
            .map_err(Into::into)
    }
}