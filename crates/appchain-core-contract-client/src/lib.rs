pub mod interfaces;
pub mod clients;

//use clients::TestTokenClient;

//use starknet_proxy_client::deploy::{deploy_contract_behind_unsafe_proxy, Error};
//use utils::{LocalWalletSignerMiddleware, NO_CONSTRUCTOR_ARG};

//const STARKNET_SOVEREIGN: &str = include_str!("./artifacts/Starknet.json");

// pub async fn deploy_starknet_sovereign_behind_unsafe_proxy(
    //client: Arc<LocalWalletSignerMiddleware>,
//) -> Result<TestTokenClient, Error> {
    //// Deploy the Starknet Core contract (no explicit constructor)
    //let core_contract = deploy_contract_behind_unsafe_proxy(client.clone(), STARKNET_SOVEREIGN, NO_CONSTRUCTOR_ARG).await?;

    //Ok(TestTokenClient::new(
        //core_contract.address(),
        //client.clone(),
    //))
//}