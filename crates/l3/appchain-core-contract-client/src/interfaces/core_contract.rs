use color_eyre::Result;
use appchain_utils::invoke_contract;
use appchain_utils::LocalWalletSignerMiddleware;
use starknet_core::types::{FieldElement, InvokeTransactionResult};

pub struct CoreContract<'a> {
    signer: &'a LocalWalletSignerMiddleware,
    address: FieldElement,
}

impl<'a> CoreContract<'a> {
    pub fn new(address: FieldElement, signer: &'a LocalWalletSignerMiddleware) -> Self {
        Self { signer, address }
    }

    pub async fn update_state(
        &self,
        program_output: Vec<FieldElement>,
        onchain_data_hash: FieldElement,
        onchain_data_size: FieldElement,
    ) -> Result<InvokeTransactionResult> {
        let mut calldata = Vec::new();
        calldata.extend(program_output);
        calldata.push(onchain_data_hash);
        calldata.push(onchain_data_size);

        invoke_contract(&self.signer, self.address, "update_state", calldata).await
    }
}
