use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub pkmgr: PkmgrConfig,
    pub defaults: Defaults,
    pub paths: Paths,
    pub network: Network,
    pub security: Security,
    pub repositories: HashMap<String, String>,
    pub aliases: HashMap<String, String>,
    pub language_defaults: LanguageDefaults,
    pub binary_sources: BinarySources,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PkmgrConfig {
    pub version: String,
    pub last_update_check: Option<String>,
    pub install_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Defaults {
    pub install_location: String,
    pub prefer_binary: bool,
    pub allow_prerelease: bool,
    pub parallel_downloads: u32,
    pub parallel_operations: u32,
    pub color_output: String,
    pub emoji_enabled: bool,
    pub progress_style: String,
    pub verbosity: String,
    pub pager: String,
    pub auto_cleanup: bool,
    pub auto_update_check: bool,
    pub confirm_major_updates: bool,
    pub keep_downloads: bool,
    pub use_cache: bool,
    pub auto_fix: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Paths {
    pub cache_dir: String,
    pub data_dir: String,
    pub config_dir: String,
    pub install_dir: String,
    pub iso_dir: String,
    pub temp_dir: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Network {
    pub timeout: u32,
    pub retry_count: u32,
    pub retry_delay: u32,
    pub bandwidth_limit: u64,
    pub proxy: String,
    pub parallel_downloads: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Security {
    pub verify_signatures: bool,
    pub verify_checksums: bool,
    pub allow_untrusted: bool,
    pub keyserver: String,
    pub key_refresh_days: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LanguageDefaults {
    pub php: String,
    pub python: String,
    pub node: String,
    pub ruby: String,
    pub go: String,
    pub rust: String,
    pub java: String,
    pub dotnet: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BinarySources {
    pub prefer_github: bool,
    pub include_prerelease: bool,
    pub asset_preference: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            pkmgr: PkmgrConfig {
                version: "1.0.0".to_string(),
                last_update_check: None,
                install_id: uuid::Uuid::new_v4().to_string(),
            },
            defaults: Defaults {
                install_location: "auto".to_string(),
                prefer_binary: true,
                allow_prerelease: false,
                parallel_downloads: 4,
                parallel_operations: 2,
                color_output: "auto".to_string(),
                emoji_enabled: true,
                progress_style: "bar".to_string(),
                verbosity: "normal".to_string(),
                pager: "auto".to_string(),
                auto_cleanup: true,
                auto_update_check: true,
                confirm_major_updates: true,
                keep_downloads: false,
                use_cache: true,
                auto_fix: true,
            },
            paths: Paths {
                cache_dir: "~/.cache/pkmgr".to_string(),
                data_dir: "~/.local/share/pkmgr".to_string(),
                config_dir: "~/.config/pkmgr".to_string(),
                install_dir: "~/.local".to_string(),
                iso_dir: "~/Downloads/ISOs".to_string(),
                temp_dir: "/tmp/pkmgr".to_string(),
            },
            network: Network {
                timeout: 30,
                retry_count: 3,
                retry_delay: 5,
                bandwidth_limit: 0,
                proxy: String::new(),
                parallel_downloads: 4,
            },
            security: Security {
                verify_signatures: true,
                verify_checksums: true,
                allow_untrusted: false,
                keyserver: "hkps://keys.openpgp.org".to_string(),
                key_refresh_days: 30,
            },
            repositories: HashMap::new(),
            aliases: {
                let mut aliases = HashMap::new();
                aliases.insert("i".to_string(), "install".to_string());
                aliases.insert("r".to_string(), "remove".to_string());
                aliases.insert("u".to_string(), "update".to_string());
                aliases.insert("s".to_string(), "search".to_string());
                aliases.insert("ls".to_string(), "list".to_string());
                aliases.insert("rm".to_string(), "remove".to_string());
                aliases.insert("up".to_string(), "update".to_string());
                aliases.insert("dl".to_string(), "install".to_string());
                aliases
            },
            language_defaults: LanguageDefaults {
                php: "7.4".to_string(),
                python: "3".to_string(),
                node: "20".to_string(),
                ruby: "3.2".to_string(),
                go: "1.21".to_string(),
                rust: "1.75".to_string(),
                java: "11".to_string(),
                dotnet: "8.0".to_string(),
            },
            binary_sources: BinarySources {
                prefer_github: true,
                include_prerelease: false,
                asset_preference: vec![
                    "static".to_string(),
                    "appimage".to_string(),
                    "archive".to_string(),
                ],
            },
        }
    }
}

impl Config {
    pub async fn load() -> Result<Self> {
        let config_dir = Self::get_config_dir()?;
        let config_file = config_dir.join("config.toml");

        if config_file.exists() {
            let content = fs::read_to_string(&config_file)
                .await
                .context("Failed to read config file")?;

            let config: Config = toml::from_str(&content)
                .context("Failed to parse config file")?;

            Ok(config)
        } else {
            // Create default config and save it
            let config = Self::default();
            config.save().await?;
            Ok(config)
        }
    }

    pub async fn save(&self) -> Result<()> {
        let config_dir = Self::get_config_dir()?;
        fs::create_dir_all(&config_dir).await?;

        let config_file = config_dir.join("config.toml");
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;

        fs::write(&config_file, content).await?;
        Ok(())
    }

    pub fn get_config_dir() -> Result<PathBuf> {
        let home_dir = dirs::home_dir().context("Failed to get home directory")?;
        Ok(home_dir.join(".config").join("pkmgr"))
    }

    pub fn get_cache_dir(&self) -> Result<PathBuf> {
        let path = shellexpand::tilde(&self.paths.cache_dir).to_string();
        Ok(PathBuf::from(path))
    }

    pub fn get_data_dir(&self) -> Result<PathBuf> {
        let path = shellexpand::tilde(&self.paths.data_dir).to_string();
        Ok(PathBuf::from(path))
    }

    pub fn get_install_dir(&self) -> Result<PathBuf> {
        let path = shellexpand::tilde(&self.paths.install_dir).to_string();
        Ok(PathBuf::from(path))
    }

    pub fn get_iso_dir(&self) -> Result<PathBuf> {
        let path = shellexpand::tilde(&self.paths.iso_dir).to_string();
        Ok(PathBuf::from(path))
    }

    pub fn resolve_alias(&self, command: &str) -> String {
        self.aliases.get(command).cloned().unwrap_or_else(|| command.to_string())
    }
}