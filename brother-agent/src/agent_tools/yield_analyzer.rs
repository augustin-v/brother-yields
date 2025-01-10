use crate::fetch_all_tokens;
use crate::types::{ProtocolYield, YieldAnalyzer};
use anyhow::{Error, Ok};

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
                risk_score: market.risk_score,
            };
            res.push(temp_proto_yield);
        }

        Ok(res).map_err(|e| Error::context(e, "err"))
    }
}
