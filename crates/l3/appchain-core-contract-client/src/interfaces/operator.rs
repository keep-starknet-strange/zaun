// allow unused imports
#![allow(unused_imports)]
use appchain_utils::LocalWalletSignerMiddleware;
use appchain_utils::{call_contract, invoke_contract};
use color_eyre::{eyre::eyre, Result};
use starknet_accounts::ConnectedAccount;
use starknet_core::types::{FieldElement, InvokeTransactionResult};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider,
};

pub struct Operator<'a> {
    signer: &'a LocalWalletSignerMiddleware,
    address: FieldElement,
}

impl<'a> Operator<'a> {
    pub fn new(address: FieldElement, signer: &'a LocalWalletSignerMiddleware) -> Self {
        Self { signer, address }
    }

    fn provider(&self) -> &JsonRpcClient<HttpTransport> {
        self.signer.provider()
    }

    pub async fn register_operator(
        &self,
        new_operator: FieldElement,
    ) -> Result<InvokeTransactionResult> {
        invoke_contract(
            self.signer,
            self.address,
            "register_operator",
            vec![new_operator],
        )
        .await
    }

    pub async fn unregister_operator(
        &self,
        removed_operator: FieldElement,
    ) -> Result<InvokeTransactionResult> {
        invoke_contract(
            self.signer,
            self.address,
            "unregister_operator",
            vec![removed_operator],
        )
        .await
    }

    pub async fn is_operator(&self, operator: FieldElement) -> Result<bool> {
        let provider = self.provider();
        let values = call_contract(provider, self.address, "is_operator", vec![operator]).await?;

        values
            .first()
            .map(|value| *value != FieldElement::ZERO)
            .ok_or_else(|| eyre!("Contract error: expected at least one return value"))
    }

    pub async fn set_program_info(
        &self,
        program_hash: FieldElement,
        config_hash: FieldElement,
    ) -> Result<InvokeTransactionResult> {
        invoke_contract(
            self.signer,
            self.address,
            "set_program_info",
            vec![program_hash, config_hash],
        )
        .await
    }

    pub async fn get_program_info(&self) -> Result<(FieldElement, FieldElement)> {
        let provider = self.provider();
        let values = call_contract(provider, self.address, "get_program_info", vec![]).await?;

        values
            .first()
            .and_then(|first| values.get(1).map(|second| (*first, *second)))
            .ok_or_else(|| eyre!("Contract error: expected exactly two return values"))
    }

    pub async fn set_facts_registry(
        &self,
        facts_registry: FieldElement,
    ) -> Result<InvokeTransactionResult> {
        invoke_contract(
            self.signer,
            self.address,
            "set_facts_registry",
            vec![facts_registry],
        )
        .await
    }

    pub async fn get_facts_registry(&self) -> Result<FieldElement> {
        let provider = self.provider();
        let values = call_contract(provider, self.address, "get_facts_registry", vec![]).await?;

        values
            .first()
            .cloned()
            .ok_or_else(|| eyre!("Contract error: expected at least one return value"))
    }
}
