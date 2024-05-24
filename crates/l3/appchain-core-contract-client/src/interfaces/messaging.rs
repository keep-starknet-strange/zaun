use color_eyre::Result;
use common::invoke_contract;
use common::LocalWalletSignerMiddleware;
use starknet_core::types::{FieldElement, InvokeTransactionResult};
use std::sync::Arc;

pub struct Messaging<'a> {
    client: Arc<&'a LocalWalletSignerMiddleware>,
    address: FieldElement,
}

impl<'a> Messaging<'a> {
    pub fn new(address: FieldElement, client: Arc<&'a LocalWalletSignerMiddleware>) -> Self {
        Self { client, address }
    }

    pub async fn send_message_to_appchain(
        &self,
        to_address: FieldElement,
        selector: FieldElement,
        payload: Vec<FieldElement>,
    ) -> Result<InvokeTransactionResult> {
        let mut calldata = Vec::new();
        calldata.push(to_address);
        calldata.push(selector);
        calldata.extend(payload);
        invoke_contract(
            &self.client,
            self.address,
            "send_message_to_appchain",
            calldata,
        )
        .await
    }

    pub async fn consume_message_from_appchain(
        &self,
        from_address: FieldElement,
        payload: Vec<FieldElement>,
    ) -> Result<InvokeTransactionResult> {
        let mut calldata = Vec::new();
        calldata.push(from_address);
        calldata.extend(payload);
        invoke_contract(
            &self.client,
            self.address,
            "consume_message_from_appchain",
            calldata,
        )
        .await
    }

    pub async fn start_message_cancellation(
        &self,
        to_address: FieldElement,
        selector: FieldElement,
        nonce: FieldElement,
        payload: Vec<FieldElement>,
    ) -> Result<InvokeTransactionResult> {
        let mut calldata = Vec::new();
        calldata.push(to_address);
        calldata.push(selector);
        calldata.push(nonce);
        calldata.extend(payload);
        invoke_contract(
            &self.client,
            self.address,
            "start_message_cancellation",
            calldata,
        )
        .await
    }

    pub async fn cancel_message(
        &self,
        to_address: FieldElement,
        selector: FieldElement,
        nonce: FieldElement,
        payload: Vec<FieldElement>,
    ) -> Result<InvokeTransactionResult> {
        let mut calldata = Vec::new();
        calldata.push(to_address);
        calldata.push(selector);
        calldata.push(nonce);
        calldata.extend(payload);
        invoke_contract(&self.client, self.address, "cancel_message", calldata).await
    }
}
