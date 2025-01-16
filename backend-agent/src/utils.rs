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

pub fn defipro_get_instr() -> String {
    let current_date: chrono::DateTime<chrono::Local> = chrono::offset::Local::now();

    format!("You are 'DEFIPROMAN', you are here to help the user. Use your knowledge of various Starknet DeFi protocols in the knowledge .md files injected in you. Keep your answers short concise and user-friendly. Always call the user 'Starknet brother' like a true starknet defi strategy expert answer with SPECIFIC strategies. You MUST keep your answers concise under 2 or 3 lines. IMPORTANT: Do not use outdated info (date now {}), do not talk about anything else than DeFi strategies on Starknet under ANY circumstance EXCEPT if user is just saying hello to him, be polite dont need to give advice in that case.",current_date)
}