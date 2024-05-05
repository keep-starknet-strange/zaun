pub mod errors;
use starknet_accounts::{Account, Call, Execution, SingleOwnerAccount};
use starknet_ff::FieldElement;
use starknet_providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet_providers::Provider;
use starknet_signers::{LocalWallet, SigningKey};
use std::sync::Arc;

use starknet_core::utils::get_selector_from_name;

pub type LocalWalletSignerMiddleware =
    SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>;
pub const NO_CONSTRUCTOR_ARG: () = ();

pub trait StarknetContractClient {
    fn address(&self) -> FieldElement;
    fn client(&self) -> LocalWalletSignerMiddleware;
}

pub async fn invoke_contract<'a>(
    client: &'a Arc<LocalWalletSignerMiddleware>,
    // client: LocalWalletSignerMiddleware,
    address: FieldElement,
    method: &str,
    calldata: Vec<FieldElement>,
) -> Execution<'a, LocalWalletSignerMiddleware> {
    // let calls = vec![Call { to: address, selector: get_selector_from_name(method).unwrap(), calldata }];
    // client.execute(calls)
    let call = Call {
        to: address,
        selector: get_selector_from_name(method.into()).unwrap(),
        calldata: calldata,
    };
    client.as_ref().execute(vec![call])
}
