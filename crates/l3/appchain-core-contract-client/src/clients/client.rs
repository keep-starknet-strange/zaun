use crate::interfaces::core_contract::CoreContract;
use crate::interfaces::messaging::Messaging;
use crate::interfaces::operator::Operator;
use starknet_ff::FieldElement;
use std::sync::Arc;

use common::LocalWalletSignerMiddleware;

pub struct StarknetCoreContractClient {
    operator: Operator,
    messaging: Messaging,
    core_contract: CoreContract,
}

impl StarknetCoreContractClient {
    pub fn new(address: FieldElement, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self {
            operator: Operator::new(address, client.clone()),
            messaging: Messaging::new(address, client.clone()),
            core_contract: CoreContract::new(address, client.clone()),
        }
    }
}

impl AsRef<Operator> for StarknetCoreContractClient {
    fn as_ref(&self) -> &Operator {
        &self.operator
    }
}

impl AsRef<Messaging> for StarknetCoreContractClient {
    fn as_ref(&self) -> &Messaging {
        &self.messaging
    }
}

impl AsRef<CoreContract> for StarknetCoreContractClient {
    fn as_ref(&self) -> &CoreContract {
        &self.core_contract
    }
}
