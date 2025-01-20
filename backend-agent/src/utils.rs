use anyhow::Error;
use starknet::{
    accounts::{Account, ExecutionEncoding, SingleOwnerAccount},
    core::{
        chain_id,
        types::{BlockId, BlockTag, Call, Felt, InvokeTransactionResult},
        utils::get_selector_from_name,
    },
    macros::felt,
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Url,
    },
    signers::{LocalWallet, SigningKey},
};

pub async fn _call_felt_2_usize_contract(
    value: Felt,
) -> anyhow::Result<InvokeTransactionResult, Error> {
    let sepolia_api_key = std::env::var("SEPOLIA_PRIVATE_KEY")
        .expect("SEPOLIA_PRIVATE_KEY must be set in environment");
    let sepolia_account_add = std::env::var("SEPOLIA_ACCOUNT_ADDRESS")
        .expect("SEPOLIA_ACCOUNT_ADDRESS must be set in environment");
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

    format!("You are 'DEFIPROMAN', you are here to help the user. Use your knowledge of various Starknet DeFi protocols in the knowledge files injected in you. When you see portfolio information in the chat history (marked as system messages('role': 'system')), 
actively incorporate this information into your responses when relevant. ALWAYS acknowledge and MAKE SURE TO reference 
the user's token holdings when discussing their portfolio or related topics. You do NOT need to fetch the same wallet address twice. Just say that you've already fetched it if requested. Keep your answers short concise and user-friendly. Always call the user 'Starknet brother' like a true starknet defi strategy expert answer with SPECIFIC strategies. You MUST keep your answers under 3 lines. IMPORTANT: Do not use outdated info (date now {}), do not talk about anything else than DeFi strategies on Starknet under ANY circumstance EXCEPT if user is just saying hello to him, be polite dont need to give advice in that case.",current_date)
}

pub fn get_verified_tokens() -> (
    Vec<(Felt, String)>,
    Vec<(Felt, String)>,
    Vec<(Felt, String)>,
) {
    let vec_six_decimals = vec![
        (
            felt!("0x004878d1148318a31829523ee9c6a5ee563af6cd87f90a30809e5b0d27db8a9b"),
            "SWAY".to_string(),
        ),
        (
            felt!("0x053c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8"),
            "USDC".to_string(),
        ),
        (
            felt!("0x068f5c6a61780768455de69077e07e89787839bf8166decfbf92b645209c0fb8"),
            "USDT".to_string(),
        ),
    ];

    let vec_eight_decimals = vec![(
        felt!("0x03fe2b97c1fd336e750087d68b9b867997fd64a2661ff3ca5a7c771641e8e7ac"),
        "WBTC".to_string(),
    )];

    let vec_eighteen_decimals = vec![
        (
            felt!("0x00e33356072418951fdf3312e3e2eef99abf6d7e12df6ff956082d3e178dde2a"),
            "EIGHT".to_string(),
        ),
        (
            felt!("0x03b405a98c9e795d427fe82cdeeeed803f221b52471e3a757574a2b4180793ee"),
            "BROTHER".to_string(),
        ),
        (
            felt!("0x0137dfca7d96cdd526d13a63176454f35c691f55837497448fad352643cfe4d4"),
            "AKU".to_string(),
        ),
        (
            felt!("0x05574eb6b8789a91466f902c380d978e472db68170ff82a5b650b95a58ddf4ad"),
            "DAI".to_string(),
        ),
        (
            felt!("0x00da114221cb83fa859dbdb4c44beeaa0bb37c7537ad5ae66fe5e0efd20e6eb3"),
            "DAIv0".to_string(),
        ),
        (
            felt!("0x075afe6402ad5a5c20dd25e10ec3b3986acaa647b77e4ae24b0cbc9a54a27a87"),
            "EKUBO".to_string(),
        ),
        (
            felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7"),
            "ETH".to_string(),
        ),
        (
            felt!("0x0124aeb495b947201f5fac96fd1138e326ad86195b98df6dec9009158a533b49"),
            "LORDS".to_string(),
        ),
        (
            felt!("0x070a76fd48ca0ef910631754d77dd822147fe98a569b826ec85e3c33fde586ac"),
            "LUSD".to_string(),
        ),
        (
            felt!("0x00c530f2c0aa4c16a0806365b0898499fba372e5df7a7172dc6fe9ba777e8007"),
            "NSTR".to_string(),
        ),
        (
            felt!("0x039877a272619050ab8b0e3e0a19b58d076fc2ce84da1dc73b699590e629f2b8"),
            "OWL".to_string(),
        ),
        (
            felt!("0x049201f03a0f0a9e70e28dcd74cbf44931174dbe3cc4b2ff488898339959e559"),
            "PAL".to_string(),
        ),
        (
            felt!("0x319111a5037cbec2b3e638cc34a3474e2d2608299f3e62866e9cc683208c610"),
            "rETH".to_string(),
        ),
        (
            felt!("0x01e0eee22c684fdf32babdd65e6bcca62a8ce2c23c8d5e68f3989595d26e1b4a"),
            "SPEPE".to_string(),
        ),
        (
            felt!("0x4718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d"),
            "STRK".to_string(),
        ),
        (
            felt!("0x049210ffc442172463f3177147c1aeaa36c51d152c1b0630f2364c300d4f48ee"),
            "UNI".to_string(),
        ),
        (
            felt!("0x0782f0ddca11d9950bc3220e35ac82cf868778edb67a5e58b39838544bc4cd0f"),
            "vSTRK".to_string(),
        ),
        (
            felt!("0x042b8f0484674ca266ac5d08e4ac6a3fe65bd3129795def2dca5c34ecc5f96d2"),
            "wstETH".to_string(),
        ),
    ];

    (vec_six_decimals, vec_eight_decimals, vec_eighteen_decimals)
}
