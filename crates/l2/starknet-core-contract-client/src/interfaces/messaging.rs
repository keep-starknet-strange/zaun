use async_trait::async_trait;
use ethers::{
    contract::ContractError,
    prelude::abigen,
    providers::Middleware,
    types::{TransactionReceipt, U256},
};

use utils::errors::Error;

type MessageHash = [u8; 32];

// StarknetMessaging.sol
abigen!(
    StarknetMessaging,
    r#"[
        function l1ToL2Messages(bytes32 msgHash) external view returns (uint256)
        function l2ToL1Messages(bytes32 msgHash) external view returns (uint256)
        function l1ToL2MessageCancellations(bytes32 msgHash) external view returns (uint256)

        function sendMessageToL2(uint256 toAddress, uint256 selector, uint256[] calldata payload) external payable override returns (bytes32, uint256)
        function consumeMessageFromL2(uint256 fromAddress, uint256[] calldata payload) external override returns (bytes32)

        function startL1ToL2MessageCancellation(uint256 toAddress, uint256 selector, uint256[] calldata payload, uint256 nonce) external override returns (bytes32) 
        function cancelL1ToL2Message(uint256 toAddress, uint256 selector, uint256[] calldata payload, uint256 nonce) external override returns (bytes32) 
    ]"#,
);

#[async_trait]
pub trait StarknetMessagingTrait<M: Middleware> {
    async fn l1_to_l2_messages(&self, msg_hash: MessageHash) -> Result<U256, Error<M>>;
    async fn l2_to_l1_messages(&self, msg_hash: MessageHash) -> Result<U256, Error<M>>;
    async fn l1_to_l2_message_cancellations(&self, msg_hash: MessageHash)
        -> Result<U256, Error<M>>;
    async fn send_message_to_l2(
        &self,
        to_address: U256,
        selector: U256,
        payload: Vec<U256>,
        fee: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn start_l1_to_l2_message_cancellation(
        &self,
        to_address: U256,
        selector: U256,
        payload: Vec<U256>,
        nonce: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn cancel_l1_to_l2_message(
        &self,
        to_address: U256,
        selector: U256,
        payload: Vec<U256>,
        nonce: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
}

#[async_trait]
impl<T, M: Middleware> StarknetMessagingTrait<M> for T
where
    T: AsRef<StarknetMessaging<M>> + Send + Sync,
{
    async fn l1_to_l2_messages(&self, msg_hash: MessageHash) -> Result<U256, Error<M>> {
        self.as_ref()
            .l_1_to_l2_messages(msg_hash)
            .call()
            .await
            .map_err(Into::into)
    }

    async fn l2_to_l1_messages(&self, msg_hash: MessageHash) -> Result<U256, Error<M>> {
        self.as_ref()
            .l_2_to_l1_messages(msg_hash)
            .call()
            .await
            .map_err(Into::into)
    }

    async fn l1_to_l2_message_cancellations(
        &self,
        msg_hash: MessageHash,
    ) -> Result<U256, Error<M>> {
        self.as_ref()
            .l_1_to_l2_message_cancellations(msg_hash)
            .call()
            .await
            .map_err(Into::into)
    }

    async fn send_message_to_l2(
        &self,
        to_address: U256,
        selector: U256,
        payload: Vec<U256>,
        fee: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .send_message_to_l2(to_address, selector, payload)
            .value(fee) // L1 message fee must be between 0 and 1 ether
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn start_l1_to_l2_message_cancellation(
        &self,
        to_address: U256,
        selector: U256,
        payload: Vec<U256>,
        nonce: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .start_l1_to_l2_message_cancellation(to_address, selector, payload, nonce)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }

    async fn cancel_l1_to_l2_message(
        &self,
        to_address: U256,
        selector: U256,
        payload: Vec<U256>,
        nonce: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .cancel_l1_to_l2_message(to_address, selector, payload, nonce)
            .send()
            .await
            .map_err(Into::<ContractError<M>>::into)?
            .await
            .map_err(Into::into)
    }
}
