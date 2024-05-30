use crate::interfaces::core_contract::CoreContract;
use crate::interfaces::messaging::Messaging;
use crate::interfaces::operator::Operator;
use starknet_ff::FieldElement;
use std::sync::Arc;

use appchain_utils::LocalWalletSignerMiddleware;

pub struct StarknetCoreContractClient<'a> {
    operator: Operator<'a>,
    messaging: Messaging<'a>,
    core_contract: CoreContract<'a>,
}

impl<'a> StarknetCoreContractClient<'a> {
    pub fn new(address: FieldElement, client: Arc<&'a LocalWalletSignerMiddleware>) -> Self {
        Self {
            operator: Operator::new(address, client.clone()),
            messaging: Messaging::new(address, client.clone()),
            core_contract: CoreContract::new(address, client.clone()),
        }
    }
}

impl<'a> AsRef<Operator<'a>> for StarknetCoreContractClient<'a> {
    fn as_ref(&self) -> &Operator<'a> {
        &self.operator
    }
}

impl<'a> AsRef<Messaging<'a>> for StarknetCoreContractClient<'a> {
    fn as_ref(&self) -> &Messaging<'a> {
        &self.messaging
    }
}

impl<'a> AsRef<CoreContract<'a>> for StarknetCoreContractClient<'a> {
    fn as_ref(&self) -> &CoreContract<'a> {
        &self.core_contract
    }
}
