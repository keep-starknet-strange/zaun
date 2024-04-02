use std::sync::Arc;

use clients::erc20::ERC20ContractClient;
use starknet_proxy_client::deploy::Error;
use utils::LocalWalletSignerMiddleware;
use ethereum_instance::deploy_contract;

pub mod clients;
pub mod interfaces;

const DAI_ERC20_TOKEN: &str = include_str!("./artifacts/ERC20Token.json");

pub async fn deploy_dai_erc20_behind_unsafe_proxy(
    client: Arc<LocalWalletSignerMiddleware>
) -> Result<ERC20ContractClient, Error> {
    // Deploy the Dai ERC20 Token contract (no explicit constructor)
    let contract = deploy_contract(client.clone(), DAI_ERC20_TOKEN, ()).await.unwrap();

    Ok(ERC20ContractClient::new(
        contract.address(),
        client.clone(),
    ))
}