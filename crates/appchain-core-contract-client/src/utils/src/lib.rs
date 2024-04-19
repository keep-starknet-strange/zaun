use starknet::{
    accounts::SingleOwnerAccount,
    providers::
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider, Signer
};
pub type LocalWalletSignerMiddleware = SingleOwnerAccount<Provider, Signer>;

pub trait StarknetContractClient {
    fn address(&self) -> Address;
    fn client(&self) -> LocalWalletSignerMiddleware;
}