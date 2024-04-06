use std::sync::Arc;

use async_trait::async_trait;

use crate::LocalWalletSignerMiddleware;

use alloy::{
    contract::Error, network::Ethereum, primitives::{Address, Bytes, I256, U256}, providers::Provider, rpc::types::eth::TransactionReceipt, sol, sol_types::SolValue, transports::{http::Http, RpcError, TransportErrorKind}
};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    interface ProxySupport {
        function isFrozen() external view virtual returns (bool);
        function initialize(bytes calldata data) external notCalledDirectly;
    }
);

#[async_trait]
pub trait ProxySupportTrait
{
    async fn is_frozen(&self) -> Result<bool, Error>;
    async fn initialize(&self, data: Bytes) -> Result<TransactionReceipt, RpcError<TransportErrorKind>>;
}

#[async_trait]
impl<T> ProxySupportTrait for T
where
    T: AsRef<ProxySupport::ProxySupportInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>> + Send + Sync,
{
    async fn is_frozen(&self) -> Result<bool, Error> {
        Ok(self.as_ref().isFrozen().call().await?._0)
    }

    async fn initialize(&self, data: Bytes) -> Result<TransactionReceipt, RpcError<TransportErrorKind>> {
        let base_fee = self.as_ref().provider().as_ref().get_gas_price().await.unwrap();
        let builder = self.as_ref().initialize(data);
        let gas = builder.estimate_gas().await.unwrap();
        builder
            .from(self.as_ref().provider().as_ref().get_accounts().await.unwrap()[0])
            .nonce(2)
            .gas(gas)
            .gas_price(base_fee)
            .send()
            .await.unwrap()
            .get_receipt()
            .await
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CoreContractState {
    pub state_root: U256,
    pub block_number: I256,
    pub block_hash: U256,
}

#[derive(Debug, Clone, Default, PartialEq)]
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
            self.sub_contract_addresses.abi_encode(),
            self.eic_address.abi_encode(),
            self.init_data.program_hash.abi_encode(),
            self.init_data.verifier_address.abi_encode(),
            self.init_data.config_hash.abi_encode(),
            self.init_data.initial_state.state_root.abi_encode(),
            self.init_data.initial_state.block_number.abi_encode(),
            self.init_data.initial_state.block_hash.abi_encode(),
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
    fn test_calldata_encoding() {
        let calldata = ProxyInitializeData::<0> {
            sub_contract_addresses: [],
            eic_address: Default::default(),
            init_data: Default::default(),
        };
        let bytes: Vec<u8> = calldata.into();
        assert_eq!(bytes, [0u8; 7 * 32].to_vec());
    }
}
