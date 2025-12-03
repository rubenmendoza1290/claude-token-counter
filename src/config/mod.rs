use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Configuration structure that holds the API key
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
}

impl Config {
    /// Get the path to the config file
    /// Returns: ~/.config/claude-token-counter/config.json
    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Could not find config directory")?
            .join("claude-token-counter");

        Ok(config_dir.join("config.json"))
    }

    /// Load configuration from disk
    /// Returns the Config if it exists, or an error if not found
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Could not read config file at {:?}", path))?;

        let config: Config = serde_json::from_str(&contents)
            .context("Could not parse config file")?;

        Ok(config)
    }

    /// Save configuration to disk
    /// Creates the directory if it doesn't exist
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;

        // Create the directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .context("Could not create config directory")?;
        }

        // Serialize and write the config
        let contents = serde_json::to_string_pretty(self)
            .context("Could not serialize config")?;

        fs::write(&path, contents)
            .with_context(|| format!("Could not write config file to {:?}", path))?;

        // Set restrictive permissions on Unix-like systems (macOS, Linux)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&path)?.permissions();
            perms.set_mode(0o600); // Read/write for owner only
            fs::set_permissions(&path, perms)?;
        }

        println!("Configuration saved to: {:?}", path);
        Ok(())
    }

    /// Create a new Config with the given API key
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}
