use color_eyre::Result;
use appchain_utils::invoke_contract;
use appchain_utils::LocalWalletSignerMiddleware;
use starknet_core::types::{FieldElement, InvokeTransactionResult};

pub struct Messaging<'a> {
    signer: &'a LocalWalletSignerMiddleware,
    address: FieldElement,
}

impl<'a> Messaging<'a> {
    pub fn new(address: FieldElement, signer: &'a LocalWalletSignerMiddleware) -> Self {
        Self { signer, address }
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
            &self.signer,
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
            &self.signer,
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
            &self.signer,
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

        invoke_contract(&self.signer, self.address, "cancel_message", calldata).await
    }
}
