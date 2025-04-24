pub mod clients;
pub mod interfaces;

use appchain_utils::deploy_contract;
use appchain_utils::{LocalWalletSignerMiddleware, NO_CONSTRUCTOR_ARG};
use clients::StarknetCoreContractClient;
use color_eyre::Result;
use std::path::Path;

// TODO: check for proxy contract implementation

pub async fn deploy_starknet_core_contract(
    signer: LocalWalletSignerMiddleware,
    path_to_sierra: &Path,
    path_to_casm: &Path,
) -> Result<StarknetCoreContractClient> {
    let contract_address =
        deploy_contract(&signer, path_to_sierra, path_to_casm, NO_CONSTRUCTOR_ARG).await?;

    Ok(StarknetCoreContractClient::new(contract_address, signer))
}
