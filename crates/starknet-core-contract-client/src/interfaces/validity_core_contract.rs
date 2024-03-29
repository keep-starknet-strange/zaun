use async_trait::async_trait;

use crate::Error;

use alloy::{
    network::Ethereum,
    primitives::{U256, I256},
    providers::Provider,
    rpc::types::eth::TransactionReceipt,
    sol,
};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    interface StarknetValidityContract {
        function setProgramHash(uint256 newProgramHash) external notFinalized onlyGovernance;
        function setConfigHash(uint256 newConfigHash) external notFinalized onlyGovernance;
        function setMessageCancellationDelay(uint256 delayInSeconds) external notFinalized onlyGovernance;

        function programHash() public view returns (uint256);
        function configHash() public view returns (uint256);

        function identify() external pure override returns (string memory);
        function stateRoot() external view returns (uint256);
        function stateBlockNumber() external view returns (int256);
        function stateBlockHash() external view returns (uint256);

        function updateState(uint256[] calldata programOutput, uint256 onchainDataHash, uint256 onchainDataSize) external onlyOperator;
    }
);

#[async_trait]
pub trait StarknetValidityContractTrait<P: Provider<Ethereum>> {
    async fn set_program_hash(
        &self,
        new_program_hash: U256,
    ) -> Result<Option<TransactionReceipt>, Error<P>>;
    async fn set_config_hash(
        &self,
        new_config_hash: U256,
    ) -> Result<Option<TransactionReceipt>, Error<P>>;
    async fn set_message_cancellation_delay(
        &self,
        delay_in_seconds: U256,
    ) -> Result<Option<TransactionReceipt>, Error<P>>;

    async fn program_hash(&self) -> Result<U256, Error<P>>;
    async fn config_hash(&self) -> Result<U256, Error<P>>;

    async fn identify(&self) -> Result<String, Error<P>>;
    async fn state_root(&self) -> Result<U256, Error<P>>;
    async fn state_block_number(&self) -> Result<I256, Error<P>>;
    async fn state_block_hash(&self) -> Result<U256, Error<P>>;
    /// Update the L1 state
    async fn update_state(
        &self,
        program_output: Vec<U256>,
        onchain_data_hash: U256,
        onchain_data_size: U256,
    ) -> Result<Option<TransactionReceipt>, Error<P>>; 
}

#[async_trait]
impl<T, P: Provider<Ethereum>> StarknetValidityContractTrait<P> for T
where
    T: AsRef<StarknetValidityContract> + Send + Sync,
{
    async fn set_program_hash(
        &self,
        new_program_hash: U256,
    ) -> Result<Option<TransactionReceipt>, Error<P>> {
        self
            .set_program_hash(new_program_hash)
            .await
            .map_err(Into::into)
    }

    async fn set_config_hash(
        &self,
        new_config_hash: U256,
    ) -> Result<Option<TransactionReceipt>, Error<P>> {
        self
            .set_config_hash(new_config_hash)
            .await
            .map_err(Into::into)
    }

    async fn set_message_cancellation_delay(
        &self,
        delay_in_seconds: U256,
    ) -> Result<Option<TransactionReceipt>, Error<P>> {
        self
            .set_message_cancellation_delay(delay_in_seconds)
            .await
            .map_err(Into::into)
    }

    async fn program_hash(&self) -> Result<U256, Error<P>> {
        self
            .program_hash()
            .await
            .map_err(Into::into)
    }

    async fn config_hash(&self) -> Result<U256, Error<P>> {
        self.config_hash().await.map_err(Into::into)
    }

    async fn identify(&self) -> Result<String, Error<P>> {
        self.identify().await.map_err(Into::into)
    }

    async fn state_root(&self) -> Result<U256, Error<P>> {
        self.state_root().await.map_err(Into::into)
    }

    async fn state_block_number(&self) -> Result<I256, Error<P>> {
        self
            .state_block_number()
            .await
            .map_err(Into::into)
    }

    async fn state_block_hash(&self) -> Result<U256, Error<P>> {
        self
            .state_block_hash()
            .await
            .map_err(Into::into)
    }

    async fn update_state(
        &self,
        program_output: Vec<U256>,
        onchain_data_hash: U256,
        onchain_data_size: U256,
    ) -> Result<Option<TransactionReceipt>, Error<P>> {
        self
            .update_state(program_output, onchain_data_hash, onchain_data_size)
            .await
            .map_err(Into::into)
    }
}
