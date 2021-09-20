use std::{io::Write, time::Duration};

use serde::{Deserialize, Serialize};

use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub sentry: Option<SentryConfig>,
    pub search: Option<SearchConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub html_files: Option<String>,
    pub listen_address: String,
    pub storage_data: Option<String>,
    pub radical_map: Option<String>,
    pub sentences: Option<String>,
    pub img_upload_dir: Option<String>,
    pub tess_data: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SentryConfig {
    pub dsn: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SearchConfig {
    pub suggestion_timeout: Option<u64>,
    pub suggestion_sources: Option<String>,
    pub indexes_source: Option<String>,
    pub report_queries_after: Option<u64>,
    pub search_timeout: Option<u64>,
}

impl Config {
    /// Returns the configured search timeout or the default value `10s`
    pub fn get_search_timeout(&self) -> Duration {
        let sec = self
            .search
            .as_ref()
            .and_then(|i| i.search_timeout)
            .unwrap_or(10);
        Duration::from_secs(sec)
    }

    /// Returns the configured suggestion timeout or its default value if not set
    pub fn get_suggestion_timeout(&self) -> Duration {
        let amount = self
            .search
            .as_ref()
            .and_then(|i| i.suggestion_timeout)
            .unwrap_or(100);

        Duration::from_millis(amount)
    }

    /// Returns the configured index source files or its default value if not set
    pub fn get_indexes_source(&self) -> &str {
        self.search
            .as_ref()
            .and_then(|i| i.indexes_source.as_ref().map(|i| i.as_str()))
            .unwrap_or("./indexes")
    }

    /// Returns the configured suggestion source files or its default value if not set
    pub fn get_suggestion_sources(&self) -> &str {
        self.search
            .as_ref()
            .and_then(|i| i.suggestion_sources.as_ref().map(|i| i.as_str()))
            .unwrap_or("./suggestions")
    }

    /// Returns the configured query report timeout
    pub fn get_query_report_timeout(&self) -> Duration {
        let timeout = self
            .search
            .as_ref()
            .and_then(|i| i.report_queries_after)
            .unwrap_or(4);

        Duration::from_secs(timeout)
    }

    /// Returns the configured (or default) path for storage data
    pub fn get_storage_data_path(&self) -> String {
        self.server
            .storage_data
            .as_ref()
            .cloned()
            .unwrap_or(ServerConfig::default().storage_data.unwrap())
    }

    /// Returns the configured (or default) path for the sentences resource file
    pub fn get_sentences_path(&self) -> String {
        self.server
            .sentences
            .as_ref()
            .cloned()
            .unwrap_or(ServerConfig::default().sentences.unwrap())
    }

    /// Returns the configured (or default) path for the radical map
    pub fn get_radical_map_path(&self) -> String {
        self.server
            .radical_map
            .as_ref()
            .cloned()
            .unwrap_or(ServerConfig::default().radical_map.unwrap())
    }

    /// Returns the configured (or default) path for the radical map
    pub fn get_img_scan_upload_path(&self) -> String {
        self.server
            .img_upload_dir
            .as_ref()
            .cloned()
            .unwrap_or(ServerConfig::default().img_upload_dir.unwrap())
    }
}

impl Default for ServerConfig {
    #[inline]
    fn default() -> Self {
        Self {
            html_files: Some(String::from("html/assets")),
            listen_address: String::from("127.0.0.1:8080"),
            sentences: Some(String::from("./resources/sentences.bin")),
            storage_data: Some(String::from("./resources/storage_data")),
            radical_map: Some(String::from("./resources/radical_map")),
            img_upload_dir: Some(String::from("./img_scan_tmp")),
            tess_data: None,
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
    pub fn new() -> Result<Self, String> {
        let config_file = std::env::var("JOTOBA_CONFIG")
            .map(|i| Path::new(&i).to_owned())
            .unwrap_or(Self::get_config_file()?);

        let config = if !config_file.exists()
            // Check if file is empty
            || fs::metadata(&config_file).map(|i| i.len()).unwrap_or(1)
                == 0
        {
            Self::default().save()?
        } else {
            let conf_data = fs::read_to_string(&config_file).map_err(|e| e.to_string())?;
            toml::from_str(&conf_data).map_err(|e| e.to_string())?
        };

        /*
        // Warn if sentry is configured but feature not enabled
        #[cfg(not(feature = "sentry_error"))]
        if let Some(ref sentry) = config.sentry {
            if !sentry.dsn.is_empty() {
                warn!("Sentry configured but not available. Build with \"sentry_error\" feature");
            }
        }
        */

        Ok(config)
    }

    // Save the config
    pub fn save(self) -> Result<Self, String> {
        let config_file = Self::get_config_file()?;

        let s = toml::to_string_pretty(&self).map_err(|e| e.to_string())?;
        let mut f = File::create(&config_file).map_err(|e| e.to_string())?;
        f.write_all(&s.as_bytes()).map_err(|e| e.to_string())?;

        Ok(self)
    }

    // load a config
    pub fn load(&mut self) -> Result<(), String> {
        let config_file = Self::get_config_file()?;

        let conf_data = fs::read_to_string(&config_file).map_err(|e| e.to_string())?;
        *self = toml::from_str(&conf_data).map_err(|e| e.to_string())?;

        Ok(())
    }

    // Create missing folders and return the config file
    pub fn get_config_file() -> Result<PathBuf, String> {
        let conf_dir: PathBuf = Path::new("./").join("data");

        if !conf_dir.exists() {
            fs::create_dir_all(&conf_dir).map_err(|e| e.to_string())?;
        }

        Ok(conf_dir.join("config.toml"))
    }
}
