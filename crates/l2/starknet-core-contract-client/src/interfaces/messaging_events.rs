use ethers::prelude::abigen;

abigen!(
    StarknetMessagingEvents,
    r#"[
        event LogMessageToL1(uint256 indexed fromAddress, address indexed toAddress, uint256[] payload)
        event LogMessageToL2(address indexed fromAddress, uint256 indexed toAddress, uint256 indexed selector, uint256[] payload, uint256 nonce, uint256 fee)
        event ConsumedMessageToL1(uint256 indexed fromAddress, address indexed toAddress, uint256[] payload)
        event ConsumedMessageToL2(address indexed fromAddress, uint256 indexed toAddress, uint256 indexed selector, uint256[] payload, uint256 nonce)
        event MessageToL2CancellationStarted(address indexed fromAddress, uint256 indexed toAddress, uint256 indexed selector, uint256[] payload, uint256 nonce)
        event MessageToL2Canceled(address indexed fromAddress, uint256 indexed toAddress, uint256 indexed selector, uint256[] payload, uint256 nonce)
    ]"#
);
