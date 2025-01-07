use anyhow::Ok;
use dotenv::dotenv;
use rig::providers::openai;
use rig::{completion::Prompt, tool::Tool};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
mod query;
mod types;
use graphql_client::GraphQLQuery;
use query::{get_pools, GetPools};
use types::{PortfolioError, StringContractAddress};

#[derive(Deserialize, Serialize, JsonSchema)]
struct ProtocolYield {
    protocol_address: String,
    apy: f64,
    tvl: f64,
    risk_score: f64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct Portfolio {
    wallet_address: StringContractAddress,
    assets: Vec<Asset>,
    total_value: f64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct Asset {
    token: StringContractAddress,
    amount: f64,
    current_protocol: Option<StringContractAddress>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct YieldAnalyzer {
    portfolio_data: Vec<Asset>,
}

impl YieldAnalyzer {

    ///havnt found a protocol to fetch but we have plans
    async fn fetch_jedi_swap_yields(&self) -> Result<Vec<ProtocolYield>, PortfolioError> {
        let client = reqwest::Client::new();

        let variables = get_pools::Variables {
            first: 100,
            token_addresses: None,
        };

        let query_body = serde_json::to_string(&GetPools::build_query(variables))
            .map_err(|e| PortfolioError(format!("Failed to serialize query: {}", e)))?;

        let response = client
            .post("https://api.v2.sepolia.jediswap.xyz/graphql")
            .header("Content-Type", "application/json")
            .body(query_body)
            .send()
            .await
            .map_err(|e| PortfolioError(format!("Request failed: {}", e)))?;

        let text = response
            .text()
            .await
            .map_err(|e| PortfolioError(format!("Failed to get response text: {}", e)))?;

        let data: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| PortfolioError(format!("Failed to parse JSON: {}", e)))?;

        // Check for GraphQL errors
        if let Some(errors) = data.get("errors") {
            return Err(PortfolioError(format!("GraphQL errors: {}", errors)));
        }

        // Parse the response into our ProtocolYield struct
        match data.get("data").and_then(|d| d.get("poolsData")) {
            Some(pools_data) => {
                println!("{}", pools_data.as_str().expect("no_pools data"));
                Ok(vec![ProtocolYield {
                    protocol_address: "test".to_string(),
                    apy: 2.0,
                    tvl: 2.0,
                    risk_score: 2.0,
                }])
                .map_err(|e| PortfolioError(format!("fetching failed: {}", e)))
            }
            None => Err(PortfolioError("Missing pools data in response".to_string())),
        }
    }

    fn calculate_apy(&self, volume: f64, tvl: f64) -> f64 {
        // Basic APY calculation: (daily_volume * fee_rate * 365) / tvl * 100
        let fee_rate = 0.003; // 0.3% fee
        (volume * fee_rate * 365.0) / tvl * 100.0
    }
}

impl Tool for YieldAnalyzer {
    const NAME: &'static str = "analyze_yield";
    type Error = PortfolioError;
    type Args = Portfolio;
    type Output = Vec<ProtocolYield>;

    async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {
        rig::completion::ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Analyzes portfolio and suggests optimal yield strategies on Starknet"
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "wallet_address": {"type": "string"},
                    "assets": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "token": {"type": "string"},
                                "amount": {"type": "number"},
                                "current_protocol": {"type": "string"}
                            }
                        }
                    }
                }
            }),
        }
    }

    async fn call(&self, portfolio: Self::Args) -> Result<Self::Output, Self::Error> {
        self.fetch_jedi_swap_yields().await
    }
}

#[tokio::main]
async fn main() {
    dotenv().expect("failed to load .env");
    let openai_client = openai::Client::from_env();

    let yield_agent = openai_client
        .agent(openai::GPT_4O)
        .preamble("You are YieldAI, an expert DeFi yield optimization assistant on Starknet.")
        .tool(YieldAnalyzer {
            portfolio_data: vec![],
        })
        .build();

    let result = yield_agent
        .prompt("Are you doing good?")
        .await
        .expect("Failed prompting gpt");
    println!("gpt: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::result::Result::Ok;
    #[tokio::test]
    async fn test_fetch_jedi_swap_yields() {
        let analyzer = YieldAnalyzer {
            portfolio_data: vec![],
        };

        let result = analyzer.fetch_jedi_swap_yields().await;
        assert!(result.is_ok(), "Should successfully fetch yields");

        if let Ok(yields) = result {
            assert!(!yields.is_empty(), "Should return at least one yield");
            assert!(yields[0].apy >= 0.0, "APY should be non-negative");
            assert!(yields[0].tvl >= 0.0, "TVL should be non-negative");
        }
    }

    #[tokio::test]
    async fn test_calculate_apy() {
        let analyzer = YieldAnalyzer {
            portfolio_data: vec![],
        };

        let volume = 1000.0;
        let tvl = 10000.0;
        let apy = analyzer.calculate_apy(volume, tvl);

        assert!(apy > 0.0, "APY should be positive");
        assert_eq!(apy, (volume * 0.003 * 365.0) / tvl * 100.0);
    }

    #[tokio::test]
    async fn test_yield_analyzer_tool() {
        let analyzer = YieldAnalyzer {
            portfolio_data: vec![],
        };

        let portfolio = Portfolio {
            wallet_address: StringContractAddress("0x123".to_string()),
            assets: vec![],
            total_value: 0.0,
        };

        let result = analyzer.call(portfolio).await;
        assert!(result.is_ok(), "Tool call should succeed");
    }
}
