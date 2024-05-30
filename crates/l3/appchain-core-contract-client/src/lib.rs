pub mod clients;
pub mod interfaces;

use clients::StarknetCoreContractClient;
use color_eyre::Result;
use appchain_utils::deploy_contract;
use appchain_utils::{LocalWalletSignerMiddleware, NO_CONSTRUCTOR_ARG};

use std::sync::Arc;
// TODO: check for proxy contract implementation

pub async fn deploy_starknet_core_contract<'a>(
    client: Arc<&'a LocalWalletSignerMiddleware>,
    path_to_sierra: &str,
    path_to_casm: &str,
) -> Result<StarknetCoreContractClient<'a>> {
    let contract_address = deploy_contract(
        client.clone(),
        path_to_sierra,
        path_to_casm,
        NO_CONSTRUCTOR_ARG,
    )
    .await?;

    Ok(StarknetCoreContractClient::new(
        contract_address,
        client.clone(),
    ))
}
