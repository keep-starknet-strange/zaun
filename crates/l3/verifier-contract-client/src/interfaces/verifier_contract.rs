use appchain_utils::LocalWalletSignerMiddleware;
use appchain_utils::{call_contract, invoke_contract};
use color_eyre::eyre::eyre;
use color_eyre::Result;
use starknet_accounts::ConnectedAccount;
use starknet_core::types::{FieldElement, InvokeTransactionResult};
use starknet_providers::jsonrpc::HttpTransport;
use starknet_providers::JsonRpcClient;

pub struct VerifierContract<'a> {
    signer: &'a LocalWalletSignerMiddleware,
    address: FieldElement,
}

impl<'a> VerifierContract<'a> {
    pub fn new(address: FieldElement, signer: &'a LocalWalletSignerMiddleware) -> Self {
        Self { signer, address }
    }

    fn provider(&self) -> &JsonRpcClient<HttpTransport> {
        self.signer.provider()
    }
    pub async fn verify_and_register_fact(
        &self,
        serialized_proof: Vec<FieldElement>,
    ) -> Result<InvokeTransactionResult> {
        invoke_contract(
            self.signer,
            self.address,
            "verify_and_register_fact",
            serialized_proof,
        )
        .await
    }

    pub async fn verify_and_register_fact_from_contract(
        &self,
        contract_address: FieldElement,
    ) -> Result<InvokeTransactionResult> {
        let mut calldata = Vec::new();
        calldata.push(contract_address);

        invoke_contract(
            self.signer,
            self.address,
            "verify_and_register_fact_from_contract",
            vec![contract_address],
        )
        .await
    }

    pub async fn is_valid(&self, fact: FieldElement) -> Result<bool> {
        let provider = self.provider();
        let values = call_contract(provider, self.address, "is_valid", vec![fact]).await?;

        values
            .first()
            .map(|value| *value != FieldElement::ZERO)
            .ok_or_else(|| eyre!("Contract error: expected at least one return value"))
    }
}
