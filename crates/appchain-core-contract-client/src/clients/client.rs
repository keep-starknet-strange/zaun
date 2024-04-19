use starknet::{
    core::types::FieldElement,
    providers::
        jsonrpc::{HttpTransport, JsonRpcClient},
        providers, signers
};
use starknet::accounts::{Account, ExecutionEncoding, SingleOwnerAccount};

pub struct TestTokenClient {
    pub provider: JsonRpcClient<HttpTransport>,
    pub contract_address: FieldElement,
}

impl TestTokenClient {
    pub fn new(provider: JsonRpcClient<HttpTransport>, contract_address: FieldElement) -> Self {
        Self { provider, contract_address }
    }
}
