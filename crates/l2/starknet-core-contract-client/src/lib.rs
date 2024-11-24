use std::sync::Arc;

use crate::clients::{StarknetCoreContractClient, StarknetDevCoreContractClient};
use starknet_proxy_client::deploy::{deploy_contract_behind_proxy, Error, ProxyVersion};
use utils::{LocalWalletSignerMiddleware, NO_CONSTRUCTOR_ARG};

pub mod clients;
pub mod interfaces;

const STARKNET_CORE_CONTRACT: &str = include_str!("../../../../artifacts/cairo-lang/Starknet.json");
const STARKNET_DEV_CORE_CONTRACT: &str =
    include_str!("../../../../artifacts/StarknetOverride.json");

pub enum CoreContractType {
    // custom contract written for testing (contains override function)
    Dev,
    // picked from cairo-lang repo
    Production,
}

pub enum CoreContractClientType {
    Dev(StarknetDevCoreContractClient),
    Production(StarknetCoreContractClient),
}

pub async fn deploy_starknet_core_contract_behind_proxy(
    client: Arc<LocalWalletSignerMiddleware>,
    proxy_type: ProxyVersion,
    core_contract_type: CoreContractType,
) -> Result<CoreContractClientType, Error> {
    match core_contract_type {
        CoreContractType::Dev => {
            let core_contract = deploy_contract_behind_proxy(
                client.clone(),
                STARKNET_DEV_CORE_CONTRACT,
                NO_CONSTRUCTOR_ARG,
                proxy_type,
            )
            .await?;
            Ok(CoreContractClientType::Dev(
                StarknetDevCoreContractClient::new(
                    core_contract.0.address(),
                    client.clone(),
                    core_contract.1.address(),
                ),
            ))
        }
        CoreContractType::Production => {
            let core_contract = deploy_contract_behind_proxy(
                client.clone(),
                STARKNET_CORE_CONTRACT,
                NO_CONSTRUCTOR_ARG,
                proxy_type,
            )
            .await?;
            Ok(CoreContractClientType::Production(
                StarknetCoreContractClient::new(
                    core_contract.0.address(),
                    client.clone(),
                    core_contract.1.address(),
                ),
            ))
        }
    }
}
