use async_trait::async_trait;
use ethers::middleware::Middleware;
use ethers::prelude::abigen;
use ethers::types::{TransactionReceipt, I256, U256};

use crate::error::Error;

abigen!(
    StarknetContract,
    r#"[
        function setProgramHash(uint256 newProgramHash) external notFinalized onlyGovernance
        function setConfigHash(uint256 newConfigHash) external notFinalized onlyGovernance
        function setMessageCancellationDelay(uint256 delayInSeconds) external notFinalized onlyGovernance

        function programHash() public view returns (uint256)
        function configHash() public view returns (uint256)

        function identify() external pure override returns (string memory)
        function stateRoot() external view returns (uint256)
        function stateBlockNumber() external view returns (int256)
        function stateBlockHash() external view returns (uint256)
        function updateState(uint256[] calldata programOutput, uint256 onchainDataHash, uint256 onchainDataSize) external onlyOperator

    ]"#,
);

#[async_trait]
pub trait StarknetContractTrait<M: Middleware> {
    async fn state_block_number(&self) -> Result<I256, Error<M>>;
    async fn state_root(&self) -> Result<U256, Error<M>>;
    async fn config_hash(&self) -> Result<U256, Error<M>>;
    async fn program_hash(&self) -> Result<U256, Error<M>>;
    /// Update the L1 state
    ///
    /// Will return `None` if the TX end up being dropped from the mempool without being executed
    async fn update_state(
        &self,
        program_output: Vec<U256>,
        onchain_data_hash: U256,
        onchain_data_size: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
}

#[async_trait]
impl<T, M: Middleware> StarknetContractTrait<M> for T
where
    T: AsRef<StarknetContract<M>> + Send + Sync,
{
    async fn state_block_number(&self) -> Result<I256, Error<M>> {
        self.as_ref()
            .state_block_number()
            .call()
            .await
            .map_err(Into::into)
    }

    async fn state_root(&self) -> Result<U256, Error<M>> {
        self.as_ref().state_root().call().await.map_err(Into::into)
    }

    async fn config_hash(&self) -> Result<U256, Error<M>> {
        self.as_ref().config_hash().call().await.map_err(Into::into)
    }

    async fn program_hash(&self) -> Result<U256, Error<M>> {
        self.as_ref()
            .program_hash()
            .call()
            .await
            .map_err(Into::into)
    }

    async fn update_state(
        &self,
        program_output: Vec<U256>,
        onchain_data_hash: U256,
        onchain_data_size: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .update_state(program_output, onchain_data_hash, onchain_data_size)
            .send()
            .await?
            .await
            .map_err(Into::into)
    }
}
