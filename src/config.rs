use config::{Config, ConfigError, Environment, File};
use once_cell::sync::Lazy;
use rusoto_core::Region;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub app: AppConfig,
    pub s3: S3Config,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub name: String,
    pub footer: Vec<FooterLink>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FooterLink {
    pub text: String,
    pub href: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S3Config {
    pub region: Region,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
}

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
