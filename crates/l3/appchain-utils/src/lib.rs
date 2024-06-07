pub mod errors;

use color_eyre::{eyre::eyre, Result};
use starknet_accounts::{Account, Call, Execution, SingleOwnerAccount};
use starknet_contract::ContractFactory;
use starknet_core::types::contract::{CompiledClass, SierraClass};
use starknet_core::types::{BlockId, BlockTag, FunctionCall, InvokeTransactionResult};
use starknet_core::utils::get_selector_from_name;
use starknet_ff::FieldElement;
use starknet_providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet_providers::Provider;
use starknet_signers::LocalWallet;
use std::path::Path;

pub type LocalWalletSignerMiddleware =
    SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>;

type RpcAccount<'a> = SingleOwnerAccount<&'a JsonRpcClient<HttpTransport>, LocalWallet>;
pub type TransactionExecution<'a> = Execution<'a, RpcAccount<'a>>;

pub const NO_CONSTRUCTOR_ARG: Vec<FieldElement> = Vec::new();

// Montgomery representation for value 0x1000000000000
pub const MAX_FEE: FieldElement = FieldElement::from_mont([
    18437736874454810625,
    18446744073709551615,
    18446744073709551615,
    423338364972826640,
]);

pub trait StarknetContractClient {
    fn address(&self) -> FieldElement;
    fn signer(&self) -> LocalWalletSignerMiddleware;
}

pub async fn invoke_contract(
    signer: &LocalWalletSignerMiddleware,
    address: FieldElement,
    method: &str,
    calldata: Vec<FieldElement>,
) -> Result<InvokeTransactionResult> {
    let selector = get_selector_from_name(method)
        .map_err(|e| eyre!("Invalid selector for {}: {}", method, e))?;
    let call = Call {
        to: address,
        selector,
        calldata,
    };
    signer
        .execute(vec![call])
        .max_fee(MAX_FEE)
        .send()
        .await
        .map_err(|e| eyre!("Failed to send transaction: {}", e))
}

pub async fn call_contract(
    provider: &JsonRpcClient<HttpTransport>,
    address: FieldElement,
    method: &str,
    calldata: Vec<FieldElement>,
) -> Result<Vec<FieldElement>> {
    let entry_point_selector = get_selector_from_name(method)
        .map_err(|e| eyre!("Invalid selector for {}: {}", method, e))?;
    let function_call = FunctionCall {
        contract_address: address,
        entry_point_selector,
        calldata,
    };
    provider
        .call(function_call, BlockId::Tag(BlockTag::Latest))
        .await
        .map_err(|e| eyre!("Provider error: {}", e))
}

pub async fn deploy_contract<'a>(
    signer: &'a LocalWalletSignerMiddleware,
    path_to_sierra: &Path,
    path_to_casm: &Path,
    constructor_args: Vec<FieldElement>,
) -> Result<FieldElement> {
    let sierra: SierraClass = {
        let sierra_file = std::fs::File::open(path_to_sierra)
            .map_err(|e| eyre!("Failed to open Sierra file: {}", e))?;
        serde_json::from_reader(sierra_file)?
    };
    let casm: CompiledClass = {
        let casm_file = std::fs::File::open(path_to_casm)
            .map_err(|e| eyre!("Failed to open CASM file: {}", e))?;
        serde_json::from_reader(casm_file)?
    };
    let compiled_class_hash = casm
        .class_hash()
        .map_err(|e| eyre!("Failed to get class hash from CASM: {}", e))?;
    let declare_tx = signer.declare(
        sierra
            .clone()
            .flatten()
            .map_err(|e| eyre!("Failed to flatten Sierra class: {}", e))?
            .into(),
        compiled_class_hash,
    );
    declare_tx
        .send()
        .await
        .map_err(|e| eyre!("Failed to send declare transaction: {}", e))?;
    let class_hash = sierra
        .class_hash()
        .map_err(|e| eyre!("Failed to get class hash from Sierra: {}", e))?;

    let contract_factory = ContractFactory::new(class_hash, signer);

    let deploy_tx = contract_factory.deploy(constructor_args, FieldElement::ZERO, true);

    let deployed_address = deploy_tx.deployed_address();
    deploy_tx
        .send()
        .await
        .map_err(|e| eyre!("Unable to deploy contract: {}", e))?;
    Ok(deployed_address)
}
