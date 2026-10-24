#[path = "../src/config.rs"]
pub mod config;
#[path = "../src/backend.rs"]
pub mod backend;

use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;
use wiremock::matchers::{body_json, body_string, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use backend::{check_endpoint, run_backend, CheckResult, CheckStatus, ResolvedRequestBody};
use config::{BodyConfig, Endpoint, StatuiConfig};

#[tokio::test]
async fn test_get_request() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let endpoint = Endpoint {
        name: "Health Check".to_string(),
        url: format!("{}/health", mock_server.uri()),
        method: Some("GET".to_string()),
        interval: None,
        timeout: Some(5),
        skip_cert_verification: None,
        headers: HashMap::new(),
        body: None,
    };

    let result = check_endpoint(&endpoint, Duration::from_secs(5), &None, &client).await;

    assert_eq!(result.name, "Health Check");
    match result.status {
        CheckStatus::Success { code, text } => {
            assert_eq!(code, 200);
            assert_eq!(text, "OK");
        }
        CheckStatus::Error { message } => panic!("Unexpected error: {message}"),
    }
}

#[tokio::test]
async fn test_post_json_body_and_headers() {
    let mock_server = MockServer::start().await;

    let json_payload = serde_json::json!({
        "service": "statui",
        "ping": true,
        "count": 42
    });

    Mock::given(method("POST"))
        .and(path("/api/ping"))
        .and(header("Authorization", "Bearer mock-token-123"))
        .and(header("content-type", "application/json"))
        .and(body_json(json_payload.clone()))
        .respond_with(ResponseTemplate::new(201))
        .mount(&mock_server)
        .await;

    let mut headers = HashMap::new();
    headers.insert(
        "Authorization".to_string(),
        "Bearer mock-token-123".to_string(),
    );

    let client = reqwest::Client::new();
    let endpoint = Endpoint {
        name: "POST Test".to_string(),
        url: format!("{}/api/ping", mock_server.uri()),
        method: Some("POST".to_string()),
        interval: None,
        timeout: Some(5),
        skip_cert_verification: None,
        headers,
        body: Some(BodyConfig::Json(json_payload.clone())),
    };

    let body_bytes = serde_json::to_vec(&json_payload).unwrap();
    let resolved = Some(ResolvedRequestBody::Json(body_bytes));

    let result = check_endpoint(&endpoint, Duration::from_secs(5), &resolved, &client).await;

    match result.status {
        CheckStatus::Success { code, .. } => assert_eq!(code, 201),
        CheckStatus::Error { message } => panic!("Unexpected error: {message}"),
    }
}

#[tokio::test]
async fn test_put_raw_body() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/sensor/update"))
        .and(body_string("sensor_id=42&status=healthy"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let endpoint = Endpoint {
        name: "PUT Test".to_string(),
        url: format!("{}/sensor/update", mock_server.uri()),
        method: Some("PUT".to_string()),
        interval: None,
        timeout: Some(5),
        skip_cert_verification: None,
        headers: HashMap::new(),
        body: Some(BodyConfig::Raw("sensor_id=42&status=healthy".to_string())),
    };

    let resolved = Some(ResolvedRequestBody::Raw(
        "sensor_id=42&status=healthy".as_bytes().to_vec(),
    ));

    let result = check_endpoint(&endpoint, Duration::from_secs(5), &resolved, &client).await;

    match result.status {
        CheckStatus::Success { code, .. } => assert_eq!(code, 200),
        CheckStatus::Error { message } => panic!("Unexpected error: {message}"),
    }
}

#[tokio::test]
async fn test_method_case_insensitivity() {
    let mock_server = MockServer::start().await;

    // Server expects standard uppercase "DELETE"
    Mock::given(method("DELETE"))
        .and(path("/resource/99"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    // Endpoint config uses lowercase "delete"
    let endpoint = Endpoint {
        name: "Delete Test".to_string(),
        url: format!("{}/resource/99", mock_server.uri()),
        method: Some("delete".to_string()),
        interval: None,
        timeout: Some(5),
        skip_cert_verification: None,
        headers: HashMap::new(),
        body: None,
    };

    let result = check_endpoint(&endpoint, Duration::from_secs(5), &None, &client).await;

    match result.status {
        CheckStatus::Success { code, .. } => assert_eq!(code, 204),
        CheckStatus::Error { message } => panic!("Unexpected error: {message}"),
    }
}

#[tokio::test]
async fn test_full_pipeline_with_run_backend() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/pipeline-check"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let config = StatuiConfig {
        default_interval: 1,
        default_timeout: 5,
        endpoints: vec![Endpoint {
            name: "Worker Pipeline".to_string(),
            url: format!("{}/pipeline-check", mock_server.uri()),
            method: Some("GET".to_string()),
            interval: Some(1),
            timeout: Some(5),
            skip_cert_verification: None,
            headers: HashMap::new(),
            body: None,
        }],
    };

    let (tx, mut rx) = mpsc::channel::<CheckResult>(10);

    tokio::spawn(async move {
        run_backend(config, tx).await;
    });

    // Worker performs a warmup check first, then enters the loop.
    // Receive the check result from the channel:
    let received = tokio::time::timeout(Duration::from_secs(5), rx.recv()).await;

    let check_result = received
        .expect("Timed out waiting for check result")
        .expect("Channel closed prematurely");

    assert_eq!(check_result.name, "Worker Pipeline");
    match check_result.status {
        CheckStatus::Success { code, .. } => assert_eq!(code, 200),
        CheckStatus::Error { message } => panic!("Unexpected error: {message}"),
    }
}
