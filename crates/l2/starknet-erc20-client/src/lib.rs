use std::sync::Arc;

use clients::erc20::ERC20ContractClient;
use ethereum_instance::deploy_contract;
use starknet_proxy_client::deploy::Error;
use utils::{LocalWalletSignerMiddleware, NO_CONSTRUCTOR_ARG};

pub mod clients;
pub mod interfaces;

const ERC20_TEST_TOKEN: &str = include_str!("../../../../artifacts/starkgate-contracts/ERC20.json");

pub async fn deploy_dai_test_erc20_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
) -> Result<ERC20ContractClient, Error> {
    // Deploy the Dai ERC20 Token contract (no explicit constructor)
    let contract = deploy_contract(client.clone(), ERC20_TEST_TOKEN, NO_CONSTRUCTOR_ARG)
        .await
        .unwrap();

    Ok(ERC20ContractClient::new(contract.address(), client.clone()))
}
