use starknet_core::types::StarknetError;
use starknet_providers::ProviderError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ProviderError(#[from] ProviderError),
    #[error(transparent)]
    StarknetError(#[from] StarknetError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("['bytecode']['object'] is not a string")]
    BytecodeObject,
    #[error(transparent)]
    Hex(#[from] hex::FromHexError),
    #[error("Failed to parse URL")]
    UrlParser,
    // CustomError which accepts string
    #[error("Custom error: {0}")]
    CustomError(String),
}
