pub mod errors;
use starknet_accounts::{Account, Call, ConnectedAccount, Execution, SingleOwnerAccount};
use starknet_core::types::{BlockId, BlockTag, FunctionCall};
use starknet_ff::FieldElement;
use starknet_providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet_providers::Provider;
use starknet_signers::LocalWallet;
use std::sync::Arc;

use starknet_core::utils::get_selector_from_name;

pub type LocalWalletSignerMiddleware =
    SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>;
pub const NO_CONSTRUCTOR_ARG: Vec<FieldElement> = Vec::new();

pub trait StarknetContractClientL3 {
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

pub async fn call_contract(
    client: Arc<LocalWalletSignerMiddleware>,
    address: FieldElement,
    method: &str,
) -> Option<Vec<FieldElement>> {
    let function_call = FunctionCall {
        contract_address: address,
        entry_point_selector: get_selector_from_name(method.into()).unwrap(),
        calldata: vec![],
    };
    let provider = client.provider();
    match provider
        .call(function_call, BlockId::Tag(BlockTag::Latest))
        .await
    {
        Ok(result) => Some(result),
        Err(_) => None,
    }
}
