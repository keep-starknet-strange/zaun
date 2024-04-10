use async_trait::async_trait;
use ethers::{
    abi::AbiEncode,
    contract::{ContractError, EthAbiCodec, EthAbiType},
    prelude::abigen,
    providers::Middleware,
    types::{Address, Bytes, TransactionReceipt, I256, U256},
};

use utils::errors::Error;

abigen!(
    ProxySupport,
    r#"[
        function isFrozen() external view virtual returns (bool)
        function initialize(bytes calldata data) external notCalledDirectly
    ]"#,
);

#[async_trait]
pub trait ProxySupportTrait<M: Middleware> {
    async fn is_frozen(&self) -> Result<bool, Error<M>>;
    async fn initialize(&self, data: Bytes) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn initialize_with<const N: usize>(
        &self,
        data: ProxyInitializeData<N>,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
}

#[async_trait]
impl<T, M: Middleware> ProxySupportTrait<M> for T
where
    T: AsRef<ProxySupport<M>> + Send + Sync,
{
    async fn is_frozen(&self) -> Result<bool, Error<M>> {
        self.as_ref().is_frozen().call().await.map_err(Into::into)
    }

    async fn initialize(&self, data: Bytes) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .initialize(data)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn initialize_with<const N: usize>(
        &self,
        data: ProxyInitializeData<N>,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.initialize(data.into()).await
    }
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
