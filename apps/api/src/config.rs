use std::net::SocketAddr;

use crate::constants::{defaults, env_keys};


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub api_addr: SocketAddr,
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum ConfigError {
    #[error("{key} must be a socket address such as {example}, got {value:?}")]
    InvalidSocketAddr {
        key: &'static str,
        example: &'static str,
        value: String,
    },
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_lookup(|key| std::env::var(key).ok())
    }

    pub fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Result<Self, ConfigError> {
        let raw_addr =
            lookup(env_keys::API_ADDR).unwrap_or_else(|| defaults::API_ADDR.to_string());
        let api_addr = raw_addr
            .parse()
            .map_err(|_| ConfigError::InvalidSocketAddr {
                key: env_keys::API_ADDR,
                example: defaults::API_ADDR,
                value: raw_addr.clone(),
            })?;

        Ok(Self { api_addr })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_default_addr_when_unset() {
        let config = Config::from_lookup(|_| None).unwrap();

        assert_eq!(config.api_addr, defaults::API_ADDR.parse().unwrap());
    }

    #[test]
    fn reads_addr_from_its_env_key() {
        let config = Config::from_lookup(|key| {
            (key == env_keys::API_ADDR).then(|| "127.0.0.1:8080".to_string())
        })
        .unwrap();

        assert_eq!(config.api_addr, "127.0.0.1:8080".parse().unwrap());
    }

    #[test]
    fn rejects_invalid_addr() {
        let error = Config::from_lookup(|_| Some("not-an-addr".to_string())).unwrap_err();

        assert_eq!(
            error,
            ConfigError::InvalidSocketAddr {
                key: env_keys::API_ADDR,
                example: defaults::API_ADDR,
                value: "not-an-addr".to_string(),
            }
        );
    }
}
