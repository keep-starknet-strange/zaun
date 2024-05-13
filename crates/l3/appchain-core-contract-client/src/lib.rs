pub mod clients;
pub mod interfaces;

use clients::client::StarknetCoreContractClient;
use common::deploy_contract;
use common::errors::Error;
use common::{LocalWalletSignerMiddleware, NO_CONSTRUCTOR_ARG};

use std::sync::Arc;
// TODO: check for proxy contract implementation

pub async fn deploy_starknet_core_contract(
    client: Arc<LocalWalletSignerMiddleware>,
    path_to_sierra: &str,
    path_to_casm: &str,
) -> Result<StarknetCoreContractClient, Error> {
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
