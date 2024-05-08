use common::invoke_contract;
use common::LocalWalletSignerMiddleware;
use starknet_accounts::Execution;
use starknet_core::types::FieldElement;
use starknet_core::types::StarknetError;
use std::sync::Arc;

pub struct CoreContract {
    client: Arc<LocalWalletSignerMiddleware>,
    address: FieldElement,
}

impl CoreContract {
    pub fn new(address: FieldElement, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self { client, address }
    }

    // update_state - program_output: Span<felt252>, onchain_data_hash: fel252, onchain_data_size: u256
    pub async fn update_state(
        &self,
        program_output: Vec<FieldElement>,
        onchain_data_hash: FieldElement,
        onchain_data_size: FieldElement,
    ) -> Result<Option<Execution<LocalWalletSignerMiddleware>>, StarknetError> {
        let mut calldata = Vec::new();
        calldata.extend(program_output);
        calldata.push(onchain_data_hash);
        calldata.push(onchain_data_size);
        let execution = invoke_contract(&self.client, self.address, "update_state", calldata).await;
        Ok(Some(execution))
    }
}
