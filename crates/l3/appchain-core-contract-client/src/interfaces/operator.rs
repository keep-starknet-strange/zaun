use common::LocalWalletSignerMiddleware;
use common::{call_contract, invoke_contract};
use starknet_accounts::Execution;
use starknet_core::types::FieldElement;
use std::sync::Arc;
use color_eyre::{eyre::eyre, Result};

pub struct Operator {
    client: Arc<LocalWalletSignerMiddleware>,
    address: FieldElement,
}

impl Operator {
    pub fn new(address: FieldElement, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self { client, address }
    }

    pub async fn register_operator(
        &self,
        new_operator: FieldElement,
    ) -> Result<Execution<LocalWalletSignerMiddleware>> {
        invoke_contract(
            &self.client,
            self.address,
            "register_operator",
            vec![new_operator.into()],
        )
        .await
    }

    pub async fn unregister_operator(
        &self,
        new_operator: FieldElement,
    ) -> Result<Execution<LocalWalletSignerMiddleware>> {
        invoke_contract(
            &self.client,
            self.address,
            "unregister_operator",
            vec![new_operator.into()],
        )
        .await
    }

    pub async fn is_operator(&self) -> Result<bool> {
        let values = call_contract(&self.client, self.address, "is_operator").await?;
        if let Some(value) = values.first() {
            Ok(value.to_string() != String::from("0"))
        } else {
            Err(eyre!("Contract error: expected at least one return value"))
        }
    }

    pub async fn set_program_info(
        &self,
        program_hash: FieldElement,
        config_hash: FieldElement,
    ) -> Result<Execution<LocalWalletSignerMiddleware>> {
        invoke_contract(
            &self.client,
            self.address,
            "set_program_info",
            vec![program_hash.into(), config_hash.into()],
        )
        .await
    }

    pub async fn get_program_info(&self) -> Result<(FieldElement, FieldElement)> {
        let values = call_contract(&self.client, self.address, "get_program_info").await?;
        if values.len() == 2 {
            Ok((values[0].clone(), values[1].clone()))
        } else {
            Err(eyre!("Contract error: expected exactly two return values"))
        }
    }

    pub async fn set_facts_registry(
        &self,
        facts_registry: FieldElement,
    ) -> Result<Execution<LocalWalletSignerMiddleware>> {
        invoke_contract(
            &self.client,
            self.address,
            "set_facts_registry",
            vec![facts_registry.into()],
        )
        .await
    }

    pub async fn get_facts_registry(&self) -> Result<FieldElement> {
        let values = call_contract(&self.client, self.address, "get_facts_registry").await?;
        if let Some(value) = values.first() {
            Ok(value.clone())
        } else {
            Err(eyre!("Contract error: expected at least one return value"))
        }
    }
}
