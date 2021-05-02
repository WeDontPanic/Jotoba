#![allow(dead_code)]
use serde::{Deserialize, Serialize};

use async_std::io::prelude::*;
use async_std::path::PathBuf;
use async_std::{
    fs::{self, File},
    path::Path,
};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub server: ServerConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub html_files: Option<String>,
    pub listen_address: String,
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
    pub(super) fn get_html_files(&self) -> &str {
        self.html_files
            .as_ref()
            .map(|i| i.as_str())
            .unwrap_or("html/assets")
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
