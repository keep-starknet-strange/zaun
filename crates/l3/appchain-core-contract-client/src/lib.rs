pub mod clients;
pub mod interfaces;

use common::{LocalWalletSignerMiddleware, NO_CONSTRUCTOR_ARG};
use starknet_instance::deploy_contract;
// TODO: check for proxy contract implementation 

const STARKNET_CORE_CONTRACT_CASM: &str =
    include_str!("../artifacts/piltover_appchain.compiled_contract_class.json");
const STARKNET_CORE_CONTRACT_SIERRA: &str =
    include_str!("../artifacts/piltover_appchain.contract_class.json");

pub async fn deploy_starknet_core_contract(client: LocalWalletSignerMiddleware) {
    // ) -> Result<TestTokkenClient, Error> {
    let _contract_address = deploy_contract(
        client,
        STARKNET_CORE_CONTRACT_SIERRA,
        STARKNET_CORE_CONTRACT_CASM,
        NO_CONSTRUCTOR_ARG,
    )
    .await;
}
