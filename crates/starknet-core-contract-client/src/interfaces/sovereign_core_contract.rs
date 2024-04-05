use std::sync::Arc;

use async_trait::async_trait;

use crate::{LocalWalletSignerMiddleware};

use alloy::{
    network::Ethereum,
    primitives::{I256, U256},
    providers::Provider,
    rpc::types::eth::TransactionReceipt,
    sol, transports::http::Http,
    contract::Error
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
    ) -> Result<Option<TransactionReceipt>, Error>;
    async fn set_config_hash(
        &self,
        new_config_hash: U256,
    ) -> Result<Option<TransactionReceipt>, Error>;
    async fn set_message_cancellation_delay(
        &self,
        delay_in_seconds: U256,
    ) -> Result<Option<TransactionReceipt>, Error>;

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
    ) -> Result<Option<TransactionReceipt>, Error>;
}

#[async_trait]
impl<T> StarknetSovereignContractTrait for T
where
    T: AsRef<StarknetSovereignContract::StarknetSovereignContractInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>> + Send + Sync,
{
    async fn set_program_hash(
        &self,
        new_program_hash: U256,
    ) -> Result<Option<TransactionReceipt>, Error> {
        self
            .set_program_hash(new_program_hash)
            .await
            .map_err(Into::into)
    }

    async fn set_config_hash(
        &self,
        new_config_hash: U256,
    ) -> Result<Option<TransactionReceipt>, Error> {
        self
            .set_config_hash(new_config_hash)
            .await
            .map_err(Into::into)
    }

    async fn set_message_cancellation_delay(
        &self,
        delay_in_seconds: U256,
    ) -> Result<Option<TransactionReceipt>, Error> {
        self
            .set_message_cancellation_delay(delay_in_seconds)
            .await
            .map_err(Into::into)
    }

    async fn program_hash(&self) -> Result<U256, Error> {
        self
            .program_hash()
            .await
            .map_err(Into::into)
    }

    async fn config_hash(&self) -> Result<U256, Error> {
        self.config_hash().await.map_err(Into::into)
    }

    async fn identify(&self) -> Result<String, Error> {
        self.identify().await.map_err(Into::into)
    }

    async fn state_root(&self) -> Result<U256, Error> {
        self.state_root().await.map_err(Into::into)
    }

    async fn state_block_number(&self) -> Result<I256, Error> {
        self
            .state_block_number()
            .await
            .map_err(Into::into)
    }

    async fn state_block_hash(&self) -> Result<U256, Error> {
        self
            .state_block_hash()
            .await
            .map_err(Into::into)
    }

    async fn update_state(
        &self,
        program_output: Vec<U256>,
    ) -> Result<Option<TransactionReceipt>, Error> {
        self
            .update_state(program_output)
            .await
            .map_err(Into::into)
    }
}
