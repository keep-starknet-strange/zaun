use async_trait::async_trait;
use ethers::{
    contract::ContractError,
    prelude::abigen,
    providers::Middleware,
    types::{TransactionReceipt, H160, U256},
};

use utils::errors::Error;

type Address = H160;

abigen!(
    StarknetTokenBridge,
    "../../../artifacts/starkgate-contracts/StarknetTokenBridge.json",
);

#[async_trait]
pub trait StarknetTokenBridgeTrait<M: Middleware> {
    async fn deposit(
        &self,
        token: Address,
        amount: U256,
        l2_recipient: U256,
        fee: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn withdraw(
        &self,
        token: Address,
        amount: U256,
        recipient: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn set_l2_token_bridge(
        &self,
        l2_token_bridge: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;

    async fn enable_withdrawal_limit(
        &self,
        address: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn disable_withdrawal_limit(
        &self,
        address: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn set_max_total_balance(
        &self,
        token: Address,
        max_total_balance: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;

    async fn identify(&self) -> Result<String, Error<M>>;
    async fn estimate_deposit_fee_wei(&self) -> Result<U256, Error<M>>;
    async fn estimate_enrollment_fee_wei(&self) -> Result<U256, Error<M>>;
    async fn is_servicing_token(&self, token: Address) -> Result<bool, Error<M>>;
    async fn get_remaining_intraday_allowance(&self, token: Address) -> Result<U256, Error<M>>;
    async fn get_max_total_balance(&self, token: Address) -> Result<U256, Error<M>>;
    async fn max_deposit(&self) -> Result<U256, Error<M>>;
}

#[async_trait]
impl<T, M: Middleware> StarknetTokenBridgeTrait<M> for T
where
    T: AsRef<StarknetTokenBridge<M>> + Send + Sync,
{
    async fn deposit(
        &self,
        token: Address,
        amount: U256,
        l2_recipient: U256,
        fee: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .deposit(token, amount, l2_recipient)
            .value(fee)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn withdraw(
        &self,
        token: Address,
        amount: U256,
        _recipient: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .withdraw(token, amount)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn set_l2_token_bridge(
        &self,
        l2_token_bridge: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .set_l2_token_bridge(l2_token_bridge)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn enable_withdrawal_limit(
        &self,
        token: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .enable_withdrawal_limit(token)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn disable_withdrawal_limit(
        &self,
        token: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .disable_withdrawal_limit(token)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn set_max_total_balance(
        &self,
        token: Address,
        max_total_balance: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .set_max_total_balance(token, max_total_balance)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn identify(&self) -> Result<String, Error<M>> {
        self.as_ref().identify().call().await.map_err(Into::into)
    }

    async fn estimate_deposit_fee_wei(&self) -> Result<U256, Error<M>> {
        self.as_ref()
            .estimate_deposit_fee_wei()
            .call()
            .await
            .map_err(Into::into)
    }

    async fn estimate_enrollment_fee_wei(&self) -> Result<U256, Error<M>> {
        self.as_ref()
            .estimate_enrollment_fee_wei()
            .call()
            .await
            .map_err(Into::into)
    }

    async fn is_servicing_token(&self, token: Address) -> Result<bool, Error<M>> {
        self.as_ref()
            .is_servicing_token(token)
            .call()
            .await
            .map_err(Into::into)
    }

    async fn get_remaining_intraday_allowance(&self, token: Address) -> Result<U256, Error<M>> {
        self.as_ref()
            .get_remaining_intraday_allowance(token)
            .call()
            .await
            .map_err(Into::into)
    }

    async fn get_max_total_balance(&self, token: Address) -> Result<U256, Error<M>> {
        self.as_ref()
            .get_max_total_balance(token)
            .call()
            .await
            .map_err(Into::into)
    }

    async fn max_deposit(&self) -> Result<U256, Error<M>> {
        self.as_ref().max_deposit().call().await.map_err(Into::into)
    }
}
