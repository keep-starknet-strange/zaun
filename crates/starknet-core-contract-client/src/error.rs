use alloy::{
    network::Ethereum,
    providers::Provider,
    sol_types::ContractError,
    transports::RpcError
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error<P: Provider<Ethereum>> {
    #[error(transparent)]
    ContractError(#[from] ContractError<P>),
    #[error(transparent)]
    ProviderError(#[from] RpcError<P>),
}
