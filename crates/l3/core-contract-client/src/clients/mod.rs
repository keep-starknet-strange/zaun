use crate::interfaces::core_contract::CoreContract;
use crate::interfaces::messaging::Messaging;
use crate::interfaces::operator::Operator;
use appchain_utils::LocalWalletSignerMiddleware;
use starknet_ff::FieldElement;

pub struct StarknetCoreContractClient<'a> {
    operator: Operator<'a>,
    messaging: Messaging<'a>,
    core_contract: CoreContract<'a>,
}

impl<'a> StarknetCoreContractClient<'a> {
    pub fn new(address: FieldElement, signer: &'a LocalWalletSignerMiddleware) -> Self {
        Self {
            operator: Operator::new(address, signer),
            messaging: Messaging::new(address, signer),
            core_contract: CoreContract::new(address, signer),
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
