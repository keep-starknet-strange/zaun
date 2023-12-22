pub mod clients;
mod error;
pub mod interfaces;

pub use error::Error;

use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;

pub type LocalMiddleware = SignerMiddleware<Provider<Http>, LocalWallet>;
