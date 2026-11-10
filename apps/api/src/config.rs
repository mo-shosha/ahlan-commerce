use std::fmt;
use std::net::SocketAddr;

use crate::constants::{defaults, env_keys};


#[derive(Clone, PartialEq, Eq)]
pub struct Config {
    pub api_addr: SocketAddr,
    pub database_url: String,
}


impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("api_addr", &self.api_addr)
            .field("database_url", &"<redacted>")
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum ConfigError {
    #[error("{key} must be a socket address such as {example}, got {value:?}")]
    InvalidSocketAddr {
        key: &'static str,
        example: &'static str,
        value: String,
    },

    #[error("{key} must be set, for example {example}")]
    Missing {
        key: &'static str,
        example: &'static str,
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

        let database_url = lookup(env_keys::DATABASE_URL)
            .filter(|url| !url.trim().is_empty())
            .ok_or(ConfigError::Missing {
                key: env_keys::DATABASE_URL,
                example: defaults::DATABASE_URL_EXAMPLE,
            })?;

        Ok(Self { api_addr, database_url })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATABASE_URL: &str = "postgres://user:secret@localhost:5432/ahlan_commerce";

    fn only_database_url(key: &str) -> Option<String> {
        (key == env_keys::DATABASE_URL).then(|| DATABASE_URL.to_string())
    }

    #[test]
    fn uses_default_addr_when_unset() {
        let config = Config::from_lookup(only_database_url).unwrap();

        assert_eq!(config.api_addr, defaults::API_ADDR.parse().unwrap());
    }

    #[test]
    fn reads_addr_from_its_env_key() {
        let config = Config::from_lookup(|key| match key {
            env_keys::API_ADDR => Some("127.0.0.1:8080".to_string()),
            _ => only_database_url(key),
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

    #[test]
    fn reads_database_url_from_its_env_key() {
        let config = Config::from_lookup(only_database_url).unwrap();

        assert_eq!(config.database_url, DATABASE_URL);
    }

    #[test]
    fn requires_database_url() {
        let error = Config::from_lookup(|_| None).unwrap_err();

        assert_eq!(
            error,
            ConfigError::Missing {
                key: env_keys::DATABASE_URL,
                example: defaults::DATABASE_URL_EXAMPLE,
            }
        );
    }

    #[test]
    fn debug_output_hides_database_url() {
        let config = Config::from_lookup(only_database_url).unwrap();

        assert!(!format!("{config:?}").contains("secret"));
    }
}
