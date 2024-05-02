use url::Url;
use starknet_providers::jsonrpc::{HttpTransport, JsonRpcClient};
//use starknet::signers::LocalWallet;
use starknet_core::types::contract::{CompiledClass, SierraClass};
//use starknet::accounts::{ExecutionEncoding, Execution, Declaration, AccountDeployment, OpenZeppelinAccountFactory};
use starknet_signers::{LocalWallet, SigningKey};
use async_trait::async_trait;
use starknet_accounts::{
    Account, Execution, OpenZeppelinAccountFactory, SingleOwnerAccount, Declaration, AccountDeployment
};
use starknet_ff::FieldElement;
use starknet_contract::{ContractFactory};

pub type RpcOzAccountFactory<'a> = OpenZeppelinAccountFactory<LocalWallet, &'a JsonRpcClient<HttpTransport>>;
pub type TransactionExecution<'a> = Execution<'a, RpcAccount<'a>>;
type TransactionDeclaration<'a> = Declaration<'a, RpcAccount<'a>>;
type TransactionAccountDeployment<'a> = AccountDeployment<'a, RpcOzAccountFactory<'a>>;
type RpcAccount<'a> = SingleOwnerAccount<&'a JsonRpcClient<HttpTransport>, LocalWallet>;
//const STARKNET_CHAIN_ID: FieldElement = FieldElement::from_hex_be("0x4b4154414e41").unwrap();
const NODE_RPC_URL: &str = "http://0.0.0.0:5050";

pub enum Transaction<'a> {
    Execution(TransactionExecution<'a>),
    Declaration(TransactionDeclaration<'a>),
    AccountDeployment(TransactionAccountDeployment<'a>),
}


#[derive(Debug)]
pub struct StarknetClient{
    url: Url
}

impl Default for StarknetClient {
    fn default() -> Self {
        let url = Url::parse(NODE_RPC_URL).expect("Invalid JSONRPC Url");
        StarknetClient::new(url)
    }
}

impl StarknetClient {
    pub fn new(url: Url) -> Self {
        Self { url }
    }

    pub fn get_starknet_client(&self) -> JsonRpcClient<HttpTransport> {
        JsonRpcClient::new(HttpTransport::new(self.url.clone()))
    }
}

#[async_trait]
pub trait AccountActions{
    fn declare_contract(
        &self,
        path_to_sierra: &str,
        path_to_casm: &str,
    ) -> (TransactionDeclaration, FieldElement, FieldElement);

}

#[async_trait]
impl AccountActions for SingleOwnerAccount<&JsonRpcClient<HttpTransport>, LocalWallet> {
    fn declare_contract(
        &self,
        path_to_sierra: &str,
        path_to_casm: &str,
    ) -> (TransactionDeclaration, FieldElement, FieldElement) {
        let sierra: SierraClass = serde_json::from_reader(
            std::fs::File::open(env!("CARGO_MANIFEST_DIR").to_owned() + "/" + path_to_sierra).unwrap(),
        )
        .unwrap();
        let casm: CompiledClass = serde_json::from_reader(
            std::fs::File::open(env!("CARGO_MANIFEST_DIR").to_owned() + "/" + path_to_casm).unwrap(),
        )
        .unwrap();
        let compiled_class_hash = casm.class_hash().unwrap();
        (
            self.declare(sierra.clone().flatten().unwrap().into(), compiled_class_hash)
				// starknet-rs calls estimateFee with incorrect version which throws an error
                .max_fee(FieldElement::from_hex_be("0x100000").unwrap()),
            sierra.class_hash().unwrap(),
            compiled_class_hash,
        )
    }
}

pub fn build_single_owner_account<'a>(
    rpc: &'a JsonRpcClient<HttpTransport>,
    private_key: &str,
    account_address: &str,
    is_legacy: bool,
) -> RpcAccount<'a> {
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(FieldElement::from_hex_be(private_key).unwrap()));
    let account_address = FieldElement::from_hex_be(account_address).expect("Invalid Contract Address");
    let execution_encoding = if is_legacy {
        starknet_accounts::ExecutionEncoding::Legacy
    } else {
        starknet_accounts::ExecutionEncoding::New
    };
    SingleOwnerAccount::new(rpc, signer, account_address, FieldElement::from_hex_be("0x4b4154414e41").unwrap(), execution_encoding)
}

const SIERRA_PATH: &str = "./artifacts/piltover_config_mock.contract_class.json";
const CASM_PATH: &str = "./artifacts/piltover_config_mock.compiled_contract_class.json";


async fn starknet_core_contract_deploy() -> Result<(), Box<dyn std::error::Error>>{

    let starknet_client = StarknetClient::new(Url::parse(NODE_RPC_URL).expect("Invalid URL"));
    let rpc = starknet_client.get_starknet_client();
    let account = build_single_owner_account(&rpc, "0x33003003001800009900180300d206308b0070db00121318d17b5e6262150b", "0x2d71e9c974539bb3ffb4b115e66a23d0f62a641ea66c4016e903454c8753bbc", false);
    let (declare_tx, class_hash, compiled_class_hash) = account.declare_contract(SIERRA_PATH, CASM_PATH);

    let result = declare_tx.send().await.unwrap();
    println!("Contract declared at address: {:?}", result);
    // print class hash
    println!("Class hash {:?}", class_hash);
    println!("Compiled Class hash {:?}", compiled_class_hash);
    let contract_factory = ContractFactory::new(class_hash, account.clone());
    let deploy_tx = &contract_factory.deploy(
       vec![FieldElement::from_hex_be("0x2d71e9c974539bb3ffb4b115e66a23d0f62a641ea66c4016e903454c8753bbc")?],
    //    vec![felt!("0x2d71e9c974539bb3ffb4b115e66a23d0f62a641ea66c4016e903454c8753bbc")],
       FieldElement::ZERO,
       true,
    );
    deploy_tx.deployed_address();
    // Print
    //println!("Contract deployed at address: {:?}", deploy_tx.deployed_address());
    // print url
    println!("URL: {:?}", NODE_RPC_URL);

    Ok(())
}


#[tokio::main]
async fn main() {
    // calling starknet_core_contract_consumes_messages_from_l2
    let _ = starknet_core_contract_deploy().await;
}






//#[tokio::main]
//async fn main() -> Result<(), Box<dyn std::error::Error>> {
//    let provider = Provider::<HttpTransport>::try_from("http://localhost:5050")?;
//    let account = SingleOwnerAccount::new(provider.clone(), wallet, None);
//
//    let client = AppChainCoreContractClient::new(provider, account);
//    // Use client to deploy or interact with the contract
//    Ok(())
//}