#![allow(dead_code)]

use std::time::Duration;

#[cfg(not(feature = "sentry_error"))]
use log::warn;

use serde::{Deserialize, Serialize};

use async_std::{
    fs::{self, File},
    io::prelude::*,
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub sentry: Option<SentryConfig>,
    pub search: Option<SearchConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub html_files: Option<String>,
    pub listen_address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SentryConfig {
    pub dsn: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchConfig {
    pub suggestion_timeout: Option<u64>,
    pub suggestion_sources: Option<String>,
}

impl Config {
    /// Returns the configured suggestion timeout or its default value if not set
    pub fn get_suggestion_timeout(&self) -> Duration {
        let amount = self
            .search
            .as_ref()
            .and_then(|i| i.suggestion_timeout)
            .unwrap_or(100);

        Duration::from_millis(amount)
    }

    /// Returns the configured suggestion source files or its default value if not set
    pub fn get_suggestion_sources(&self) -> &str {
        self.search
            .as_ref()
            .and_then(|i| i.suggestion_sources.as_ref().map(|i| i.as_str()))
            .unwrap_or("./suggestions")
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            html_files: Some(String::from("html/assets")),
            listen_address: String::from("127.0.0.1:8080"),
        }
    }
}

impl ServerConfig {
    pub fn get_html_files(&self) -> &str {
        self.html_files.as_deref().unwrap_or("html/assets")
    }

    pub fn get_locale_path(&self) -> &str {
        "./locales"
    }
}

impl Config {
    /// Create a new config object
    pub async fn new() -> Result<Self, String> {
        let config_file = std::env::var("JOTOBA_CONFIG")
            .map(|i| Path::new(&i).to_owned())
            .unwrap_or(Self::get_config_file().await?);

        let config = if !config_file.exists().await
            // Check if file is empty
            || fs::metadata(&config_file)
                .await
                .map(|i| i.len())
                .unwrap_or(1)
                == 0
        {
            Self::default().save().await?
        } else {
            let conf_data = fs::read_to_string(&config_file)
                .await
                .map_err(|e| e.to_string())?;

            toml::from_str(&conf_data).map_err(|e| e.to_string())?
        };

        // Warn if sentry is configured but feature not enabled
        #[cfg(not(feature = "sentry_error"))]
        if let Some(ref sentry) = config.sentry {
            if !sentry.dsn.is_empty() {
                warn!("Sentry configured but not available. Build with \"sentry_error\" feature");
            }
        }

        Ok(config)
    }

    // Save the config
    pub async fn save(self) -> Result<Self, String> {
        let config_file = Self::get_config_file().await?;

        let s = toml::to_string_pretty(&self).map_err(|e| e.to_string())?;
        let mut f = File::create(&config_file)
            .await
            .map_err(|e| e.to_string())?;
        f.write_all(&s.as_bytes())
            .await
            .map_err(|e| e.to_string())?;

        Ok(self)
    }

    // load a config
    pub async fn load(&mut self) -> Result<(), String> {
        let config_file = Self::get_config_file().await?;

        let conf_data = fs::read_to_string(&config_file)
            .await
            .map_err(|e| e.to_string())?;
        *self = toml::from_str(&conf_data).map_err(|e| e.to_string())?;

        Ok(())
    }

    // Create missing folders and return the config file
    pub async fn get_config_file() -> Result<PathBuf, String> {
        let conf_dir: PathBuf = Path::new("./").join("data");

        if !conf_dir.exists().await {
            fs::create_dir_all(&conf_dir)
                .await
                .map_err(|e| e.to_string())?;
        }

        Ok(conf_dir.join("config.toml"))
    }
}
