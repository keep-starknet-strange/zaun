use ethers::contract::ContractError;
use ethers::middleware::Middleware;
use ethers::providers::ProviderError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error<M: Middleware> {
    #[error(transparent)]
    ContractError(#[from] ContractError<M>),
    #[error(transparent)]
    ProviderError(#[from] ProviderError),
}
