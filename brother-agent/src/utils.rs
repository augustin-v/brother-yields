use anyhow::Error;
use starknet::{
    accounts::{Account, ExecutionEncoding, SingleOwnerAccount},
    core::{
        chain_id,
        types::{BlockId, BlockTag, Call, Felt, InvokeTransactionResult},
        utils::get_selector_from_name,
    },
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Url,
    },
    signers::{LocalWallet, SigningKey},
};

pub async fn _call_felt_2_usize_contract(
    value: Felt,
) -> anyhow::Result<InvokeTransactionResult, Error> {
    let sepolia_api_key =
        std::env::var("SEPOLIA_PRIVATE_KEY").expect("SEPOLIA_PRIVATE_KEY not set in .env");
    let sepolia_account_add =
        std::env::var("SEPOLIA_ACCOUNT_ADDRESS").expect("SEPOLIA_ACCOUNT_ADDRESS not set in .env");
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex(&sepolia_api_key).unwrap(),
    ));
    let address = Felt::from_hex(&sepolia_account_add).unwrap();
    let contract_address =
        Felt::from_hex("0x0638ff764ddd96be61cc35eb6cc7da3702790c4056c3fa976e0931441d33ef1e")
            .unwrap();

    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::SEPOLIA,
        ExecutionEncoding::New,
    );

    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let result: Result<
        InvokeTransactionResult,
        starknet::accounts::AccountError<
            starknet::accounts::single_owner::SignError<starknet::signers::local_wallet::SignError>,
        >,
    > = account
        .execute_v1(vec![Call {
            to: contract_address,
            selector: get_selector_from_name("felt252_to_usize").unwrap(),
            calldata: vec![value],
        }])
        .send()
        .await;

    Ok(result.expect("Couldn't get transaction hash"))
}
