use async_trait::async_trait;
use ethers::contract::ContractError;
use ethers::middleware::Middleware;
use ethers::prelude::abigen;
use ethers::types::{Bytes, TransactionReceipt, I256, U256};

use crate::interfaces::core_contract::StarknetCoreContractTrait;
use utils::errors::Error;

abigen!(
    StarknetDevCoreContract,
    "../../../artifacts/StarknetDevCoreContract.json",
);

pub struct DevCoreContract<M: Middleware>(pub StarknetDevCoreContract<M>);

#[allow(dead_code)]
impl<M: Middleware> DevCoreContract<M> {
    async fn update_state_override(
        &self,
        global_root: U256,
        block_number: I256,
        block_hash: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.0
            .update_state_override(global_root, block_number, block_hash)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }
}

#[async_trait]
impl<M: Middleware> StarknetCoreContractTrait<M> for DevCoreContract<M> {
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
