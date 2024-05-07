use common::{call_contract, invoke_contract};
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
        Self {
            client,
            address,
        }
    }

    // update_state - program_output: Span<felt252>, onchain_data_hash: fel252, onchain_data_size: u256
}
