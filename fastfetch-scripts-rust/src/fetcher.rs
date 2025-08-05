use crate::error::Result;
use crate::config::Config;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FetchContext {
    pub username: String,
    pub config: Config,
}

impl FetchContext {
    pub fn new(platform: &str) -> Result<Self> {
        let config = Config::load()?;
        let username = config.validate_platform(platform)?;
        Ok(Self { username, config })
    }
}

pub trait Fetcher {
    fn platform_name() -> &'static str;
    fn fetch(&self, ctx: &FetchContext, param: &str) -> Result<String>;
    fn supported_params() -> &'static [&'static str];
    
    fn validate_param(&self, param: &str) -> Result<()> {
        if Self::supported_params().contains(&param) {
            Ok(())
        } else {
            Err(crate::error::FetchError::invalid_param(Self::platform_name(), param))
        }
    }
}

pub struct BatchFetcher<T: Fetcher> {
    fetcher: T,
}

impl<T: Fetcher> BatchFetcher<T> {
    pub fn new(fetcher: T) -> Self {
        Self { fetcher }
    }
    
    pub fn fetch_multiple(&self, ctx: &FetchContext, params: &[&str]) -> HashMap<String, Result<String>> {
        let mut results = HashMap::new();
        
        for &param in params {
            let result = self.fetcher.validate_param(param)
                .and_then(|_| self.fetcher.fetch(ctx, param));
            results.insert(param.to_string(), result);
        }
        
        results
    }
}
