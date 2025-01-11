use anyhow::Error;

#[derive(Debug)]
pub enum ModelProvider {
    OpenAI,
    // will add other providers later
}

#[derive(Debug, Clone)]
pub struct Start {
    pub provider: ProviderConfig,
    pub coingecko: String,
}
#[derive(Debug, Clone)]
pub enum ProviderConfig {
    OpenAI(String),
    // will add more
}

impl Start {
    pub fn check_env() -> Result<Self, Error> {
        // Check required Coingecko API key first
        let coingecko = std::env::var("COINGECKO_API_KEY")
            .map_err(|_| Error::msg("COINGECKO_API_KEY is required but not set in .env"))?;

        // Check for model providers
        let provider = if let Ok(openai_key) = std::env::var("OPENAI_API_KEY") {
            if !openai_key.starts_with("sk-") {
                return Err(Error::msg("OPENAI_API_KEY must start with 'sk-'"));
            }
            ProviderConfig::OpenAI(openai_key)
        } else {
            return Err(Error::msg("No valid model provider API key found. Currently supporting: OPENAI_API_KEY"));
        };

        Ok(Self {
            provider,
            coingecko,
        })
    }
}