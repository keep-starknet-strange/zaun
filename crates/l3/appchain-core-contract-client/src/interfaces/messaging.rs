use common::invoke_contract;
use common::LocalWalletSignerMiddleware;
use starknet_accounts::Execution;
use starknet_core::types::FieldElement;
use starknet_core::types::StarknetError;
use std::sync::Arc;

pub struct Messaging {
    client: Arc<LocalWalletSignerMiddleware>,
    address: FieldElement,
}

impl Messaging {
    pub fn new(address: FieldElement, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self {
            client,
            address,
        }
    }

   // send_message_to_appchain - to_address, selector: felt252, payload: Span<felt252> -> Returns(felt252, felt252)
    // pub async fn send_message_to_appchain(
    //     &self,
    //     to_address: FieldElement,
    //     selector: FieldElement,
    //     payload: Vec<FieldElement>,
    // ) -> Result<(FieldElement, FieldElement), StarknetError> {
    //     let execution = invoke_contract(
    //         &self.client,
    //         self.address,
    //         "send_message_to_appchain",
    //         vec![to_address.into(), selector.into(), payload.into()],
    //     )
    //     .await;
    //     // let nonce = execution.estimate_fee(); // how to get the nonce from the execution



    // }  
   // consume_message_from_appchain - from_address, payload: Span<felt252> -> Returns(felt252)
   // start_message_cancellation - to_address, selector: felt252, payload: Span<felt252>, nonce: felt252 -> Returns(felt252)
    // cancel_message - to_address, selector: felt252, payload: Span<felt252>, nonce: felt252 -> Returns(felt252)
}