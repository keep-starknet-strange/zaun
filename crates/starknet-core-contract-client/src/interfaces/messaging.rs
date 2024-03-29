use async_trait::async_trait;

use crate::Error;

use alloy::{
    network::Ethereum,
    primitives::U256,
    providers::Provider,
    rpc::types::eth::TransactionReceipt,
    sol,
    sol_types::ContractError,
};

type MessageHash = [u8; 32];

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
pub trait StarknetMessagingTrait<P: Provider<Ethereum>> {
    async fn l1_to_l2_messages(&self, msg_hash: MessageHash) -> Result<U256, Error<P>>;
    async fn l2_to_l1_messages(&self, msg_hash: MessageHash) -> Result<U256, Error<P>>;
    async fn l1_to_l2_message_cancellations(&self, msg_hash: MessageHash)
        -> Result<U256, Error<P>>;
    async fn send_message_to_l2(
        &self,
        to_address: U256,
        selector: U256,
        payload: Vec<U256>,
        fee: U256,
    ) -> Result<Option<TransactionReceipt>, Error<P>>;
    async fn start_l1_to_l2_message_cancellation(
        &self,
        to_address: U256,
        selector: U256,
        payload: Vec<U256>,
        nonce: U256,
    ) -> Result<Option<TransactionReceipt>, Error<P>>;
    async fn cancel_l1_to_l2_message(
        &self,
        to_address: U256,
        selector: U256,
        payload: Vec<U256>,
        nonce: U256,
    ) -> Result<Option<TransactionReceipt>, Error<P>>;
}

#[async_trait]
impl<T, P: Provider<Ethereum>> StarknetMessagingTrait<P> for T
where
    T: AsRef<StarknetMessaging::StarknetMessagingInstance<Ethereum, T, P>> + Send + Sync,
{
    async fn l1_to_l2_messages(&self, msg_hash: MessageHash) -> Result<U256, Error<P>> {
        self
            .l1_to_l2_messages(msg_hash)
            .await
            .map_err(Into::into)
    }

    async fn l2_to_l1_messages(&self, msg_hash: MessageHash) -> Result<U256, Error<P>> {
        self
            .l2_to_l1_messages(msg_hash)
            .await
            .map_err(Into::into)
    }

    async fn l1_to_l2_message_cancellations(
        &self,
        msg_hash: MessageHash,
    ) -> Result<U256, Error<P>> {
        self
            .l1_to_l2_message_cancellations(msg_hash)
            .await
            .map_err(Into::into)
    }

    async fn send_message_to_l2(
        &self,
        to_address: U256,
        selector: U256,
        payload: Vec<U256>,
        fee: U256,
    ) -> Result<Option<TransactionReceipt>, Error<P>> {
        self
            .send_message_to_l2(to_address, selector, payload, fee)
            .await
            .map_err(Into::into)
    }

    async fn start_l1_to_l2_message_cancellation(
        &self,
        to_address: U256,
        selector: U256,
        payload: Vec<U256>,
        nonce: U256,
    ) -> Result<Option<TransactionReceipt>, Error<P>> {
        self
            .start_l1_to_l2_message_cancellation(to_address, selector, payload, nonce)
            .await
            .map_err(Into::into)
    }

    async fn cancel_l1_to_l2_message(
        &self,
        to_address: U256,
        selector: U256,
        payload: Vec<U256>,
        nonce: U256,
    ) -> Result<Option<TransactionReceipt>, Error<P>> {
        self
            .cancel_l1_to_l2_message(to_address, selector, payload, nonce)
            .await
            .map_err(Into::into)
    }
}
