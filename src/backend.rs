use crate::config::{Endpoint, StatuiConfig};
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

    // Throw-away request for the client to warm up
    let _ = check_endpoint(&endpoint, timeout, &client).await;

    sleep(Duration::from_millis(50)).await;

    loop {
        // Perform the actual HTTP check
        let result = check_endpoint(&endpoint, timeout, &client).await;

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

async fn check_endpoint(
    endpoint: &Endpoint,
    timeout: Duration,
    client: &reqwest::Client,
) -> CheckResult {
    let method_str = endpoint.method.as_deref().unwrap_or("GET");
    let method = reqwest::Method::from_bytes(method_str.as_bytes()).unwrap_or(reqwest::Method::GET);

    let start_time = std::time::Instant::now();

    let mut request = client.request(method, &endpoint.url).timeout(timeout);

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
