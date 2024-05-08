use common::invoke_contract;
use common::LocalWalletSignerMiddleware;
use starknet_accounts::Execution;
use starknet_core::types::FieldElement;
use starknet_core::types::StarknetError;
use std::sync::Arc;

pub struct Messaging {
    client: Arc<LocalWalletSignerMiddleware>,
    address: FieldElement,
}

impl Messaging {
    pub fn new(address: FieldElement, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self {
            client,
            address,
        }
    }

    pub async fn send_message_to_appchain(
        &self,
        to_address: FieldElement,
        selector: FieldElement,
        payload: Vec<FieldElement>,
    ) -> Result<Option<Execution<LocalWalletSignerMiddleware>>, StarknetError> {
        let mut calldata = Vec::new();
        calldata.push(to_address);
        calldata.push(selector);
        calldata.extend(payload);
        let execution = invoke_contract(
            &self.client,
            self.address,
            "send_message_to_appchain",
            calldata,
        )
        .await;
        Ok(Some(execution))
    }  

    pub async fn consume_message_from_appchain(
        &self,
        from_address: FieldElement,
        payload: Vec<FieldElement>,
    ) -> Result<Option<Execution<LocalWalletSignerMiddleware>>, StarknetError> {
        let mut calldata = Vec::new();
        calldata.push(from_address);
        calldata.extend(payload);
        let execution = invoke_contract(
            &self.client,
            self.address,
            "consume_message_from_appchain",
            calldata,
        )
        .await;
        Ok(Some(execution))
    }

    pub async fn start_message_cancellation(
        &self,
        to_address: FieldElement,
        selector: FieldElement,
        payload: Vec<FieldElement>,
        nonce: FieldElement,
    ) -> Result<Option<Execution<LocalWalletSignerMiddleware>>, StarknetError> {
        let mut calldata = Vec::new();
        calldata.push(to_address);
        calldata.push(selector);
        calldata.extend(payload);
        calldata.push(nonce);
        let execution = invoke_contract(
            &self.client,
            self.address,
            "start_message_cancellation",
            calldata,
        )
        .await;
        Ok(Some(execution))
    }
    
    pub async fn cancel_message(
        &self,
        to_address: FieldElement,
        selector: FieldElement,
        payload: Vec<FieldElement>,
        nonce: FieldElement,
    ) -> Result<Option<Execution<LocalWalletSignerMiddleware>>, StarknetError> {
        let mut calldata = Vec::new();
        calldata.push(to_address);
        calldata.push(selector);
        calldata.extend(payload);
        calldata.push(nonce);
        let execution = invoke_contract(
            &self.client,
            self.address,
            "cancel_message",
            calldata,
        )
        .await;
        Ok(Some(execution))
    }
}

