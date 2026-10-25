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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{BasicAuth, Endpoint, FileBody};
    use serial_test::serial;
    use std::collections::HashMap;

    fn parse_endpoint(toml_str: &str) -> Endpoint {
        config::Config::builder()
            .add_source(config::File::from_str(toml_str, config::FileFormat::Toml))
            .build()
            .expect("Failed to build config")
            .try_deserialize()
            .expect("Failed to deserialize Endpoint")
    }

    fn mock_endpoint(name: &str, url: &str) -> Endpoint {
        Endpoint {
            name: name.to_string(),
            url: url.to_string(),
            interval: None,
            timeout: None,
            method: None,
            skip_cert_verification: None,
            headers: HashMap::new(),
            body: None,
            basic_auth: None,
        }
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
    #[serial]
    fn test_expand_json_nested_and_types() {
        unsafe {
            std::env::set_var("STATUI_TEST_JSON_USER", "octocat");
            std::env::set_var("STATUI_TEST_JSON_ENV", "production");
        }

        let mut val = serde_json::json!({
            "user": "${STATUI_TEST_JSON_USER}",
            "environment": "$STATUI_TEST_JSON_ENV",
            "fallback": "${STATUI_TEST_NONEXISTENT_VAR_XYZ:-default_fallback}",
            "numeric_count": 100,
            "float_val": 3.1415,
            "is_active": true,
            "empty_val": null,
            "nested": {
                "arr": [
                    "literal",
                    "prefix_${STATUI_TEST_JSON_USER}_suffix",
                    {
                        "deep_key": "${STATUI_TEST_JSON_ENV}"
                    }
                ]
            }
        });

        expand_json_value(&mut val).expect("Failed to expand json value");

        assert_eq!(val["user"], "octocat");
        assert_eq!(val["environment"], "production");
        assert_eq!(val["fallback"], "default_fallback");
        assert_eq!(val["numeric_count"], 100);
        assert_eq!(val["float_val"], 3.1415);
        assert_eq!(val["is_active"], true);
        assert_eq!(val["empty_val"], serde_json::Value::Null);
        assert_eq!(val["nested"]["arr"][0], "literal");
        assert_eq!(val["nested"]["arr"][1], "prefix_octocat_suffix");
        assert_eq!(val["nested"]["arr"][2]["deep_key"], "production");
    }

    #[test]
    fn test_expand_json_escaped_dollars() {
        let mut val = serde_json::json!({
            "price": "$$99.99",
            "formula": "$$x + $$y = $$z",
            "single_dollar_end": "Total: 50$$"
        });

        expand_json_value(&mut val).expect("Failed to expand escaped dollars");

        assert_eq!(val["price"], "$99.99");
        assert_eq!(val["formula"], "$x + $y = $z");
        assert_eq!(val["single_dollar_end"], "Total: 50$");
    }

    #[test]
    fn test_expand_json_missing_var_error() {
        let mut val = serde_json::json!({
            "data": "${STATUI_DEFINITELY_UNSET_VAR_XYZ123}"
        });

        let err = expand_json_value(&mut val).unwrap_err();
        let err_msg = err.to_string();
        assert!(
            err_msg.contains("STATUI_DEFINITELY_UNSET_VAR_XYZ123"),
            "Error should mention missing variable name: {err_msg}"
        );
    }

    #[test]
    #[serial]
    fn test_expand_empty_var() {
        unsafe {
            std::env::set_var("STATUI_TEST_EMPTY_VAR", "");
        }
        let mut val = serde_json::json!({
            "value": "start_${STATUI_TEST_EMPTY_VAR}_end",
            "just_var": "$STATUI_TEST_EMPTY_VAR"
        });
        expand_json_value(&mut val).expect("Expansion with empty var should succeed");
        assert_eq!(val["value"], "start__end");
        assert_eq!(val["just_var"], "");
    }

    #[test]
    #[serial]
    fn test_expand_url_all_scenarios() {
        unsafe {
            std::env::set_var("STATUI_TEST_HOST", "api.example.com");
            std::env::set_var("STATUI_TEST_PATH", "v2/health");
        }

        // 1. Multiple variables + fallback ports + query params + escaped $$
        let mut ep = mock_endpoint(
            "url-test",
            "${STATUI_TEST_PROTO:-https}://${STATUI_TEST_HOST}:${STATUI_TEST_PORT:-8443}/${STATUI_TEST_PATH}?query=$$token",
        );
        expand_url(&mut ep).expect("URL expansion should succeed");
        assert_eq!(
            ep.url,
            "https://api.example.com:8443/v2/health?query=$token"
        );

        // 2. Missing variable without fallback
        let mut ep_err = mock_endpoint("failing-url-ep", "https://${STATUI_UNSET_URL_VAR}/status");
        let err = expand_url(&mut ep_err).unwrap_err();
        assert!(
            err.to_string().contains("In endpoint 'failing-url-ep'"),
            "Error must mention endpoint name: {}",
            err
        );
        assert!(
            err.to_string().contains("STATUI_UNSET_URL_VAR"),
            "Error must mention missing variable: {}",
            err
        );
    }

    #[test]
    #[serial]
    fn test_expand_auth_all_scenarios() {
        unsafe {
            std::env::set_var("STATUI_TEST_AUTH_USER", "superadmin");
            std::env::set_var("STATUI_TEST_AUTH_PASS", "s3cr3t_p@ss");
        }

        // 1. Both username and password with env vars
        let mut ep = mock_endpoint("auth-test", "https://example.com");
        ep.basic_auth = Some(BasicAuth {
            username: "${STATUI_TEST_AUTH_USER}".to_string(),
            password: Some("${STATUI_TEST_AUTH_PASS}".to_string()),
        });
        expand_auth(&mut ep).expect("Auth expansion should succeed");
        assert_eq!(
            ep.basic_auth,
            Some(BasicAuth {
                username: "superadmin".to_string(),
                password: Some("s3cr3t_p@ss".to_string()),
            })
        );

        // 2. Fallbacks when variables are unset
        let mut ep_fallback = mock_endpoint("auth-fallback", "https://example.com");
        ep_fallback.basic_auth = Some(BasicAuth {
            username: "${STATUI_UNSET_USER_XYZ:-guest}".to_string(),
            password: Some("${STATUI_UNSET_PASS_XYZ:-anonymous}".to_string()),
        });
        expand_auth(&mut ep_fallback).expect("Auth fallback should succeed");
        assert_eq!(
            ep_fallback.basic_auth,
            Some(BasicAuth {
                username: "guest".to_string(),
                password: Some("anonymous".to_string()),
            })
        );

        // 3. Password is None
        let mut ep_no_pass = mock_endpoint("auth-no-pass", "https://example.com");
        ep_no_pass.basic_auth = Some(BasicAuth {
            username: "${STATUI_TEST_AUTH_USER}".to_string(),
            password: None,
        });
        expand_auth(&mut ep_no_pass).expect("Auth without password should succeed");
        assert_eq!(
            ep_no_pass.basic_auth,
            Some(BasicAuth {
                username: "superadmin".to_string(),
                password: None,
            })
        );

        // 4. Missing username error context
        let mut ep_err_user = mock_endpoint("auth-err-user", "https://example.com");
        ep_err_user.basic_auth = Some(BasicAuth {
            username: "$STATUI_UNSET_USER_VAR_ERR".to_string(),
            password: None,
        });
        let err = expand_auth(&mut ep_err_user).unwrap_err();
        assert!(
            err.to_string()
                .contains("In endpoint 'auth-err-user' basic_auth username")
        );

        // 5. Missing password error context
        let mut ep_err_pass = mock_endpoint("auth-err-pass", "https://example.com");
        ep_err_pass.basic_auth = Some(BasicAuth {
            username: "admin".to_string(),
            password: Some("$STATUI_UNSET_PASS_VAR_ERR".to_string()),
        });
        let err = expand_auth(&mut ep_err_pass).unwrap_err();
        assert!(
            err.to_string()
                .contains("In endpoint 'auth-err-pass' basic_auth password")
        );
    }

    #[test]
    #[serial]
    fn test_expand_headers_all_scenarios() {
        unsafe {
            std::env::set_var("STATUI_TEST_API_KEY", "key_12345");
            std::env::set_var("STATUI_TEST_TENANT", "acme-corp");
        }

        let mut ep = mock_endpoint("headers-test", "https://example.com");
        ep.headers.insert(
            "Authorization".to_string(),
            "Bearer ${STATUI_TEST_API_KEY}".to_string(),
        );
        ep.headers
            .insert("X-Tenant".to_string(), "${STATUI_TEST_TENANT}".to_string());
        ep.headers.insert(
            "X-Region".to_string(),
            "${STATUI_UNSET_REGION_XYZ:-us-east-1}".to_string(),
        );
        ep.headers
            .insert("X-Escaped".to_string(), "foo$$bar".to_string());

        expand_headers(&mut ep).expect("Headers expansion should succeed");
        assert_eq!(
            ep.headers.get("Authorization").map(|s| s.as_str()),
            Some("Bearer key_12345")
        );
        assert_eq!(
            ep.headers.get("X-Tenant").map(|s| s.as_str()),
            Some("acme-corp")
        );
        assert_eq!(
            ep.headers.get("X-Region").map(|s| s.as_str()),
            Some("us-east-1")
        );
        assert_eq!(
            ep.headers.get("X-Escaped").map(|s| s.as_str()),
            Some("foo$bar")
        );

        // Missing header variable error
        let mut ep_err = mock_endpoint("headers-err", "https://example.com");
        ep_err.headers.insert(
            "X-Secret".to_string(),
            "$STATUI_UNSET_SECRET_HEADER".to_string(),
        );
        let err = expand_headers(&mut ep_err).unwrap_err();
        let err_msg = err.to_string();
        assert!(err_msg.contains("In endpoint 'headers-err' header 'X-Secret'"));
        assert!(err_msg.contains("STATUI_UNSET_SECRET_HEADER"));
    }

    #[test]
    #[serial]
    fn test_expand_body_variants_and_errors() {
        unsafe {
            std::env::set_var("STATUI_TEST_DIR", "/tmp/statui_test");
            std::env::set_var("STATUI_TEST_RAW_ID", "42");
        }

        // 1. BodyConfig::File
        let mut ep_file = mock_endpoint("file-ep", "https://example.com");
        ep_file.body = Some(BodyConfig::File(FileBody {
            file: PathBuf::from("${STATUI_TEST_DIR}/payload.json"),
        }));
        expand_body(&mut ep_file).expect("File body expansion should succeed");
        match &ep_file.body {
            Some(BodyConfig::File(fb)) => {
                assert_eq!(fb.file, PathBuf::from("/tmp/statui_test/payload.json"))
            }
            _ => panic!("Expected BodyConfig::File"),
        }

        // 2. BodyConfig::Raw (multiline + fallbacks + escaped dollars)
        let mut ep_raw = mock_endpoint("raw-ep", "https://example.com");
        ep_raw.body = Some(BodyConfig::Raw(
            "id=${STATUI_TEST_RAW_ID}\nmode=${STATUI_UNSET_MODE_XYZ:-fast}\nprice=$$10".to_string(),
        ));
        expand_body(&mut ep_raw).expect("Raw body expansion should succeed");
        match &ep_raw.body {
            Some(BodyConfig::Raw(s)) => assert_eq!(s, "id=42\nmode=fast\nprice=$10"),
            _ => panic!("Expected BodyConfig::Raw"),
        }

        // 3. BodyConfig::Json
        let mut ep_json = mock_endpoint("json-ep", "https://example.com");
        ep_json.body = Some(BodyConfig::Json(serde_json::json!({
            "target_id": "${STATUI_TEST_RAW_ID}",
            "currency": "$$USD"
        })));
        expand_body(&mut ep_json).expect("Json body expansion should succeed");
        match &ep_json.body {
            Some(BodyConfig::Json(v)) => {
                assert_eq!(v["target_id"], "42");
                assert_eq!(v["currency"], "$USD");
            }
            _ => panic!("Expected BodyConfig::Json"),
        }

        // 4. BodyConfig::None
        let mut ep_none = mock_endpoint("none-ep", "https://example.com");
        ep_none.body = None;
        expand_body(&mut ep_none).expect("None body expansion should succeed");
        assert!(ep_none.body.is_none());

        // 5. Error in BodyConfig::File
        let mut ep_file_err = mock_endpoint("file-err-ep", "https://example.com");
        ep_file_err.body = Some(BodyConfig::File(FileBody {
            file: PathBuf::from("$STATUI_UNSET_FILE_DIR_XYZ/data.json"),
        }));
        let err = expand_body(&mut ep_file_err).unwrap_err();
        assert!(
            err.to_string()
                .contains("In endpoint 'file-err-ep' body file")
        );

        // 6. Error in BodyConfig::Raw
        let mut ep_raw_err = mock_endpoint("raw-err-ep", "https://example.com");
        ep_raw_err.body = Some(BodyConfig::Raw("$STATUI_UNSET_RAW_VAR_XYZ".to_string()));
        let err = expand_body(&mut ep_raw_err).unwrap_err();
        assert!(err.to_string().contains("In endpoint 'raw-err-ep' body"));

        // 7. Error in BodyConfig::Json
        let mut ep_json_err = mock_endpoint("json-err-ep", "https://example.com");
        ep_json_err.body = Some(BodyConfig::Json(serde_json::json!({
            "key": "$STATUI_UNSET_JSON_VAR_XYZ"
        })));
        let err = expand_body(&mut ep_json_err).unwrap_err();
        assert!(
            err.to_string()
                .contains("In endpoint 'json-err-ep' JSON body")
        );
    }
}
