use crate::interfaces::verifier_contract::VerifierContract;
use appchain_utils::LocalWalletSignerMiddleware;
use starknet_ff::FieldElement;

pub struct StarknetVerifierContractClient<'a> {
    verifier_contract: VerifierContract<'a>,
}

impl<'a> StarknetVerifierContractClient<'a> {
    pub fn new(address: FieldElement, signer: &'a LocalWalletSignerMiddleware) -> Self {
        Self {
            verifier_contract: VerifierContract::new(address, signer),
        }
    }
}

impl<'a> AsRef<VerifierContract<'a>> for StarknetVerifierContractClient<'a> {
    fn as_ref(&self) -> &VerifierContract<'a> {
        &self.verifier_contract
    }
}
