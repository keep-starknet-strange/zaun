use color_eyre::Result;
use common::invoke_contract;
use common::LocalWalletSignerMiddleware;
use starknet_core::types::{FieldElement, InvokeTransactionResult};
use std::sync::Arc;

pub struct CoreContract {
    client: Arc<LocalWalletSignerMiddleware>,
    address: FieldElement,
}

impl CoreContract {
    pub fn new(address: FieldElement, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self { client, address }
    }

    pub async fn update_state(
        &self,
        calldata: Vec<FieldElement>,
    ) -> Result<InvokeTransactionResult> {
        invoke_contract(&self.client, self.address, "update_state", calldata).await
    }
}
