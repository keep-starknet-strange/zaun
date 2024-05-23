use color_eyre::{eyre::eyre, Result};
use common::LocalWalletSignerMiddleware;
use common::{call_contract, invoke_contract};
use starknet_core::types::{FieldElement, InvokeTransactionResult};
use std::sync::Arc;

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
        calldata: Vec<FieldElement>,
    ) -> Result<InvokeTransactionResult> {
        invoke_contract(&self.client, self.address, "register_operator", calldata).await
    }

    pub async fn unregister_operator(
        &self,
        calldata: Vec<FieldElement>,
    ) -> Result<InvokeTransactionResult> {
        invoke_contract(&self.client, self.address, "unregister_operator", calldata).await
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
        calldata: Vec<FieldElement>,
    ) -> Result<InvokeTransactionResult> {
        invoke_contract(&self.client, self.address, "set_program_info", calldata).await
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
        calldata: Vec<FieldElement>,
    ) -> Result<InvokeTransactionResult> {
        invoke_contract(&self.client, self.address, "set_facts_registry", calldata).await
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
