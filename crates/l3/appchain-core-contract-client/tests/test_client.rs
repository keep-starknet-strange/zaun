use appchain_core_contract_client::clients;
use appchain_core_contract_client::interfaces::operator::Operator;
use clients::client::StarknetCoreContractClient;
use color_eyre::Result;
use starknet_ff::FieldElement;
use starknet_instance::StarknetClient;
use std::sync::Arc;

#[tokio::test]
async fn test_operator() -> Result<()> {
    let rpc_endpoint = "http://localhost:5050".to_string();
    let chain_id = "0x4b4154414e41".to_string();
    let priv_key = "0x1800000000300000180000000000030000000000003006001800006600".to_string();
    let account_addr =
        "0x6162896d1d7ab204c7ccac6dd5f8e9e7c25ecd5ae4fcb4ad32e57786bb46e03".to_string();
    let starknet_client = StarknetClient::attach(rpc_endpoint, chain_id, priv_key, account_addr)?;

    let address = FieldElement::from_hex_be(
        "0x07100a5f88487652c9efaa81d9037927f67000512ff25e00dded29e5af58ae58",
    )?;
    let client = Arc::new(starknet_client.client());
    let core_contract_client = StarknetCoreContractClient::new(address, client);

    let get_operator =
        <StarknetCoreContractClient as AsRef<Operator>>::as_ref(&core_contract_client)
            .get_program_info()
            .await?;
    assert_eq!(get_operator.0, FieldElement::from(0u64));
    assert_eq!(get_operator.1, FieldElement::from(0u64));

    let operator = FieldElement::from_hex_be(
        "0x6b86e40118f29ebe393a75469b4d926c7a44c2e2681b6d319520b7c1156d114",
    )?;
    let register_operator =
        <StarknetCoreContractClient as AsRef<Operator>>::as_ref(&core_contract_client)
            .register_operator(operator)
            .await?;

    let is_operator =
        <StarknetCoreContractClient as AsRef<Operator>>::as_ref(&core_contract_client)
            .is_operator(operator)
            .await?;
    assert_eq!(is_operator, true);

    Ok(())
}
