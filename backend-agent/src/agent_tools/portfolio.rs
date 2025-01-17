use rig::{
    completion::{CompletionModel, ToolDefinition},
    tool::Tool,
};
use serde_json::json;
use starknet::{
    core::types::{BlockId, BlockTag, Felt, FunctionCall},
    macros::selector,
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider, Url,
    },
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::task::spawn_blocking;
use tracing::{error, info, warn};

use crate::{
    backend::AppState,
    types::{PortfolioError, Price, Token},
    utils::get_verified_tokens,
};

#[derive(Clone)]
pub struct PortfolioFetch<M: CompletionModel> {
    pub appstate: Arc<Mutex<AppState<M>>>,
}

#[derive(serde::Deserialize)]
pub struct PortfolioArgs {
    wallet_address: Felt,
}

impl<M: CompletionModel + 'static> Tool for PortfolioFetch<M> {
    const NAME: &'static str = "mainnet_fetch_portfolio_balance";
    type Args = PortfolioArgs;
    type Output = ();
    type Error = PortfolioError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Retrieve mainnet portfolio using the user input wallet address. REQUIRES user wallet address".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "wallet_address": {
                        "type": "string",
                        "description": "The Starknet wallet address to fetch the portfolio for"
                    }
                },
                "required": ["wallet_address"]
            })
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        info!(
            "Starting portfolio fetch for wallet: {}",
            args.wallet_address.to_hex_string()
        );

        let mut token_balances: HashMap<Token, f64> = HashMap::new();
        let (vec6, vec8, vec18) = get_verified_tokens();
        let wallet_address = args.wallet_address;

        // Create a single provider instance
        info!("Creating provider...");

        let provider = Arc::new(JsonRpcClient::new(HttpTransport::new(
            Url::parse("https://starknet-mainnet.public.blastapi.io/rpc/v0_7").unwrap(),
        )));

        // Spawn each balance fetch in its own task to make it Sync
        info!("Spawning balance fetch tasks...");

        let balances6 = tokio::task::spawn({
            let provider = provider.clone();
            async move {
                info!("Starting 6 decimals fetch");

                let res = fetch_balances_with_provider(provider, 6.0, wallet_address, vec6).await;
                info!("Completed 6 decimals fetch");
                res
            }
        });

        let balances8 = tokio::task::spawn({
            let provider = provider.clone();
            async move {
                info!("Starting 8 decimals fetch");

                let res = fetch_balances_with_provider(provider, 8.0, wallet_address, vec8).await;
                info!("Completed 8 decimals fetch");
                res
            }
        });

        let balances18 = tokio::task::spawn({
            let provider = provider.clone();
            async move {
                info!("Starting 18 decimals fetch");
                let res = fetch_balances_with_provider(provider, 18.0, wallet_address, vec18).await;
                info!("Completed 18 decimals fetch");
                res
            }
        });

        // Await all tasks and handle errors
        info!("Awaiting balance results...");

        token_balances.extend(
            balances6
                .await
                .map_err(|e| PortfolioError(e.to_string()))??,
        );
        token_balances.extend(
            balances8
                .await
                .map_err(|e| PortfolioError(e.to_string()))??,
        );
        token_balances.extend(
            balances18
                .await
                .map_err(|e| PortfolioError(e.to_string()))??,
        );
        info!(" balance result suxess");

        // Format content for chat history
        info!("Formatting content...");

        let content = format!(
            "User wallet {} portfolio balances:\n{}",
            wallet_address.to_hex_string(),
            token_balances
                .iter()
                .map(|(token, amount)| format!("{}: {:.6} tokens", token.name, amount))
                .collect::<Vec<_>>()
                .join("\n")
        );
        info!("Updating state...");

        // First get the chat history reference without holding the appstate lock
        let chat_history = {
            let state = self.appstate.lock().await;
            info!("Got appstate lock");
            if let Some(agent) = state.agent_state.as_ref() {
                info!("Found agent state");
                // Get the chat_history Arc<Mutex> from inside Navigator
                agent.navigator.lock().await.chat_history.clone()
            } else {
                error!("No agent state found");
                return Err(PortfolioError("No agent state found".to_string()));
            }
        };
        info!("Released appstate lock");

        // Then update chat history directly
        info!("Attempting to update chat history...");
        {
            let mut history = chat_history.lock().await;
            history.push(rig::completion::Message {
                role: "system".to_string(),
                content: content.to_string(),
            });
        }
        info!("Chat history updated");

        // Finally update portfolio
        {
            let mut state = self.appstate.lock().await;
            state.update_portfolio(wallet_address.to_hex_string(), token_balances);
        }
        info!("Portfolio fetch completed successfully");

        Ok(())
    }
}

async fn fetch_balances_with_provider(
    provider: Arc<JsonRpcClient<HttpTransport>>,
    decimals: f64,
    wallet_address: Felt,
    tokens: Vec<(Felt, String)>,
) -> Result<HashMap<Token, f64>, PortfolioError> {
    let mut balances = HashMap::new();
    info!(
        "Starting fetch_balances_with_provider for {} decimals",
        decimals
    );
    for (token_address, token_name) in tokens {
        info!("Fetching balance for token: {}", token_name);

        let call_result = provider
            .call(
                FunctionCall {
                    contract_address: token_address,
                    entry_point_selector: selector!("balanceOf"),
                    calldata: vec![wallet_address],
                },
                BlockId::Tag(BlockTag::Latest),
            )
            .await
            .map_err(|e| PortfolioError(e.to_string()))?;

        if !call_result.is_empty() && call_result[0] > Felt::ZERO {
            let balance: u128 = call_result[0]
                .try_into()
                .expect("Failed converting Felt to u128");
            let adjusted_balance: f64 = balance as f64 / 10_f64.powf(decimals);
            let token_info = Token {
                name: token_name,
                address: crate::types::StringContractAddress(token_address.to_hex_string()),
                price: Price::default(),
            };
            balances.insert(token_info, adjusted_balance);
        }
    }
    info!(
        "Completed fetch_balances_with_provider for {} decimals",
        decimals
    );

    Ok(balances)
}
