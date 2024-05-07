use common::{call_contract, invoke_contract};
use common::LocalWalletSignerMiddleware;
use starknet_accounts::Execution;
use starknet_core::types::FieldElement;
use starknet_core::types::StarknetError;
use std::sync::Arc;

pub struct Operator {
    client: Arc<LocalWalletSignerMiddleware>,
    address: FieldElement,
}

impl Operator {
    pub fn new(address: FieldElement, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self {
            client,
            address,
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

    pub async fn unregister_operator(
        &self,
        new_operator: FieldElement,
    ) -> Result<Option<Execution<LocalWalletSignerMiddleware>>, StarknetError> {
        let execution = invoke_contract(
            &self.client,
            self.address,
            "unregister_operator",
            vec![new_operator.into()],
        )
        .await;
        Ok(Some(execution))
    }

    pub async fn is_operator(
        &self,
    ) -> Result<bool, StarknetError> {
        let result = call_contract(self.client.clone(), self.address, "is_operator").await;
        match result {
            Some(values) => {
                if let Some(value) = values.first() {
                    Ok(value.to_string() != String::from("0"))
                } else {
                    Err(StarknetError::ContractError)
                }
            },
            None => Err(StarknetError::ContractError),
        }
    }   

    pub async fn set_program_info(
        &self,
        program_hash: FieldElement,
        config_hash: FieldElement,
    ) -> Result<Option<Execution<LocalWalletSignerMiddleware>>, StarknetError> {
        let execution = invoke_contract(
            &self.client,
            self.address,
            "set_program_info",
            vec![program_hash.into(), config_hash.into()],
        )
        .await;
        Ok(Some(execution))
    }

    pub async fn get_program_info(
        &self,
    ) -> Result<(FieldElement, FieldElement), StarknetError> {
        let result = call_contract(self.client.clone(), self.address, "get_program_info").await;
        match result {
            Some(values) => {
                if values.len() == 2 {
                    Ok((values[0].clone(), values[1].clone()))
                } else {
                    Err(StarknetError::ContractError)
                }
            },
            None => Err(StarknetError::ContractError),
        }
    }

    pub async fn set_facts_registry(
        &self,
        facts_registry: FieldElement,
    ) -> Result<Option<Execution<LocalWalletSignerMiddleware>>, StarknetError> {
        let execution = invoke_contract(
            &self.client,
            self.address,
            "set_facts_registry",
            vec![facts_registry.into()],
        )
        .await;
        Ok(Some(execution))
    }

    pub async fn get_facts_registry(
        &self,
    ) -> Result<FieldElement, StarknetError> {
        let result = call_contract(self.client.clone(), self.address, "get_facts_registry").await;
        match result {
            Some(values) => {
                if let Some(value) = values.first() {
                    Ok(value.clone())
                } else {
                    Err(StarknetError::ContractError)
                }
            },
            None => Err(StarknetError::ContractError),
        }
    }
}
