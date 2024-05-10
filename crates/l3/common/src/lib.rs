pub mod errors;
use starknet_accounts::{Account, Call, ConnectedAccount, Execution, SingleOwnerAccount};
use starknet_core::types::{BlockId, BlockTag, FunctionCall};
use starknet_ff::FieldElement;
use starknet_providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet_providers::Provider;
use starknet_signers::LocalWallet;
use std::sync::Arc;
use starknet_contract::ContractFactory;
use starknet_core::types::contract::{CompiledClass, SierraClass};
use errors::Error;

use starknet_core::utils::get_selector_from_name;

pub type LocalWalletSignerMiddleware =
    SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>;

type RpcAccount<'a> = SingleOwnerAccount<&'a JsonRpcClient<HttpTransport>, LocalWallet>;
pub type TransactionExecution<'a> = Execution<'a, RpcAccount<'a>>;

pub const NO_CONSTRUCTOR_ARG: Vec<FieldElement> = Vec::new();

pub trait StarknetContractClient {
    fn address(&self) -> FieldElement;
    fn client(&self) -> LocalWalletSignerMiddleware;
}




pub async fn invoke_contract<'a>(
    client: &'a Arc<LocalWalletSignerMiddleware>,
    address: FieldElement,
    method: &str,
    calldata: Vec<FieldElement>,
) -> Execution<'a, LocalWalletSignerMiddleware> {
    let call = Call {
        to: address,
        selector: get_selector_from_name(method.into()).unwrap(),
        calldata: calldata,
    };
    client.as_ref().execute(vec![call])
}

// pub async fn call_contract(
//     client: Arc<LocalWalletSignerMiddleware>,
//     address: FieldElement,
//     method: &str,
// ) -> Option<Vec<FieldElement>> {
//     let function_call = FunctionCall {
//         contract_address: address,
//         entry_point_selector: get_selector_from_name(method.into()).unwrap(),
//         calldata: vec![],
//     };
//     let provider = client.provider();
//     match provider
//         .call(function_call, BlockId::Tag(BlockTag::Latest))
//         .await
//     {
//         Ok(result) => Some(result),
//         Err(_) => None,
//     }
// }

pub async fn call_contract(
    client: &LocalWalletSignerMiddleware,
    address: FieldElement,
    method: &str,
) -> Result<Vec<FieldElement>, Error> {
    let function_call = FunctionCall {
        contract_address: address,
        entry_point_selector: get_selector_from_name(method.into()).unwrap(),
        calldata: vec![],
    };
    let provider = client.provider();
    provider
        .call(function_call, BlockId::Tag(BlockTag::Latest))
        .await
        .map_err(|e| Error::CustomError(format!("Failed to call contract: {}", e)))
        .map(|result| result)
}

pub async fn deploy_contract(
    client: Arc<LocalWalletSignerMiddleware>,
    path_to_sierra: &str,
    path_to_casm: &str,
    constructor_args: Vec<FieldElement>,
) -> Result<FieldElement, Error> {
    let sierra_file = std::fs::File::open(path_to_sierra)
        .map_err(|e| Error::CustomError(format!("Failed to open Sierra file: {}", e)))?;
    let sierra: SierraClass = serde_json::from_reader(sierra_file)?;
    let casm_file = std::fs::File::open(path_to_casm)
        .map_err(|e| Error::CustomError(format!("Failed to open CASM file: {}", e)))?;
    let casm: CompiledClass = serde_json::from_reader(casm_file)?;
    let compiled_class_hash = casm.class_hash()
        .map_err(|e| Error::CustomError(format!("Failed to get class hash from CASM: {}", e)))?;
    let declare_tx = client
        .declare(
            sierra.clone().flatten()
                .map_err(|e| Error::CustomError(format!("Failed to flatten Sierra class: {}", e)))?.into(),
            compiled_class_hash,
        );            
    declare_tx.send().await
        .map_err(|e| Error::CustomError(format!("Failed to send declare transaction: {}", e)))?;
    let class_hash = sierra.class_hash()
        .map_err(|e| Error::CustomError(format!("Failed to get class hash from Sierra: {}", e)))?;

    let contract_factory = ContractFactory::new(class_hash, client);

    let deploy_tx = &contract_factory.deploy(constructor_args, FieldElement::ZERO, true);

    let deployed_address = deploy_tx.deployed_address();
    deploy_tx.send().await
        .map_err(|e| Error::CustomError(format!("Unable to deploy contract: {}", e)))?;
    Ok(deployed_address)
}
