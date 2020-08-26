use config::{Config, ConfigError, Environment, File};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Configuration {}

impl Configuration {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("config/default"))?;

        s.merge(
            File::with_name(&format!(
                "config/{}",
                env::var("LOLI_ENVIRONMENT").unwrap_or("development".into())
            ))
            .required(false),
        )?;

        s.merge(File::with_name("config/local").required(false))?;
        s.merge(Environment::with_prefix("loli"))?;

        s.try_into()
    }
}

pub static CONFIG: Lazy<Configuration> =
    Lazy::new(|| Configuration::new().expect("failed to load configuration"));
