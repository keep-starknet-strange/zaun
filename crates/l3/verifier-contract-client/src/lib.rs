pub mod clients;
pub mod interfaces;

use appchain_utils::deploy_contract;
use appchain_utils::{LocalWalletSignerMiddleware, NO_CONSTRUCTOR_ARG};
use clients::StarknetVerifierContractClient;
use color_eyre::Result;
use std::path::Path;


pub async fn deploy_starknet_verifier_contract<'a>(
    signer: &'a LocalWalletSignerMiddleware,
    path_to_sierra: &Path,
    path_to_casm: &Path,
) -> Result<StarknetVerifierContractClient<'a>> {
    let contract_address =
        deploy_contract(signer, path_to_sierra, path_to_casm, NO_CONSTRUCTOR_ARG).await?;

    Ok(StarknetVerifierContractClient::new(contract_address, signer))
}
