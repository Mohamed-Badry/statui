use color_eyre::Result;
use config::{Config, File};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

const APP_QUALIFIER: &str = "com";
const APP_ORGANIZATION: &str = "statui";
const APP_NAME: &str = "statui";

/// The configuration for a single endpoint.
///
/// This maps directly to the `[[endpoints]]` block in statui.toml.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Endpoint {
    pub name: String,
    pub url: String,

    // -- Optional Overrides --
    pub interval: Option<u64>,
    pub timeout: Option<u64>,
    pub method: Option<String>,
    pub skip_cert_verification: Option<bool>,

    #[serde(default)]
    pub headers: HashMap<String, String>,
}

/// The full configuration for statui
///
/// Loaded from the file statui.toml or ~/.config/statui/config.toml.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct StatuiConfig {
    pub default_interval: u64,
    pub default_timeout: u64,

    #[serde(default)]
    pub endpoints: Vec<Endpoint>,
}

/// Default configuration for all endpoints
impl Default for StatuiConfig {
    fn default() -> Self {
        Self {
            default_interval: 60,
            default_timeout: 5,
            endpoints: Vec::new(),
        }
    }
}

impl StatuiConfig {
    /// Function to build a Configuration by combining the default config,
    /// the global app config (in ~/.config/statui/config.toml for linux or wherever
    /// it is for other systems), and the config at the path passed in the first argument
    /// (if no arguments were passed statui.toml is used by default).
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<StatuiConfig> {
        args.next();
        let local_config_path: String = match args.next() {
            Some(arg) => arg,
            None => "statui.toml".to_string(),
        };

        // default config
        let mut builder = Config::builder().add_source(Config::try_from(&StatuiConfig::default())?);

        if let Some(proj_dirs) = ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME) {
            // merge global config
            let global_config_path = proj_dirs.config_dir().join("config.toml");
            builder = builder.add_source(File::from(global_config_path).required(false));
        }

        // merge local config
        builder = builder.add_source(File::from(Path::new(&local_config_path)).required(false));

        let config = builder.build()?.try_deserialize::<StatuiConfig>()?;
        Ok(config)
    }
}

// Helper function I use in the welcome message to show the user where to put the config file
pub fn get_default_config_dir() -> String {
    if let Some(proj_dirs) = ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME) {
        proj_dirs.config_dir().to_string_lossy().to_string()
    } else {
        "your system config directory".to_string()
    }
}
