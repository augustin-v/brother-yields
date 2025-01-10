use crate::json;
use crate::types::{Portfolio, PortfolioError, ProtocolYield, YieldAnalyzer};
use rig::tool::Tool;

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
        Ok(vec![ProtocolYield::default()])
    }
}
