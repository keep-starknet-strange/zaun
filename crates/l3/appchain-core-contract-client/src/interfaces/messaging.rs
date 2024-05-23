use color_eyre::Result;
use common::invoke_contract;
use common::LocalWalletSignerMiddleware;
use starknet_core::types::{FieldElement, InvokeTransactionResult};
use std::sync::Arc;

pub struct Messaging {
    client: Arc<LocalWalletSignerMiddleware>,
    address: FieldElement,
}

impl Messaging {
    pub fn new(address: FieldElement, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self { client, address }
    }

    pub async fn send_message_to_appchain(
        &self,
        calldata: Vec<FieldElement>,
    ) -> Result<InvokeTransactionResult> {
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
        calldata: Vec<FieldElement>,
    ) -> Result<InvokeTransactionResult> {
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
        calldata: Vec<FieldElement>,
    ) -> Result<InvokeTransactionResult> {
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
        calldata: Vec<FieldElement>,
    ) -> Result<InvokeTransactionResult> {
        invoke_contract(&self.client, self.address, "cancel_message", calldata).await
    }
}
