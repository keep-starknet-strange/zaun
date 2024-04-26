use starknet::{
    core::types::{FieldElement, Address},
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Url
    }
};
use std::sync::Arc;
use async_trait::async_trait;

use appchain_core_contract_client::interfaces::TestTokenClient;

// #[tokio::main]
// async fn main() {
//     let provider = JsonRpcClient::new(HttpTransport::new(
//         Url::parse("http://0.0.0.0:5050/rpc").unwrap(),
//     ));

//     let contract_address = FieldElement::from_hex_be("0x012882de303fa6907d3c03ef62f9b5ca7e0da51ef5bd6a6438f60089a3f59baf").unwrap();

//     let client = TestTokenClient::new(provider, contract_address);

//     let operator_address = FieldElement::from_hex_be("0x6162896d1d7ab204c7ccac6dd5f8e9e7c25ecd5ae4fcb4ad32e57786bb46e03").unwrap();

//     // match client.register_operator(operator_address).await {
//     //     Ok(receipt) => println!("Transaction receipt: {:?}", receipt),
//     //     Err(e) => println!("Error calling contract: {:?}", e),
//     // }
// }




// #[async_trait]
// pub trait MyStarknetProvider: Send + Sync {
//     async fn register_operator(&self, operator_address: Address) -> Result<Vec<FieldElement>, Box<dyn std::error::Error>>;
// }

// pub struct StarknetProvider {
//     client: Arc<JsonRpcClient<HttpTransport>>,
// }

// impl StarknetProvider {
//     pub fn new(url: Url) -> Self {
//         let transport = HttpTransport::new(url);
//         let client = JsonRpcClient::new(transport);
//         StarknetProvider { client: Arc::new(client) }
//     }
// }

// #[async_trait]
// impl MyStarknetProvider for StarknetProvider {
//     async fn register_operator(&self, operator_address: Address) -> Result<Vec<FieldElement>, Box<dyn std::error::Error>> {
//         self.client.call(
//             FunctionCall {
//                 contract_address: self.contract_address,
//                 entry_point_selector: get_selector_from_name("register_operator").unwrap(),
//                 calldata: vec![operator],
//             },
//             BlockId::Tag(BlockTag::Latest),
//         ).await?;
//     }
// }




use starknet::providers::{Provider, HttpProvider};
use starknet::core::types::{Contract, Transaction};
use starknet::signers::{Wallet, LocalWallet};
use std::sync::Arc;

// /// Errors that can occur during StarkNet operations
// #[derive(Debug, thiserror::Error)]
// pub enum Error {
//     #[error("Network error: {0}")]
//     NetworkError(#[from] starknet::providers::ProviderError),

//     #[error("Contract deployment failed: {0}")]
//     DeploymentError(String),
// }

// /// StarkNet client to interact with the network
// pub struct StarknetClient {
//     provider: Arc<HttpProvider>,
//     wallet: LocalWallet,
// }

// impl StarknetClient {
//     /// Creates a new client instance
//     pub fn new(provider_url: &str, private_key: &str) -> Result<Self, Error> {
//         let provider = Arc::new(HttpProvider::new(provider_url));
//         let wallet = LocalWallet::from_private_key(private_key, provider.clone());
//         Ok(Self { provider, wallet })
//     }

//     /// Deploys a contract to StarkNet
//     pub async fn deploy_contract(&self, contract_artifacts: &str) -> Result<Contract, Error> {
//         let compiled_contract = starknet::core::compile_contract(contract_artifacts)?;
//         let deployment_tx = Transaction::new(&compiled_contract, &[]);
//         let response = self.wallet.send_transaction(&deployment_tx).await?;
//         Ok(response)
//     }
// }


use starknet::providers::{Provider, SequencerGatewayProvider};
use starknet::accounts::{Account, SingleOwnerAccount};
use starknet::signers::{LocalWallet, SigningKey};
use starknet::core::types::{FieldElement, ChainId};
use starknet::contract::{ContractFactory, Deployment};



// async fn deploy_contract() -> Result<(), Box<dyn std::error::Error>> {
//     // Setup JSON-RPC transport and client
//     let transport = HttpTransport::new(Url::parse("https://alpha4.starknet.io")?);
//     let client = JsonRpcClient::new(transport);

//     // Placeholder for account and contract deployment logic
//     // This should include loading the account and creating the contract deployment object
//     let account_address = Address::new(FieldElement::from_hex_be("account_address_hex").unwrap());
//     let class_hash = FieldElement::from_hex_be("class_hash_hex").unwrap();
    
//     // Constructor calldata and salt
//     let constructor_calldata = vec![FieldElement::from(1), FieldElement::from(2)];
//     let salt = FieldElement::from(123456);
    
//     // Simulating the contract deployment
//     let result = client.deploy_contract(
//         account_address,
//         class_hash,
//         constructor_calldata,
//         salt,
//         true // Assuming this is a uniqueness parameter
//     ).await?;

//     println!("Deployment result: {:?}", result);
//     Ok(())
// }


use starknet::providers::{Provider, HttpProvider};
use starknet::signers::{LocalWallet, Signer};
use starknet::core::types::{FieldElement, Contract};

pub struct StarknetClient {
    wallet: LocalWallet<HttpProvider>,
    provider: HttpProvider,
}