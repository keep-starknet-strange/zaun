use starknet_core::types::StarknetError;
use starknet_providers::ProviderError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ProviderError(#[from] ProviderError),
    #[error(transparent)]
    StarknetError(#[from] StarknetError),
}
