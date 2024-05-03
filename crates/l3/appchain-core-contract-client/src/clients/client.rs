use starknet::accounts::SingleOwnerAccount;
use starknet_ff::FieldElement;
use std::sync::Arc;
use crate::interfaces::Operator;

use common::LocalWalletSignerMiddleware;


pub struct StarknetCoreContractClient {
    operator: Operator<LocalWalletSignerMiddleware>,
}

impl StarknetCoreContractClient {
    pub fn new(address: FieldElement, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self {
            operator: Operator::new(address, client.clone()),
        }
    }
}   

impl AsRef<Operator<LocalWalletSignerMiddleware>> for StarknetCoreContractClient {
    fn as_ref(&self) -> &Operator<LocalWalletSignerMiddleware> {
        &self.operator
    }
}