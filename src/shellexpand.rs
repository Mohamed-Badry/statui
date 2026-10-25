use std::path::PathBuf;

use crate::config::{BodyConfig, Endpoint};

pub fn expand_url(endpoint: &mut Endpoint) -> Result<(), color_eyre::Report> {
    endpoint.url = shellexpand::env(&endpoint.url)
        .map_err(|e| color_eyre::eyre::eyre!("In endpoint '{}': {}", endpoint.name, e))?
        .into_owned();
    Ok(())
}

pub fn expand_auth(endpoint: &mut Endpoint) -> Result<(), color_eyre::Report> {
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
    Ok(())
}
pub fn expand_body(endpoint: &mut Endpoint) -> Result<(), color_eyre::Report> {
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
    Ok(())
}
pub fn expand_headers(endpoint: &mut Endpoint) -> Result<(), color_eyre::Report> {
    // shell expansion for headers
    for (key, val) in endpoint.headers.iter_mut() {
        *val = shellexpand::env(val)
            .map_err(|e| {
                color_eyre::eyre::eyre!("In endpoint '{}' header '{}': {}", endpoint.name, key, e)
            })?
            .into_owned();
    }
    Ok(())
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
