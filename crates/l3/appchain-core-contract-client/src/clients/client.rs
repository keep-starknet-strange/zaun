use crate::interfaces::Operator;
use starknet_ff::FieldElement;
use std::sync::Arc;

use common::LocalWalletSignerMiddleware;

pub struct StarknetCoreContractClient {
    operator: Operator,
}

impl StarknetCoreContractClient {
    pub fn new(address: FieldElement, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self {
            operator: Operator::new(address, client.clone()),
        }
    }
}

impl AsRef<Operator> for StarknetCoreContractClient {
    fn as_ref(&self) -> &Operator {
        &self.operator
    }
}
