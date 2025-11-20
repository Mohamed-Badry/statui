use color_eyre::Result;
use config::{Config, File};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

const APP_QUALIFIER: &str = "com";
const APP_ORGANIZATION: &str = "statui";
const APP_NAME: &str = "statui";

/// Default configuration for all endpoints
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct StatuiConfig {
    pub default_interval: u64,
    pub default_timeout: u64,

    #[serde(default)]
    pub endpoints: Vec<Endpoint>,
}

impl Default for StatuiConfig {
    fn default() -> Self {
        Self {
            default_interval: 60,
            default_timeout: 5,
            endpoints: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Endpoint {
    pub name: String,
    pub url: String,

    // -- Optional Overrides --
    pub interval: Option<u64>,
    pub timeout: Option<u64>,
    pub method: Option<String>,

    #[serde(default)]
    pub headers: HashMap<String, String>,
}

/// Function to build a Configuration by combining the default config,
/// the global app config (in ~/.config/statui/config.toml for linux or wherever
/// it is for other systems), and the local statui.toml in the current directory.
pub fn load_config() -> Result<StatuiConfig> {
    
    // default config
    let mut builder = Config::builder().add_source(Config::try_from(&StatuiConfig::default())?);

    if let Some(proj_dirs) = ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME) {
        // merge global config
        let global_config_path = proj_dirs.config_dir().join("config.toml");
        builder = builder.add_source(File::from(global_config_path).required(false));
    }

    // merge local config
    builder = builder.add_source(File::from(Path::new("statui.toml")).required(false));

    let config = builder.build()?.try_deserialize::<StatuiConfig>()?;
    Ok(config)
}
