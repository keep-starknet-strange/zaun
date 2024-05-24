use color_eyre::Result;
use common::invoke_contract;
use common::LocalWalletSignerMiddleware;
use starknet_core::types::{FieldElement, InvokeTransactionResult};
use std::sync::Arc;

pub struct CoreContract<'a> {
    client: Arc<&'a LocalWalletSignerMiddleware>,
    address: FieldElement,
}

impl<'a> CoreContract<'a> {
    pub fn new(address: FieldElement, client: Arc<&'a LocalWalletSignerMiddleware>) -> Self {
        Self { client, address }
    }

    pub async fn update_state(
        &self,
        calldata: Vec<FieldElement>,
    ) -> Result<InvokeTransactionResult> {
        invoke_contract(&self.client, self.address, "update_state", calldata).await
    }
}
