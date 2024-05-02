pub mod interfaces;
pub mod clients;

use starknet_instance::deploy_contract;
use common::{LocalWalletSignerMiddleware, NO_CONSTRUCTOR_ARG};
// use clients::TestTokenClient;
// need to import ERROR
// TODO: need to decide the return type of deploy_starknet_core_contract

const STARKNET_CORE_CONTRACT_CASM : &str = include_str!("../artifacts/piltover_appchain.compiled_contract_class.json");
const STARKNET_CORE_CONTRACT_SIERRA : &str = include_str!("../artifacts/piltover_appchain.contract_class.json");

pub async fn deploy_starknet_core_contract(
    client: LocalWalletSignerMiddleware,
) {
// ) -> Result<TestTokkenClient, Error> {
    let contract_address = deploy_contract(client, STARKNET_CORE_CONTRACT_SIERRA, STARKNET_CORE_CONTRACT_CASM, NO_CONSTRUCTOR_ARG).await;
    // Ok(TestTokenClient::new(
    //     contract_address,
    //     client.clone(),
    // ))
}