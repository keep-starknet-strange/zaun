pub mod clients;
pub mod interfaces;

use clients::client::StarknetCoreContractClient;
use color_eyre::Result;
use common::deploy_contract;
use common::{LocalWalletSignerMiddleware, NO_CONSTRUCTOR_ARG};

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
