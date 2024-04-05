use alloy::{
    network::Ethereum,
    providers::Provider,
    sol_types::ContractError,
    transports::{http::Http, RpcError}
};
use thiserror::Error;

// #[derive(Debug, Error)]
// pub enum Error {
//     #[error(transparent)]
//     ContractError(#[from] ContractError<P>),
//     #[error(transparent)]
//     ProviderError(#[from] RpcError<P>),
// }
