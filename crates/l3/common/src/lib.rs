pub mod errors;
use starknet_accounts::SingleOwnerAccount;
use starknet_signers::{LocalWallet, SigningKey};
use starknet_providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet_providers::Provider;
use starknet_ff::FieldElement;

pub type LocalWalletSignerMiddleware = SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>;
pub const NO_CONSTRUCTOR_ARG:() = ();

// pub trait StarknetContractClient {
//     fn address(&self) -> Address;
//     fn client(&self) -> LocalWalletSignerMiddleware;
// }

