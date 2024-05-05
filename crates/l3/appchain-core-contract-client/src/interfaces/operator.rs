use async_trait::async_trait;
use common::invoke_contract;
use common::LocalWalletSignerMiddleware;
use starknet_accounts::Execution;
use starknet_core::types::FieldElement;
use starknet_core::types::StarknetError;
use std::sync::Arc;

// Operator accepts LocalWalletSignerMiddleware as argument
pub struct Operator {
    client: Arc<LocalWalletSignerMiddleware>,
    address: FieldElement,
}

impl Operator {
    pub fn new(address: FieldElement, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self {
            client: client,
            address: address,
        }
    }
    pub async fn register_operator(
        &self,
        new_operator: FieldElement,
    ) -> Result<Option<Execution<LocalWalletSignerMiddleware>>, StarknetError> {
        let execution = invoke_contract(
            &self.client,
            self.address,
            "register_operator",
            vec![new_operator.into()],
        )
        .await;
        Ok(Some(execution))
    }
}

#[async_trait]
pub trait OperatorTrait {
    async fn register_operator(
        &self,
        new_operator: FieldElement,
    ) -> Result<Option<Execution<LocalWalletSignerMiddleware>>, StarknetError>;
    // unregister_operator - address
    // is_operator - address -> Returns(bool)
    // set_program_info - program_hash, config_hash
    // get_program_info -> Returns(felt252, felt252)
    // set_facts_registry - address
    // get_facts_registry -> Returns(address)
}

#[async_trait]
impl<T> OperatorTrait for T
where
    T: AsRef<Operator> + Send + Sync,
{
    async fn register_operator(
        &self,
        new_operator: FieldElement,
    ) -> Result<Option<Execution<LocalWalletSignerMiddleware>>, StarknetError> {
        self.as_ref()
            .register_operator(new_operator)
            .await
            .map_err(Into::<StarknetError>::into)
    }
}
