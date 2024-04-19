use starknet::{
    core::types::FieldElement,
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Url
    }
};
use appchain_core_contract_client::interfaces::TestTokenClient;

#[tokio::main]
async fn main() {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("http://0.0.0.0:5050/rpc").unwrap(),
    ));

    let contract_address = FieldElement::from_hex_be("0x012882de303fa6907d3c03ef62f9b5ca7e0da51ef5bd6a6438f60089a3f59baf").unwrap();

    let client = TestTokenClient::new(provider, contract_address);

    let operator_address = FieldElement::from_hex_be("0x6162896d1d7ab204c7ccac6dd5f8e9e7c25ecd5ae4fcb4ad32e57786bb46e03").unwrap();

    // match client.register_operator(operator_address).await {
    //     Ok(receipt) => println!("Transaction receipt: {:?}", receipt),
    //     Err(e) => println!("Error calling contract: {:?}", e),
    // }
}