pub use ::config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
  pub host: String,
  pub port: String,
  pub pg: deadpool_postgres::Config,
}

impl Config {
  pub fn load() -> Result<Self, ConfigError> {
    let mut cfg = ::config::Config::new();
    cfg.merge(::config::Environment::new())?;
    cfg.try_into()
  }
}
