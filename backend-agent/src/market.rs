use crate::math::{calculate_risk_score, STARKNET_TVL_ESTIMATE};
use crate::tokens::{fetch_token_reserve, fetch_usdc_reserve};
use crate::types::ComputeError;

use starknet::macros::felt;

#[derive(Debug)]
pub struct CoinMarketData {
    pub price: f64,
    pub reserve_a: f64,
    pub reserve_b: f64,
    pub volume_24h: f64,
    pub price_change_24h: f64,

    // Pool metrics
    pub liquidity: f64,
    pub tvl: f64,

    // Computed metrics
    pub apy: f64,
    pub risk_score: f64,
}

impl CoinMarketData {
    pub async fn from_gecko_data(
        token_name: &str,
        price: f64,
        volume_24h: f64,
        price_change_24h: f64,
    ) -> CoinMarketData {
        let usdc_total_supply: usize = fetch_usdc_reserve()
            .await
            .try_into()
            .expect("Error converting Felt to usize");

        let usdc_scaled = (usdc_total_supply as f64) / 10_f64.powf(6.0); // Starknet USDC has 6 Decimals https://voyager.online/token/0x053c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8#readFunctions

        let decimals = 18.0; // ERC-20 Token by default use a value of 18 for decimals https://docs.openzeppelin.com/contracts/3.x/erc20#:~:text=By%20default%2C%20ERC20%20uses%20a%20value%20of%2018%20for%20decimals%20.
        let reserve_a: u128;
        let scaled_reserve_a: f64;
        match token_name {
            "STRK" => {
                let contract_address =
                    felt!("0x4718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d");
                reserve_a = fetch_token_reserve(contract_address)
                    .await
                    .try_into()
                    .expect("Failed converting felt u128");

                scaled_reserve_a = (reserve_a as f64) / 10_f64.powf(decimals);
            }
            "ETH" => {
                let contract_address =
                    felt!("0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");
                reserve_a = fetch_token_reserve(contract_address)
                    .await
                    .try_into()
                    .expect("Failed converting felt to u128");
                scaled_reserve_a = (reserve_a as f64) / 10_f64.powf(decimals);
            }
            "BROTHER" => {
                let contract_address =
                    felt!("0x3b405a98c9e795d427fe82cdeeeed803f221b52471e3a757574a2b4180793ee");
                reserve_a = fetch_token_reserve(contract_address)
                    .await
                    .try_into()
                    .expect("Failed converting felt u128");
                println!("brother supply: {reserve_a}");

                scaled_reserve_a = (reserve_a as f64) / 10_f64.powf(decimals);
            }
            _ => panic!("Token name didnt match supported addresses: see `tokens.rs`"),
        }

        let mut res = CoinMarketData {
            price,
            reserve_a: scaled_reserve_a as f64,
            reserve_b: usdc_scaled,
            volume_24h,
            price_change_24h,
            liquidity: 0.0, //default
            tvl: STARKNET_TVL_ESTIMATE,
            apy: 0.0,
            risk_score: 0.0,
        };
        res.calculate_metrics().expect("error computing metrics");

        res
    }

    pub fn calculate_metrics(&mut self) -> Result<(), ComputeError> {
        if self.reserve_a == 0.0 || self.reserve_b == 0.0 {
            return Err(ComputeError::InvalidPool);
        }

        self.liquidity = 2.0 * (self.reserve_a * self.reserve_b).sqrt();

        self.tvl = (self.reserve_a + self.reserve_b) * self.price;

        self.apy = self.estimate_base_apy()?;

        self.risk_score = calculate_risk_score(self.tvl, self.volume_24h, self.price_change_24h);

        Ok(())
    }

    pub fn estimate_base_apy(&self) -> Result<f64, ComputeError> {
        if self.tvl <= 0.0 {
            return Err(ComputeError::InvalidPool);
        }
        // no fee computation here, since there is no way yet to fetch the yield farming protocol fee rates, the ai agent will have it injected in its knowledge base
        let yearly_volume = self.volume_24h * 365.0;
        let base_apy = (yearly_volume / self.tvl) * 100.0;

        Ok(base_apy)
    }
}
