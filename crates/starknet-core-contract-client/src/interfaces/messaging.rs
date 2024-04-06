use std::sync::Arc;

use async_trait::async_trait;

use crate::LocalWalletSignerMiddleware;

use alloy::{
    contract::Error, network::Ethereum, primitives::{FixedBytes, U256}, providers::Provider, rpc::types::eth::TransactionReceipt, sol, transports::{http::Http, RpcError, TransportErrorKind}
};

// StarknetMessaging.sol
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    interface StarknetMessaging {
        function l1ToL2Messages(bytes32 msgHash) external view returns (uint256);
        function l2ToL1Messages(bytes32 msgHash) external view returns (uint256);
        function l1ToL2MessageCancellations(bytes32 msgHash) external view returns (uint256);

        function sendMessageToL2(uint256 toAddress, uint256 selector, uint256[] calldata payload) external payable override returns (bytes32, uint256);
        function consumeMessageFromL2(uint256 fromAddress, uint256[] calldata payload) external override returns (bytes32);

        function startL1ToL2MessageCancellation(uint256 toAddress, uint256 selector, uint256[] calldata payload, uint256 nonce) external override returns (bytes32);
        function cancelL1ToL2Message(uint256 toAddress, uint256 selector, uint256[] calldata payload, uint256 nonce) external override returns (bytes32);
    }
);

#[async_trait]
pub trait StarknetMessagingTrait {
    async fn l1_to_l2_messages(&self, msg_hash: FixedBytes<32>) -> Result<U256, Error>;
    async fn l2_to_l1_messages(&self, msg_hash: FixedBytes<32>) -> Result<U256, Error>;
    async fn l1_to_l2_message_cancellations(&self, msg_hash: FixedBytes<32>)
        -> Result<U256, Error>;
    async fn send_message_to_l2(
        &self,
        to_address: U256,
        selector: U256,
        payload: Vec<U256>,
        fee: U256,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>>;
    async fn start_l1_to_l2_message_cancellation(
        &self,
        to_address: U256,
        selector: U256,
        payload: Vec<U256>,
        nonce: U256,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>>;
    async fn cancel_l1_to_l2_message(
        &self,
        to_address: U256,
        selector: U256,
        payload: Vec<U256>,
        nonce: U256,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>>;
}

#[async_trait]
impl<T> StarknetMessagingTrait for T
where
    T: AsRef<StarknetMessaging::StarknetMessagingInstance<Ethereum, Http<reqwest::Client>, Arc<LocalWalletSignerMiddleware>>> + Send + Sync,
{
    async fn l1_to_l2_messages(&self, msg_hash: FixedBytes<32>) -> Result<U256, Error> {
        Ok(self.as_ref().l1ToL2Messages(msg_hash).call().await?._0)
    }

    async fn l2_to_l1_messages(&self, msg_hash: FixedBytes<32>) -> Result<U256, Error> {
        Ok(self.as_ref().l2ToL1Messages(msg_hash).call().await?._0)
    }

    async fn l1_to_l2_message_cancellations(
        &self,
        msg_hash: FixedBytes<32>,
    ) -> Result<U256, Error> {
        Ok(self.as_ref().l1ToL2MessageCancellations(msg_hash).call().await?._0)
    }

    async fn send_message_to_l2(
        &self,
        to_address: U256,
        selector: U256,
        payload: Vec<U256>,
        fee: U256,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>> {
        let base_fee = self.as_ref().provider().as_ref().get_gas_price().await.unwrap();
        let builder = self.as_ref().sendMessageToL2(to_address, selector, payload);
        let gas = builder.estimate_gas().await.unwrap();
        builder
            .from(self.as_ref().provider().as_ref().get_accounts().await.unwrap()[0])
            .value(fee)
            .nonce(2)
            .gas(gas)
            .gas_price(base_fee)
            .send()
            .await.unwrap()
            .get_receipt()
            .await
    }

    async fn start_l1_to_l2_message_cancellation(
        &self,
        to_address: U256,
        selector: U256,
        payload: Vec<U256>,
        nonce: U256,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>> {
        let base_fee = self.as_ref().provider().as_ref().get_gas_price().await.unwrap();
        let builder = self.as_ref().startL1ToL2MessageCancellation(to_address, selector, payload, nonce);
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

    async fn cancel_l1_to_l2_message(
        &self,
        to_address: U256,
        selector: U256,
        payload: Vec<U256>,
        nonce: U256,
    ) -> Result<TransactionReceipt, RpcError<TransportErrorKind>> {
        let base_fee = self.as_ref().provider().as_ref().get_gas_price().await.unwrap();
        let builder = self.as_ref().cancelL1ToL2Message(to_address, selector, payload, nonce);
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
