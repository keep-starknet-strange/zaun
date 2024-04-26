use starknet::{
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Url
    },
    core::types::{TransactionReceipt, DeployTransaction},
    signers::LocalWallet,
    accounts::SingleOwnerAccount,
    contract::ContractFactory,
};
use tokio;

// pub struct StarkNetInstance {
//     provider: Box<dyn Provider>,
//     account: SingleOwnerAccount<dyn LocalWallet>,
// }   

// impl StarkNetInstance {
//     pub fn new(provider_url: &str, private_key: &str) -> Self {
//         let http_transport = HttpTransport::new(Url::parse(provider_url).unwrap());
//         let provider = Box::new(JsonRpcClient::new(http_transport));

//         let wallet = LocalWallet::from_private_key(private_key.parse().unwrap(), Chain::Testnet);
//         let account = SingleOwnerAccount::new(provider.clone(), wallet, None);

//         StarkNetInstance { provider, account }
//     }

//     pub async fn deploy_contract(&self, compiled_contract: &CompiledContract) -> Result<TransactionReceipt, Box<dyn std::error::Error>> {
//         let factory = ContractFactory::new(compiled_contract.clone(), self.account.clone());
//         let deployment = factory.deploy(())?;
//         let receipt = deployment.send().await?;
//         Ok(receipt)
//     }

//     pub async fn call_function(&self, contract_address: FieldElement, selector: FieldElement, calldata: Vec<FieldElement>) -> Result<FieldElement, Box<dyn std::error::Error>> {
//         let call = FunctionCall {
//             contract_address,
//             entry_point_selector: selector,
//             calldata,
//         };

//         let result = self.provider.call(call, BlockId::Latest).await?;
//         Ok(result)
//     }
// }



#[derive(Debug)]
pub struct Client {
    url: Url,
}

const NODE_RPC_URL: &str = "http://localhost:9944";

impl Default for Client {
    fn default() -> Self {
        let url = Url::parse(NODE_RPC_URL).expect("Invalid JSONRPC Url");
        Client::new(url)
    }
}


impl Client {
    pub fn new(url: Url) -> Self {
        Self {url}
    }

    pub fn get_starknet_client(&self) -> JsonRpcClient<HttpTransport> {
        JsonRpcClient::new(HttpTransport::new(self.url.clone()))
    }
}       