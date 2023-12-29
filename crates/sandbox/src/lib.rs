use ethers::abi::Tokenize;
use ethers::prelude::{ContractFactory, ContractInstance};
use ethers::types::{Address, Bytes};
use ethers::utils::hex::FromHex;
use ethers::utils::{Anvil, AnvilInstance};
use starknet_core_contract_client::clients::StarknetSovereignContractClient;
use starknet_core_contract_client::{LocalWalletSignerMiddleware, StarknetCoreContractClient};
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("['bytecode']['object'] is not a string")]
    BytecodeObject,
    #[error(transparent)]
    Hex(#[from] hex::FromHexError),
}

pub struct EthereumSandbox {
    _anvil: AnvilInstance,
    client: Arc<StarknetSovereignContractClient>,
}

impl EthereumSandbox {
    pub fn new(
        core_contract_address: Address,
        ether_client: Arc<LocalWalletSignerMiddleware>,
        anvil_path: Option<PathBuf>,
    ) -> Self {
        let anvil_path: PathBuf = anvil_path
            .or_else(|| std::env::var("ANVIL_PATH").map(Into::into).ok())
            .unwrap_or_else(|| dirs::home_dir().unwrap().join(".foundry/bin/anvil"));

        // Will panic if invalid path
        let anvil = Anvil::at(anvil_path).spawn();

        let client = StarknetSovereignContractClient::new(core_contract_address, ether_client);
        Self {
            _anvil: anvil,
            client: Arc::new(client),
        }
    }

    pub fn client(&self) -> Arc<StarknetSovereignContractClient> {
        self.client.clone()
    }

    pub fn address(&self) -> Address {
        self.client.address()
    }

    pub async fn deploy<T: Tokenize>(
        &self,
        contract_build_artifacts: &str,
        contructor_args: T,
    ) -> Result<
        ContractInstance<Arc<LocalWalletSignerMiddleware>, LocalWalletSignerMiddleware>,
        Error,
    > {
        let (abi, bytecode) = {
            let mut artifacts: serde_json::Value = serde_json::from_str(contract_build_artifacts)?;
            let abi = serde_json::from_value(artifacts["abi"].take())?;
            let bytecode = Bytes::from_hex(
                artifacts["bytecode"]["object"]
                    .as_str()
                    .ok_or(Error::BytecodeObject)?,
            )?;
            (abi, bytecode)
        };

        let factory = ContractFactory::new(abi, bytecode, self.client.client().clone());

        Ok(factory
            .deploy(contructor_args)
            .expect("Failed to deploy contract")
            .send()
            .await
            .expect("Ethereum polling error"))
    }
}
