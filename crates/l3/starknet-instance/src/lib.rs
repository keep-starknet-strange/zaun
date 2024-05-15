use starknet_accounts::SingleOwnerAccount;
use starknet_ff::FieldElement;
use starknet_providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet_signers::{LocalWallet, SigningKey};
use url::Url;
use common::errors::Error;

pub type LocalWalletSignerMiddleware =
    SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>;

#[derive(Debug)]
pub struct StarknetClient {
    client: LocalWalletSignerMiddleware,
    url: Url,
}

impl StarknetClient {
    pub fn attach(
        rpc_endpoint: Option<String>,
        chain_id: Option<String>,
        priv_key: Option<String>,
        account_addr: Option<String>,
    ) -> Result<Self, Error> {
        let chain_id = chain_id
            .ok_or(Error::CustomError("chain_id missing".to_string()))?
            .parse::<FieldElement>()
            .map_err(|_| Error::CustomError("Invalid chain_id format".to_string()))?;
        let url = Url::parse(&rpc_endpoint.ok_or(Error::CustomError("rpc_endpoint missing".to_string()))?)?;
        let provider = JsonRpcClient::new(HttpTransport::new(url.clone()));
        let signer_key = FieldElement::from_hex_be(&priv_key.ok_or(Error::Hex(hex::FromHexError::InvalidStringLength))?)
            .map_err(|_| Error::Hex(hex::FromHexError::InvalidStringLength))?;
        let signer = LocalWallet::from(SigningKey::from_secret_scalar(signer_key));
        let account_address = FieldElement::from_hex_be(&account_addr.ok_or(Error::Hex(hex::FromHexError::InvalidStringLength))?)
            .map_err(|_| Error::Hex(hex::FromHexError::InvalidStringLength))?;

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

