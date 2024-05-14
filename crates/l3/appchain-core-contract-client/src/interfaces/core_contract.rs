use common::invoke_contract;
use common::LocalWalletSignerMiddleware;
use starknet_accounts::Execution;
use starknet_core::types::FieldElement;
use std::sync::Arc;
use color_eyre::Result;

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
        program_output: Vec<FieldElement>,
        onchain_data_hash: FieldElement,
        onchain_data_size: FieldElement,
    ) -> Result<Execution<LocalWalletSignerMiddleware>> {
        let mut calldata = Vec::new();
        calldata.extend(program_output);
        calldata.push(onchain_data_hash);
        calldata.push(onchain_data_size);
        invoke_contract(&self.client, self.address, "update_state", calldata).await
    }
}
