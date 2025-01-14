use crate::tokens::fetch_all_tokens;
use crate::types::{ProtocolYield, YieldAnalyzer};
use anyhow::Error;
use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize, Clone)]
pub struct AnalyzerTool{
    pub yields_data: Vec<ProtocolYield>
}

impl YieldAnalyzer {
    /// supported for now: "STRK", "BROTHER", "ETH"; {token}/USDC pairs
    pub async fn get_yields_data() -> Result<Vec<ProtocolYield>, Error> {
        let (tokens, market_data) = fetch_all_tokens().await;

        let mut res: Vec<ProtocolYield> = Vec::with_capacity(tokens.len());
        for (token, market) in tokens.iter().zip(market_data.iter()) {
            println!(
                "Token {} has price ${} and 24h volume ${}",
                token.name, market.price, market.volume_24h
            );

            let temp_proto_yield = ProtocolYield {
                token: token.clone(),
                apy: market.apy,
                tvl: market.tvl,
                volume_24h: market.volume_24h,
                risk_score: market.risk_score,
                pool_type: market.pool_type.clone()
            };
            res.push(temp_proto_yield);
        }

        anyhow::Ok(res).map_err(|e| Error::context(e, "err"))
    }
}

impl Tool for AnalyzerTool {
    const NAME: &'static str = "estimate_yield_returns";

    type Args = AnalyzArgs;
    type Output = String;
    type Error = AnalyzeError;
    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Fetch the estimated yield returns for supported tokens. ETH, STRK, BROTHER all with USDC pair".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "token": {
                        "type": "string",
                        "description": "The token symbol to fetch yields for"
                    },
                    "risk_score": {
                        "type": "string",
                        "description": "Optional risk score filter"
                    }
                },
                "required": ["token"]
            })
        }
    }
    

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let filtered_data = if let Some(risk_score) = args.risk_score {
            self.yields_data.iter()
                .filter(|yield_data| yield_data.token.name == args.token && yield_data.risk_score.to_string() == risk_score)
                .cloned()
                .collect()
        } else {
            self.yields_data.iter()
                .filter(|yield_data| yield_data.token.name == args.token)
                .cloned()
                .collect()
        };
        
        Ok(format_yields_data(filtered_data))
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Portfolio error: {0}")]
pub struct AnalyzeError(pub String);

#[derive(serde::Deserialize)]
pub struct AnalyzArgs {
    token: String,
    risk_score: Option<String>
}

pub fn format_yields_data(yields_data: Vec<ProtocolYield>) -> String {
    let formatted_data = yields_data.iter().map(|yield_data| {
        format!("Token: {}, APY: {:.2}%, TVL: ${:.2}, Risk Score: {:.2}",
            yield_data.token.name, yield_data.apy * 100.0, yield_data.tvl, yield_data.risk_score)
    }).collect::<Vec<String>>().join(", ");
    
    format!("Here is the latest yields data of {{token}}/USDC pair: {}.", formatted_data)
}