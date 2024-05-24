use common::errors::Error;
use starknet_accounts::SingleOwnerAccount;
use starknet_ff::FieldElement;
use starknet_providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet_signers::{LocalWallet, SigningKey};
use url::Url;

pub type LocalWalletSignerMiddleware =
    SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>;

#[derive(Debug)]
pub struct StarknetClient {
    client: LocalWalletSignerMiddleware,
    url: Url,
}

impl StarknetClient {
    pub fn client(&self) -> &LocalWalletSignerMiddleware {
        &self.client
    }
    pub fn attach(
        rpc_endpoint: String,
        chain_id: String,
        priv_key: String,
        account_addr: String,
    ) -> Result<Self, Error> {
        let chain_id = parse_field_element(&chain_id, "Invalid chain_id format")?;
        let url = Url::parse(&rpc_endpoint)?;
        let provider = JsonRpcClient::new(HttpTransport::new(url.clone()));

        let signer_key = parse_field_element(&priv_key, "Invalid private key format")?;
        let signer = LocalWallet::from(SigningKey::from_secret_scalar(signer_key));

        let account_address = parse_field_element(&account_addr, "Invalid account address format")?;

        let account = SingleOwnerAccount::new(
            provider,
            signer,
            account_address,
            chain_id,
            starknet_accounts::ExecutionEncoding::New,
        );

        Ok(Self {
            client: account,
            url: url,
        })
    }
}

fn parse_field_element(hex_str: &str, error_msg: &str) -> Result<FieldElement, Error> {
    FieldElement::from_hex_be(hex_str).map_err(|_| Error::CustomError(error_msg.to_string()))
}
