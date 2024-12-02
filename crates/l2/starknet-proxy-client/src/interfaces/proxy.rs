use async_trait::async_trait;
use ethers::abi::AbiEncode;
use ethers::addressbook::Address;
use ethers::contract::{EthAbiCodec, EthAbiType};
use ethers::middleware::Middleware;
use ethers::prelude::{Bytes, TransactionReceipt, I256, U256};
use utils::errors::Error;

#[async_trait]
pub trait ProxySupport3_0_2Trait<M: Middleware> {
    async fn initialize(&self, data: Bytes) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn initialize_with<const N: usize>(
        &self,
        data: ProxyInitializeData<N>,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn upgrade_to(
        &self,
        data: Bytes,
        implementation_address: Address,
        finalized: bool,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn add_implementation(
        &self,
        data: Bytes,
        implementation_address: Address,
        finalized: bool,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn proxy_nominate_new_governor(
        &self,
        new_governor: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn proxy_remove_governance(
        &self,
        governor: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn proxy_accept_governance(&self) -> Result<Option<TransactionReceipt>, Error<M>>;
}

#[async_trait]
pub trait ProxySupport5_0_0Trait<M: Middleware> {
    async fn initialize(&self, data: Bytes) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn initialize_with<const N: usize>(
        &self,
        data: ProxyInitializeData<N>,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn upgrade_to(
        &self,
        data: Bytes,
        implementation_address: Address,
        finalized: bool,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn add_implementation(
        &self,
        data: Bytes,
        implementation_address: Address,
        finalized: bool,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn proxy_nominate_new_governor(
        &self,
        new_governor: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn proxy_remove_governance(
        &self,
        governor: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn register_app_governor(
        &self,
        account: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn register_app_role_admin(
        &self,
        account: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn register_governance_admin(
        &self,
        account: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn register_operator(
        &self,
        account: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn register_security_admin(
        &self,
        account: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn register_security_agent(
        &self,
        account: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn register_token_admin(
        &self,
        account: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn register_upgrade_governor(
        &self,
        account: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
}

#[derive(Debug, Clone, Default, PartialEq, EthAbiType, EthAbiCodec)]
pub struct CoreContractState {
    pub state_root: U256,
    pub block_number: I256,
    pub block_hash: U256,
}

#[derive(Debug, Clone, Default, PartialEq, EthAbiType, EthAbiCodec)]
pub struct CoreContractInitData {
    pub program_hash: U256,
    pub aggregate_program_hash: U256,
    pub verifier_address: Address,
    pub config_hash: U256,
    pub initial_state: CoreContractState,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProxyInitializeData<const N: usize> {
    pub sub_contract_addresses: [Address; N],
    pub eic_address: Address,
    pub init_data: CoreContractInitData,
}

#[allow(clippy::from_over_into)]
impl<const N: usize> Into<Vec<u8>> for ProxyInitializeData<N> {
    fn into(self) -> Vec<u8> {
        [
            self.sub_contract_addresses.encode(),
            self.eic_address.encode(),
            self.init_data.encode(),
        ]
        .concat()
    }
}

#[allow(clippy::from_over_into)]
impl Into<Vec<u8>> for CoreContractInitData {
    fn into(self) -> Vec<u8> {
        [
            self.program_hash.encode(),
            self.aggregate_program_hash.encode(),
            self.verifier_address.encode(),
            self.config_hash.encode(),
            self.initial_state.into(),
        ]
        .concat()
    }
}

#[allow(clippy::from_over_into)]
impl Into<Vec<u8>> for CoreContractState {
    fn into(self) -> Vec<u8> {
        [
            self.state_root.encode(),
            self.block_number.encode(),
            self.block_hash.encode(),
        ]
        .concat()
    }
}

#[allow(clippy::from_over_into)]
impl<const N: usize> Into<Bytes> for ProxyInitializeData<N> {
    fn into(self) -> Bytes {
        Into::<Vec<u8>>::into(self).into()
    }
}

#[cfg(test)]
mod tests {
    use super::ProxyInitializeData;

    #[test]
    fn test_initialize_calldata_encoding() {
        let calldata = ProxyInitializeData::<0> {
            sub_contract_addresses: [],
            eic_address: Default::default(),
            init_data: Default::default(),
        };
        let bytes: Vec<u8> = calldata.into();
        assert_eq!(bytes, [0u8; 7 * 32].to_vec());
    }
}
