use starknet::accounts::SingleOwnerAccount;
use std::sync::Arc;
use crate::interfaces::AppChainCoreContractTrait;

// pub struct AppChainCoreContractClient {
//     core_contract: AppChainCoreContractTrait<Arc<SingleOwnerAccount<Provider<HttpTransport>>>>,
// }

// impl AppChainCoreContractClient {
//     pub fn new(provider: Arc<Provider<HttpTransport>>, account: Arc<SingleOwnerAccount<dyn Provider<HttpTransport>>>) -> Self {
//         Self {
//             core_contract: AppChainCoreContractTrait::new(provider, account),
//         }
//     }
// }