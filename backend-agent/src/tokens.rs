use crate::StringContractAddress;
use crate::{market::CoinMarketData, types::Token};
use starknet::{
    core::types::{BlockId, BlockTag, Felt, FunctionCall},
    macros::{felt, selector},
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider, Url,
    },
};
use std::collections::HashMap;

// some verified tokens addresses on Starknet
const BROTHER: &str = "0x3b405a98c9e795d427fe82cdeeeed803f221b52471e3a757574a2b4180793ee";
const STRK: &str = "0x4718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d";
const ETH: &str = "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
// const USDC: &str = "0x53c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8";

// chain id on coingecko
const CHAIN_ID: &str = "starknet";

// vec1[x] related to vec2[x] and so on (vec2[x] countains Market Data about vec1[x])
pub async fn fetch_all_tokens() -> (Vec<Token>, Vec<CoinMarketData>) {
    let api_key = std::env::var("COINGECKO_API_KEY").expect("COINGECKO_API_KEY not set");
    let mut tokens = Vec::new();
    let mut market_data = Vec::new();
    let client = reqwest::Client::new();

    let addresses = vec![BROTHER, STRK, ETH];
    let addresses_str = addresses.join(",");

    let url = format!(
        "https://api.coingecko.com/api/v3/simple/token_price/{CHAIN_ID}?contract_addresses={addresses_str}&vs_currencies=usd&include_market_cap=true&include_24hr_vol=true&include_24hr_change=true"
    );

    if let Ok(resp) = client
        .get(&url)
        .header("accept", "application/json")
        .header("x-cg-demo-api-key", api_key)
        .send()
        .await
    {
        if let Ok(prices) = resp.json::<HashMap<String, HashMap<String, f64>>>().await {
            // Process each token
            for (address, data) in prices {
                let price = data.get("usd").copied().unwrap_or_default();
                let volume_24h = data.get("usd_24h_vol").copied().unwrap_or_default();
                let price_change_24h = data.get("usd_24h_change").copied().unwrap_or_default();

                let token_name = match address.as_str() {
                    addr if addr == BROTHER => "BROTHER",
                    addr if addr == STRK => "STRK",
                    addr if addr == ETH => "ETH",
                    _ => continue,
                };

                // Create and store Token
                tokens.push(Token {
                    name: token_name.to_string(),
                    address: StringContractAddress::from(address.as_str()),
                    price,
                });

                // Create and store CoinMarketData
                let market_data_entry = CoinMarketData::from_gecko_data(
                    token_name,
                    price,
                    volume_24h,
                    price_change_24h,
                )
                .await;

                market_data.push(market_data_entry);
            }
        }
    }
    (tokens, market_data)
}

pub async fn fetch_token_reserve(contract_address: Felt) -> Felt {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-mainnet.public.blastapi.io/rpc/v0_7").unwrap(),
    ));

    let call_result = provider
        .call(
            FunctionCall {
                contract_address,
                entry_point_selector: selector!("total_supply"),
                calldata: Vec::with_capacity(0),
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .expect("failed to call contract");

    let res = call_result;
    // first item is the total supply
    res[0]
}
pub async fn fetch_usdc_reserve() -> Felt {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-mainnet.public.blastapi.io/rpc/v0_7").unwrap(),
    ));

    let contract_address =
        felt!("0x53c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8");

    let call_result = provider
        .call(
            FunctionCall {
                contract_address,
                entry_point_selector: selector!("total_supply"),
                calldata: Vec::with_capacity(0),
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .expect("failed to call contract");

    let res = call_result;
    // first item is the total supply
    res[0]
}
