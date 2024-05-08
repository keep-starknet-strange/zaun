use starknet_accounts::{Account, Execution, SingleOwnerAccount};
use starknet_contract::ContractFactory;
use starknet_core::types::contract::{CompiledClass, SierraClass};
use starknet_ff::FieldElement;
use starknet_providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet_signers::{LocalWallet, SigningKey};
use std::sync::Arc;
use url::Url;

// const STARKNET_DEFAULT_URL: &str = "http://0.0.0.0:5050";
//const STARKNET_CHAIN_ID : FieldElement = FieldElement::from_hex_be("0x4b4154414e41").unwrap();
pub const MAX_FEE_OVERRIDE: &str = "0x100000";

pub type LocalWalletSignerMiddleware =
    SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>;
type RpcAccount<'a> = SingleOwnerAccount<&'a JsonRpcClient<HttpTransport>, LocalWallet>;
pub type TransactionExecution<'a> = Execution<'a, RpcAccount<'a>>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("['bytecode']['object'] is not a string")]
    BytecodeObject,
    #[error(transparent)]
    Hex(#[from] hex::FromHexError),
    #[error("Failed to parse URL")]
    UrlParser,
    //#[error(transparent)]
    //EthersContract(#[from] ContractError<LocalWalletSignerMiddleware>),
    //#[error(transparent)]
    //EthersProvider(#[from] ProviderError),
    //#[error("Invalid contract build artifacts: missing field `{0}`")]
    //ContractBuildArtifacts(&'static str),
}

#[derive(Debug)]
pub struct StarknetClient {
    client: LocalWalletSignerMiddleware,
    url: Url,
}

impl StarknetClient {
    pub fn attach(
        rpc_endpoint: Option<String>,
        priv_key: Option<String>,
        account_addr: Option<String>,
    ) -> Result<Self, Error> {
        let url = Url::parse(rpc_endpoint.unwrap().as_str()).expect("Invalid JSONRPC Url");
        let provider = JsonRpcClient::new(HttpTransport::new(url.clone()));
        let signer = LocalWallet::from(SigningKey::from_secret_scalar(
            FieldElement::from_hex_be(priv_key.unwrap().as_str()).unwrap(),
        ));
        let account_address = FieldElement::from_hex_be(account_addr.unwrap().as_str())
            .expect("Invalid Contract Address");
        let account = SingleOwnerAccount::new(
            provider,
            signer,
            account_address,
            FieldElement::from_hex_be("0x4b4154414e41").unwrap(),
            starknet_accounts::ExecutionEncoding::New,
        );
        Ok(Self {
            client: account,
            url: url,
        })
    }
}

pub async fn deploy_contract(
    client: Arc<LocalWalletSignerMiddleware>,
    contract_build_sierra: &str,
    contract_build_casm: &str,
    constructor_args: Vec<FieldElement>,
) -> FieldElement {
    let sierra: SierraClass = serde_json::from_reader(
        std::fs::File::open(env!("CARGO_MANIFEST_DIR").to_owned() + "/" + contract_build_sierra)
            .unwrap(),
    )
    .unwrap();
    let casm: CompiledClass = serde_json::from_reader(
        std::fs::File::open(env!("CARGO_MANIFEST_DIR").to_owned() + "/" + contract_build_casm)
            .unwrap(),
    )
    .unwrap();
    let compiled_class_hash = casm.class_hash().unwrap();
    let declare_tx = client
        .declare(
            sierra.clone().flatten().unwrap().into(),
            compiled_class_hash,
        )
        .max_fee(FieldElement::from_hex_be("0x100000").unwrap());
    let _result = declare_tx.send().await.unwrap();
    let class_hash = sierra.class_hash().unwrap();
    let _compiled_class_hash = compiled_class_hash;

    let contract_factory = ContractFactory::new(class_hash, client);

    let deploy_tx = &contract_factory.deploy(constructor_args, FieldElement::ZERO, true);

    let deployed_address = deploy_tx.deployed_address();
    deploy_tx.send().await.expect("Unable to deploy contract");

    deployed_address
}
