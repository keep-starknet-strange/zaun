use async_trait::async_trait;
use starknet::{
    providers::jsonrpc::{HttpTransport, JsonRpcClient},
    core::{
        types::{BlockId, BlockTag, FieldElement, FunctionCall},
        utils::get_selector_from_name,
    },
    providers::Provider,
};

pub struct TestTokenClient {
    pub provider: JsonRpcClient<HttpTransport>,
    pub contract_address: FieldElement,
}

impl TestTokenClient {
    pub fn new(provider: JsonRpcClient<HttpTransport>, contract_address: FieldElement) -> Self {
        Self { provider, contract_address }
    }
}

#[async_trait]
pub trait TestTokenContractTrait {
    async fn register_operator(
        &self,
        operator: FieldElement,
    ) -> Result<Vec<FieldElement>, Box<dyn std::error::Error>>;
}

#[async_trait]
impl TestTokenContractTrait for TestTokenClient {
    async fn register_operator(
        &self,
        operator: FieldElement,
    ) -> Result<Vec<FieldElement>, Box<dyn std::error::Error>> {
        let call_result = self.provider.call(
            FunctionCall {
                contract_address: self.contract_address,
                entry_point_selector: get_selector_from_name("register_operator").unwrap(),
                calldata: vec![operator],
            },
            BlockId::Tag(BlockTag::Latest),
        ).await?;

        Ok(call_result)
    }
}