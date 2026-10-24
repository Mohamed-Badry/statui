use crate::config::{BasicAuth, BodyConfig, Endpoint, FileBody, StatuiConfig};
use std::time::Duration;
use tokio::{sync::mpsc::Sender, time::sleep};

// This builds the User-Agent string at compile time
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// The status of an HTTP check,
///
/// Either a Success with code and message (e.g. 200 OK)
/// or Error.
#[derive(Debug, Clone)]
pub enum CheckStatus {
    Success {
        code: u16,
        text: String,
    },
    Error {
        // (e.g. "Timeout", "DNS Error")
        message: String,
    },
}

#[derive(Debug, Clone)]
pub enum ResolvedRequestBody {
    Json(Vec<u8>),
    Raw(Vec<u8>),
}

/// The result of an HTTP check on an endpoint.
#[derive(Debug, Clone)]
pub struct CheckResult {
    pub name: String,
    pub status: CheckStatus,
    pub latency: Duration,
}

/// Backend entry point that spawns N async tasks with endpoint_worker
/// using tokio where N is the number of Endpoints in StatuiConfig.
pub async fn run_backend(config: StatuiConfig, tx: Sender<CheckResult>) {
    for endpoint in config.endpoints {
        let tx_clone = tx.clone();
        let skip_cert_verification = endpoint.skip_cert_verification.unwrap_or(false);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .danger_accept_invalid_certs(skip_cert_verification)
            .build()
            .expect("Failed to build reqwest client");
        let client_clone = client.clone();

        tokio::spawn(endpoint_worker(
            endpoint,
            config.default_interval,
            config.default_timeout,
            client_clone,
            tx_clone,
        ));
    }
}

async fn endpoint_worker(
    endpoint: Endpoint,
    default_interval: u64,
    default_timeout: u64,
    client: reqwest::Client,
    tx: Sender<CheckResult>,
) {
    let interval = endpoint.interval.unwrap_or(default_interval);
    let interval = Duration::from_secs(interval);

    let timeout = endpoint.timeout.unwrap_or(default_timeout);
    let timeout = Duration::from_secs(timeout);

    let resolved_body: Result<Option<ResolvedRequestBody>, String> = match &endpoint.body {
        Some(BodyConfig::File(FileBody { file })) => std::fs::read(file)
            .map(|bytes| {
                if file.extension().is_some_and(|ext| ext == "json") {
                    Some(ResolvedRequestBody::Json(bytes))
                } else {
                    Some(ResolvedRequestBody::Raw(bytes))
                }
            })
            .map_err(|e| format!("Failed to read from file '{}': {e}", file.display())),
        Some(BodyConfig::Json(value)) => serde_json::to_vec(value)
            .map(|bytes| Some(ResolvedRequestBody::Json(bytes)))
            .map_err(|e| format!("Failed to serialize JSON body: {e}")),
        Some(BodyConfig::Raw(text)) => Ok(Some(ResolvedRequestBody::Raw(text.as_bytes().to_vec()))),
        None => Ok(None),
    };

    let initial_error = resolved_body.as_ref().err().cloned();
    let body = resolved_body.unwrap_or(None);

    // Throw-away request for the client to warm up
    let _ = check_endpoint(&endpoint, timeout, &body, &client).await;

    sleep(Duration::from_millis(50)).await;

    loop {
        // Perform the actual HTTP check
        let result = if let Some(err_msg) = &initial_error {
            CheckResult {
                name: endpoint.name.clone(),
                status: CheckStatus::Error {
                    message: err_msg.to_string(),
                },
                latency: Duration::ZERO,
            }
        } else {
            check_endpoint(&endpoint, timeout, &body, &client).await
        };

        // Send the result to the TUI.
        // If tx.send fails, the TUI has probably quit.
        // We can safely shut down this worker task by breaking the loop.
        if tx.send(result).await.is_err() {
            break;
        }

        // Sleep until the next check
        sleep(interval).await;
    }
}

pub async fn check_endpoint(
    endpoint: &Endpoint,
    timeout: Duration,
    body: &Option<ResolvedRequestBody>,
    client: &reqwest::Client,
) -> CheckResult {
    let method_str = endpoint.method.as_deref().unwrap_or("GET").to_uppercase();
    let method = reqwest::Method::from_bytes(method_str.as_bytes()).unwrap_or(reqwest::Method::GET);

    let start_time = std::time::Instant::now();

    let mut request = client.request(method, &endpoint.url).timeout(timeout);

    if let Some(BasicAuth { username, password }) = &endpoint.basic_auth {
        request = request.basic_auth(username, password.as_deref());
    };

    if let Some(body_data) = body {
        match body_data {
            ResolvedRequestBody::Json(bytes) => {
                request = request
                    .body(bytes.clone())
                    .header(reqwest::header::CONTENT_TYPE, "application/json")
            }
            ResolvedRequestBody::Raw(bytes) => request = request.body(bytes.clone()),
        }
    };

    for (key, value) in &endpoint.headers {
        request = request.header(key, value);
    }

    let (status, latency) = match request.send().await {
        Ok(response) => {
            let latency = start_time.elapsed();
            let status = CheckStatus::Success {
                code: response.status().as_u16(),
                text: response
                    .status()
                    .canonical_reason()
                    .unwrap_or("Unknown")
                    .to_string(),
            };
            (status, latency)
        }
        Err(e) => {
            let latency = start_time.elapsed();
            let status = CheckStatus::Error {
                message: if e.is_timeout() {
                    String::from("Timeout")
                } else if e.is_connect() {
                    String::from("Connection Error")
                } else {
                    String::from("Error")
                },
            };
            (status, latency)
        }
    };

    CheckResult {
        name: endpoint.name.clone(),
        status,
        latency,
    }
}
