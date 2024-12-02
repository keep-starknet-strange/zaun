use async_trait::async_trait;
use ethers::contract::ContractError;
use ethers::middleware::Middleware;
use ethers::prelude::{abigen, Bytes};
use ethers::types::{TransactionReceipt, I256, U256};

use utils::errors::Error;

abigen!(
    StarknetCoreContract,
    "../../../artifacts/cairo-lang/Starknet.json",
);

#[async_trait]
pub trait StarknetCoreContractTrait<M: Middleware> {
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
    /// Update the L1 state using calldata
    async fn update_state(
        &self,
        program_output: Vec<U256>,
        onchain_data_hash: U256,
        onchain_data_size: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    /// Update the L1 state using blob and kzg
    async fn update_state_kzg_da(
        &self,
        program_output: Vec<U256>,
        kzg_hashes: Vec<Bytes>,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
}

pub struct StandardCoreContract<M: Middleware>(pub StarknetCoreContract<M>);

#[async_trait]
impl<M: Middleware> StarknetCoreContractTrait<M> for StandardCoreContract<M> {
    async fn set_program_hash(
        &self,
        new_program_hash: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.0
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
        self.0
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
        self.0
            .set_message_cancellation_delay(delay_in_seconds)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn program_hash(&self) -> Result<U256, Error<M>> {
        self.0.program_hash().call().await.map_err(Into::into)
    }

    async fn config_hash(&self) -> Result<U256, Error<M>> {
        self.0.config_hash().call().await.map_err(Into::into)
    }

    async fn identify(&self) -> Result<String, Error<M>> {
        self.0.identify().call().await.map_err(Into::into)
    }

    async fn state_root(&self) -> Result<U256, Error<M>> {
        self.0.state_root().call().await.map_err(Into::into)
    }

    async fn state_block_number(&self) -> Result<I256, Error<M>> {
        self.0.state_block_number().call().await.map_err(Into::into)
    }

    async fn state_block_hash(&self) -> Result<U256, Error<M>> {
        self.0.state_block_hash().call().await.map_err(Into::into)
    }

    async fn update_state(
        &self,
        program_output: Vec<U256>,
        onchain_data_hash: U256,
        onchain_data_size: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.0
            .update_state(program_output, onchain_data_hash, onchain_data_size)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn update_state_kzg_da(
        &self,
        program_output: Vec<U256>,
        kzg_hashes: Vec<Bytes>,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.0
            .update_state_kzg_da(program_output, kzg_hashes)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }
}
