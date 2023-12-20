use ethers::prelude::abigen;

// Give access to the external method defined in StarknetMessaging.sol
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
