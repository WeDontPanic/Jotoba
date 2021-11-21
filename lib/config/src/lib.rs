use std::{
    fs::DirEntry,
    io::{BufReader, Read, Write},
    time::Duration,
};

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

    #[serde(skip)]
    pub asset_hash: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub html_files: Option<String>,
    pub audio_files: Option<String>,
    pub listen_address: String,
    pub storage_data: Option<String>,
    pub radical_map: Option<String>,
    pub sentences: Option<String>,
    pub img_upload_dir: Option<String>,
    pub tess_data: Option<String>,
    pub news_folder: Option<String>,
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
            audio_files: Some(String::from("html/audio")),
            listen_address: String::from("127.0.0.1:8080"),
            sentences: Some(String::from("./resources/sentences.bin")),
            storage_data: Some(String::from("./resources/storage_data")),
            radical_map: Some(String::from("./resources/radical_map")),
            img_upload_dir: Some(String::from("./img_scan_tmp")),
            tess_data: None,
            news_folder: Some(String::from("./news")),
        }
    }
}

impl ServerConfig {
    pub fn get_audio_files(&self) -> &str {
        self.audio_files.as_deref().unwrap_or("html/audio")
    }

    pub fn get_html_files(&self) -> &str {
        self.html_files.as_deref().unwrap_or("html/assets")
    }

    pub fn get_locale_path(&self) -> &str {
        "./locales"
    }

    pub fn get_news_folder(&self) -> &str {
        self.news_folder.as_deref().unwrap_or("./news")
    }
}

impl Config {
    /// Create a new config object
    pub fn new() -> Result<Self, String> {
        let config_file = std::env::var("JOTOBA_CONFIG")
            .map(|i| Path::new(&i).to_owned())
            .unwrap_or(Self::get_config_file()?);

        let mut config = if !config_file.exists()
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

        config.asset_hash = variable_asset_hash(&config).map_err(|i| i.to_string())?;

        Ok(config)
    }

    // Save the config
    fn save(self) -> Result<Self, String> {
        let config_file = Self::get_config_file()?;

        let s = toml::to_string_pretty(&self).map_err(|e| e.to_string())?;
        let mut f = File::create(&config_file).map_err(|e| e.to_string())?;
        f.write_all(&s.as_bytes()).map_err(|e| e.to_string())?;

        Ok(self)
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

fn variable_asset_hash(config: &Config) -> std::io::Result<String> {
    let asset_path = Path::new(config.server.get_html_files());
    let js_files = dir_content(&asset_path.join("js"))?;
    let css_files = dir_content(&asset_path.join("css"))?;

    let mut files = js_files
        .into_iter()
        .chain(css_files.into_iter())
        .collect::<Vec<_>>();

    files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    let mut hash = sha1::Sha1::new();
    let mut buf: Vec<u8> = vec![0u8; 100];

    for file in files {
        let mut content = BufReader::new(File::open(file)?);

        loop {
            let read = content.read(&mut buf[..])?;
            if read == 0 {
                break;
            }
            hash.update(&buf[..read]);
        }
    }

    Ok(hash.digest().to_string())
}

fn dir_content(path: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    visit_dirs(path, &mut files)?;

    let out = files
        .into_iter()
        .map(|i| i.path().clone())
        .collect::<Vec<_>>();

    Ok(out)
}

fn visit_dirs(dir: &Path, out: &mut Vec<DirEntry>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, out)?;
            } else {
                out.push(entry)
            }
        }
    }
    Ok(())
}
