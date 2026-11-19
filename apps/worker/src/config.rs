use std::fmt;
use std::time::Duration;

pub mod env_keys {
    pub const DATABASE_URL: &str = "DATABASE_URL";
    pub const POLL_INTERVAL_MS: &str = "AHLAN_WORKER_POLL_INTERVAL_MS";
}

pub mod defaults {
    use std::time::Duration;

    pub const DATABASE_URL_EXAMPLE: &str =
        "postgres://postgres:postgres@localhost:5433/ahlan_commerce";
    pub const DB_MAX_CONNECTIONS: usize = 5;
    pub const DB_ACQUIRE_TIMEOUT: Duration = Duration::from_secs(3);
    pub const POLL_INTERVAL: Duration = Duration::from_secs(2);
}

#[derive(Clone, PartialEq, Eq)]
pub struct Config {
    pub database_url: String,
    pub poll_interval: Duration,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("database_url", &"<redacted>")
            .field("poll_interval", &self.poll_interval)
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum ConfigError {
    #[error("{key} must be set, for example {example}")]
    Missing {
        key: &'static str,
        example: &'static str,
    },

    #[error("{key} must be a positive number of milliseconds, got {value:?}")]
    InvalidPollInterval {
        key: &'static str,
        value: String,
    },
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_lookup(|key| std::env::var(key).ok())
    }

    pub fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Result<Self, ConfigError> {
        let database_url = lookup(env_keys::DATABASE_URL)
            .filter(|url| !url.trim().is_empty())
            .ok_or(ConfigError::Missing {
                key: env_keys::DATABASE_URL,
                example: defaults::DATABASE_URL_EXAMPLE,
            })?;

        let poll_interval = match lookup(env_keys::POLL_INTERVAL_MS) {
            None => defaults::POLL_INTERVAL,
            Some(raw) => {
                let millis = raw.parse::<u64>().map_err(|_| ConfigError::InvalidPollInterval {
                    key: env_keys::POLL_INTERVAL_MS,
                    value: raw.clone(),
                })?;
                if millis == 0 {
                    return Err(ConfigError::InvalidPollInterval {
                        key: env_keys::POLL_INTERVAL_MS,
                        value: raw,
                    });
                }
                Duration::from_millis(millis)
            }
        };

        Ok(Self {
            database_url,
            poll_interval,
        })
    }
}
