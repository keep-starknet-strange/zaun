use async_trait::async_trait;
use starknet_core::types::FieldElement;
use starknet_core::types::TransactionReceipt;
use starknet_providers::ProviderError;
use common::errors::Error;
use starknet_core::types::EthAddress;

#[async_trait]
pub trait OperatorTrait {
    // Add other necessary methods
    // register_operator - address
    async fn register_operator(
        &self, new_operator: EthAddress
    ) -> Result<Option<TransactionReceipt>, Error>;
    // unregister_operator - address
    // is_operator - address -> Returns(bool)
    // set_program_info - program_hash, config_hash
    // get_program_info -> Returns(felt252, felt252)
    // set_facts_registry - address
    // get_facts_registry -> Returns(address)
}

#[async_trait]
impl OperatorTrait 
{
    async fn register_operator(
        &self, new_operator: EthAddress
    ) -> Result<Option<TransactionReceipt>, Error> {
        // self.as_ref()
        //     .register_operator(new_operator)
        //     .send()
        //     .await
        //     .map_err(Into::<ProviderError>::into)?
        //     .map_err(Into::into)
    }
}