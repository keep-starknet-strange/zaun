use std::sync::Arc;

use async_trait::async_trait;

use crate::LocalWalletSignerMiddleware;

use alloy::{
    contract::Error, network::Ethereum, primitives::{I256, U256}, providers::Provider, rpc::types::eth::TransactionReceipt, sol, transports::{http::Http, RpcError, TransportErrorKind}
};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    interface StarknetSovereignContract {
        function setProgramHash(uint256 newProgramHash) external notFinalized onlyGovernance;
        function setConfigHash(uint256 newConfigHash) external notFinalized onlyGovernance;
        function setMessageCancellationDelay(uint256 delayInSeconds) external notFinalized onlyGovernance;

        function programHash() public view returns (uint256);
        function configHash() public view returns (uint256);

        function identify() external pure override returns (string memory);
        function stateRoot() external view returns (uint256);
        function stateBlockNumber() external view returns (int256);
        function stateBlockHash() external view returns (uint256);

        function updateState(uint256[] calldata programOutput) external onlyOperator;
    }
);

#[async_trait]
pub trait StarknetSovereignContractTrait {
    async fn set_program_hash(
        &self,
        new_program_hash: U256,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>>;
    async fn set_config_hash(
        &self,
        new_config_hash: U256,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>>;
    async fn set_message_cancellation_delay(
        &self,
        delay_in_seconds: U256,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>>;

    async fn program_hash(&self) -> Result<U256, Error>;
    async fn config_hash(&self) -> Result<U256, Error>;

    async fn identify(&self) -> Result<String, Error>;
    async fn state_root(&self) -> Result<U256, Error>;
    async fn state_block_number(&self) -> Result<I256, Error>;
    async fn state_block_hash(&self) -> Result<U256, Error>;
    /// Update the L1 state
    async fn update_state(
        &self,
        program_output: Vec<U256>,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>>;
}

#[async_trait]
impl<T> StarknetSovereignContractTrait for T
where
    T: AsRef<StarknetSovereignContract::StarknetSovereignContractInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>> + Send + Sync,
{
    async fn set_program_hash(
        &self,
        new_program_hash: U256,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>> {
        let base_fee = self.as_ref().provider().as_ref().get_gas_price().await.unwrap();
        let from_address = self.as_ref().provider().as_ref().get_accounts().await.unwrap()[0];
        let gas = self.as_ref().setProgramHash(new_program_hash).from(from_address).estimate_gas().await.unwrap();
        let builder = self.as_ref().setProgramHash(new_program_hash);
        builder
            .from(from_address)
            .nonce(2)
            .gas(gas)
            .gas_price(base_fee)
            .send()
            .await.unwrap()
            .get_receipt()
            .await
    }

    async fn set_config_hash(
        &self,
        new_config_hash: U256,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>> {
        let base_fee = self.as_ref().provider().as_ref().get_gas_price().await.unwrap();
        let from_address = self.as_ref().provider().as_ref().get_accounts().await.unwrap()[0];
        let gas = self.as_ref().setConfigHash(new_config_hash).from(from_address).estimate_gas().await.unwrap();
        let builder = self.as_ref().setConfigHash(new_config_hash);
        builder
            .from(from_address)
            .nonce(2)
            .gas(gas)
            .gas_price(base_fee)
            .send()
            .await.unwrap()
            .get_receipt()
            .await
    }

    async fn set_message_cancellation_delay(
        &self,
        delay_in_seconds: U256,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>> {
        let base_fee = self.as_ref().provider().as_ref().get_gas_price().await.unwrap();
        let from_address = self.as_ref().provider().as_ref().get_accounts().await.unwrap()[0];
        let gas = self.as_ref().setMessageCancellationDelay(delay_in_seconds).from(from_address).estimate_gas().await.unwrap();
        let builder = self.as_ref().setMessageCancellationDelay(delay_in_seconds);
        builder
            .from(from_address)
            .nonce(2)
            .gas(gas)
            .gas_price(base_fee)
            .send()
            .await.unwrap()
            .get_receipt()
            .await
    }

    async fn program_hash(&self) -> Result<U256, Error> {
        Ok(self.as_ref().programHash().call().await?._0)
    }

    async fn config_hash(&self) -> Result<U256, Error> {
        Ok(self.as_ref().configHash().call().await?._0)
    }

    async fn identify(&self) -> Result<String, Error> {
        Ok(self.as_ref().identify().call().await?._0)
    }

    async fn state_root(&self) -> Result<U256, Error> {
        Ok(self.as_ref().stateRoot().call().await?._0)
    }

    async fn state_block_number(&self) -> Result<I256, Error> {
        Ok(self.as_ref().stateBlockNumber().call().await?._0)
    }

    async fn state_block_hash(&self) -> Result<U256, Error> {
        Ok(self.as_ref().stateBlockHash().call().await?._0)
    }

    async fn update_state(
        &self,
        program_output: Vec<U256>,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>> {
        let base_fee = self.as_ref().provider().as_ref().get_gas_price().await.unwrap();
        let from_address = self.as_ref().provider().as_ref().get_accounts().await.unwrap()[0];
        let gas = self.as_ref().updateState(program_output.clone()).from(from_address).estimate_gas().await.unwrap();
        let builder = self.as_ref().updateState(program_output);
        builder
            .from(from_address)
            .nonce(2)
            .gas(gas)
            .gas_price(base_fee)
            .send()
            .await.unwrap()
            .get_receipt()
            .await
    }
}
