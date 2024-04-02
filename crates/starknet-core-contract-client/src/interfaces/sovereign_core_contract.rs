use async_trait::async_trait;
use ethers::contract::ContractError;
use ethers::middleware::Middleware;
use ethers::prelude::abigen;
use ethers::types::{TransactionReceipt, I256, U256};

use utils::errors::Error;

abigen!(
    StarknetSovereignContract,
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

        function updateState(uint256[] calldata programOutput) external onlyOperator

    ]"#,
);

#[async_trait]
pub trait StarknetSovereignContractTrait<M: Middleware> {
    async fn set_program_hash(
        &self,
        new_program_hash: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn set_config_hash(
        &self,
        new_config_hash: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn set_message_cancellation_delay(
        &self,
        delay_in_seconds: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;

    async fn program_hash(&self) -> Result<U256, Error<M>>;
    async fn config_hash(&self) -> Result<U256, Error<M>>;

    async fn identify(&self) -> Result<String, Error<M>>;
    async fn state_root(&self) -> Result<U256, Error<M>>;
    async fn state_block_number(&self) -> Result<I256, Error<M>>;
    async fn state_block_hash(&self) -> Result<U256, Error<M>>;
    /// Update the L1 state
    async fn update_state(
        &self,
        program_output: Vec<U256>,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
}

#[async_trait]
impl<T, M: Middleware> StarknetSovereignContractTrait<M> for T
where
    T: AsRef<StarknetSovereignContract<M>> + Send + Sync,
{
    async fn set_program_hash(
        &self,
        new_program_hash: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .set_program_hash(new_program_hash)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn set_config_hash(
        &self,
        new_config_hash: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .set_config_hash(new_config_hash)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn set_message_cancellation_delay(
        &self,
        delay_in_seconds: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .set_message_cancellation_delay(delay_in_seconds)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn program_hash(&self) -> Result<U256, Error<M>> {
        self.as_ref()
            .program_hash()
            .call()
            .await
            .map_err(Into::into)
    }

    async fn config_hash(&self) -> Result<U256, Error<M>> {
        self.as_ref().config_hash().call().await.map_err(Into::into)
    }

    async fn identify(&self) -> Result<String, Error<M>> {
        self.as_ref().identify().call().await.map_err(Into::into)
    }

    async fn state_root(&self) -> Result<U256, Error<M>> {
        self.as_ref().state_root().call().await.map_err(Into::into)
    }

    async fn state_block_number(&self) -> Result<I256, Error<M>> {
        self.as_ref()
            .state_block_number()
            .call()
            .await
            .map_err(Into::into)
    }

    async fn state_block_hash(&self) -> Result<U256, Error<M>> {
        self.as_ref()
            .state_block_hash()
            .call()
            .await
            .map_err(Into::into)
    }

    async fn update_state(
        &self,
        program_output: Vec<U256>,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .update_state(program_output)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }
}
