use color_eyre::Result;
use config::{Config, File};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const APP_QUALIFIER: &str = "com";
const APP_ORGANIZATION: &str = "statui";
const APP_NAME: &str = "statui";

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct FileBody {
    pub file: PathBuf,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum BodyConfig {
    // Matches: body = { file = "path/to/payload.json" }
    File(FileBody),

    // Matches: body = '{"raw": "string"}' or body = "plain text"
    Raw(String),

    // Matches: body = { key = "value", count = 42 } or [1, 2, 3]
    Json(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct BasicAuth {
    pub username: String,
    pub password: Option<String>,
}

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
    pub body: Option<BodyConfig>,
    pub basic_auth: Option<BasicAuth>,
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

        let mut config = builder.build()?.try_deserialize::<StatuiConfig>()?;

        // TODO: Refactor this whole shell expansion shit into its own file
        for endpoint in config.endpoints.iter_mut() {
            // shell expansion for basic_auth
            if let Some(basic_auth) = &mut endpoint.basic_auth {
                basic_auth.username = shellexpand::env(&basic_auth.username)
                    .map_err(|e| {
                        color_eyre::eyre::eyre!(
                            "In endpoint '{}' basic_auth username: {}",
                            endpoint.name,
                            e
                        )
                    })?
                    .into_owned();
                if let Some(pass) = &basic_auth.password {
                    basic_auth.password = Some(
                        shellexpand::env(pass)
                            .map_err(|e| {
                                color_eyre::eyre::eyre!(
                                    "In endpoint '{}' basic_auth password: {}",
                                    endpoint.name,
                                    e
                                )
                            })?
                            .into_owned(),
                    )
                };
            };

            // shell expansion for body
            match &mut endpoint.body {
                Some(BodyConfig::File(file_body)) => {
                    let tmp_file_body = file_body.file.to_string_lossy();
                    let expanded = shellexpand::env(&tmp_file_body).map_err(|e| {
                        color_eyre::eyre::eyre!("In endpoint '{}' body file: {}", endpoint.name, e)
                    })?;
                    file_body.file = PathBuf::from(expanded.as_ref());
                }
                Some(BodyConfig::Raw(text)) => {
                    *text = shellexpand::env(text)
                        .map_err(|e| {
                            color_eyre::eyre::eyre!("In endpoint '{}' body: {}", endpoint.name, e)
                        })?
                        .into_owned();
                }
                Some(BodyConfig::Json(val)) => {
                    expand_json_value(val).map_err(|e| {
                        color_eyre::eyre::eyre!("In endpoint '{}' JSON body: {}", endpoint.name, e)
                    })?;
                }
                None => {}
            };

            endpoint.url = shellexpand::env(&endpoint.url)
                .map_err(|e| color_eyre::eyre::eyre!("In endpoint '{}': {}", endpoint.name, e))?
                .into_owned();

            // shell expansion for headers
            for (key, val) in endpoint.headers.iter_mut() {
                *val = shellexpand::env(val)
                    .map_err(|e| {
                        color_eyre::eyre::eyre!(
                            "In endpoint '{}' header '{}': {}",
                            endpoint.name,
                            key,
                            e
                        )
                    })?
                    .into_owned();
            }
        }
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

// Helper function that recursively shellexpands strings in json body
pub fn expand_json_value(val: &mut serde_json::Value) -> Result<(), color_eyre::Report> {
    match val {
        serde_json::Value::String(s) => {
            *s = shellexpand::env(s)?.into_owned();
        }
        serde_json::Value::Array(arr) => {
            for item in arr {
                expand_json_value(item)?;
            }
        }
        serde_json::Value::Object(map) => {
            for val in map.values_mut() {
                expand_json_value(val)?;
            }
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_endpoint(toml_str: &str) -> Endpoint {
        config::Config::builder()
            .add_source(config::File::from_str(toml_str, config::FileFormat::Toml))
            .build()
            .expect("Failed to build config")
            .try_deserialize()
            .expect("Failed to deserialize Endpoint")
    }

    #[test]
    fn test_deserialize_json_body() {
        let toml_str = r#"
            name = "Test JSON"
            url = "https://example.com"
            method = "POST"
            body = { count = 42, flag = true, service = "probe" }
        "#;
        let endpoint = parse_endpoint(toml_str);
        assert_eq!(endpoint.name, "Test JSON");
        assert_eq!(endpoint.method.as_deref(), Some("POST"));
        match endpoint.body {
            Some(BodyConfig::Json(val)) => {
                assert_eq!(val["count"], 42);
                assert_eq!(val["flag"], true);
                assert_eq!(val["service"], "probe");
            }
            other => panic!("Expected BodyConfig::Json, got {:?}", other),
        }
    }

    #[test]
    fn test_deserialize_raw_string_body() {
        let toml_str = r#"
            name = "Test Raw"
            url = "https://example.com"
            method = "PUT"
            body = "raw plaintext payload"
        "#;
        let endpoint = parse_endpoint(toml_str);
        assert_eq!(endpoint.method.as_deref(), Some("PUT"));
        match endpoint.body {
            Some(BodyConfig::Raw(text)) => assert_eq!(text, "raw plaintext payload"),
            other => panic!("Expected BodyConfig::Raw, got {:?}", other),
        }
    }

    #[test]
    fn test_deserialize_file_body() {
        let toml_str = r#"
            name = "Test File"
            url = "https://example.com"
            method = "POST"
            body = { file = "presets/ai_llm.toml" }
        "#;
        let endpoint = parse_endpoint(toml_str);
        match endpoint.body {
            Some(BodyConfig::File(FileBody { file })) => {
                assert_eq!(file, PathBuf::from("presets/ai_llm.toml"));
            }
            other => panic!("Expected BodyConfig::File, got {:?}", other),
        }
    }

    #[test]
    fn test_headers_inline() {
        let toml_str = r#"
            name = "Test Headers"
            url = "https://example.com"
            headers = { "Authorization" = "Bearer secret123", "X-Custom" = "hello" }
        "#;
        let endpoint = parse_endpoint(toml_str);
        assert_eq!(
            endpoint.headers.get("Authorization").map(|s| s.as_str()),
            Some("Bearer secret123")
        );
        assert_eq!(
            endpoint.headers.get("X-Custom").map(|s| s.as_str()),
            Some("hello")
        );
    }

    #[test]
    fn test_headers_subtable() {
        let toml_str = r#"
            name = "Test Subtable Headers"
            url = "https://example.com"

            [headers]
            Authorization = "Bearer env-secret"
            X-Environment = "staging"
        "#;
        let endpoint = parse_endpoint(toml_str);
        assert_eq!(
            endpoint.headers.get("Authorization").map(|s| s.as_str()),
            Some("Bearer env-secret")
        );
        assert_eq!(
            endpoint.headers.get("X-Environment").map(|s| s.as_str()),
            Some("staging")
        );
    }

    #[test]
    fn test_basic_auth_inline() {
        let toml_str = r#"
            name = "Test Basic Auth"
            url = "https://example.com"
            basic_auth = { username = "admin", password = "secret123" }
        "#;
        let endpoint = parse_endpoint(toml_str);
        assert_eq!(
            endpoint.basic_auth,
            Some(BasicAuth {
                username: "admin".to_string(),
                password: Some("secret123".to_string()),
            })
        );
    }

    #[test]
    fn test_basic_auth_subtable_without_password() {
        let toml_str = r#"
            name = "Test Basic Auth No Pass"
            url = "https://example.com"

            [basic_auth]
            username = "admin"
        "#;
        let endpoint = parse_endpoint(toml_str);
        assert_eq!(
            endpoint.basic_auth,
            Some(BasicAuth {
                username: "admin".to_string(),
                password: None,
            })
        );
    }

    #[test]
    fn test_endpoint_defaults() {
        let toml_str = r#"
            name = "Minimal Endpoint"
            url = "https://example.com"
        "#;
        let endpoint = parse_endpoint(toml_str);
        assert_eq!(endpoint.name, "Minimal Endpoint");
        assert_eq!(endpoint.method, None);
        assert_eq!(endpoint.interval, None);
        assert_eq!(endpoint.timeout, None);
        assert_eq!(endpoint.skip_cert_verification, None);
        assert!(endpoint.headers.is_empty());
        assert!(endpoint.body.is_none());
        assert_eq!(endpoint.basic_auth, None);
    }
}
