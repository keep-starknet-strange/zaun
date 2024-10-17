use appchain_utils::invoke_contract;
use appchain_utils::LocalWalletSignerMiddleware;
use color_eyre::Result;
use starknet_core::types::{Felt, InvokeTransactionResult};

pub struct CoreContract {
    signer: LocalWalletSignerMiddleware,
    address: Felt,
}

impl CoreContract {
    pub fn new(address: Felt, signer: LocalWalletSignerMiddleware) -> Self {
        Self { signer, address }
    }

    pub async fn update_state(
        &self,
        program_output: Vec<Felt>,
        onchain_data_hash: Felt,
        onchain_data_size: Felt,
    ) -> Result<InvokeTransactionResult> {
        let mut calldata = Vec::with_capacity(program_output.len() + 2);
        calldata.extend(program_output);
        calldata.push(onchain_data_hash);
        calldata.push(onchain_data_size);

        invoke_contract(&self.signer, self.address, "update_state", calldata).await
    }
}
