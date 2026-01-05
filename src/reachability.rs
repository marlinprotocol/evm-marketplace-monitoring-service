use anyhow::{Result, anyhow};
use log::{error, info};
use std::time::Duration as StdDuration;
use tokio::net::TcpStream;

pub async fn wait_for_ip_address(url: &str, job_id: String, region: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let mut last_response = String::new();
    const IP_CHECK_RETRIES: u32 = 20;
    const IP_CHECK_INTERVAL: u64 = 15; // seconds

    // Construct the IP endpoint URL with query parameters
    let ip_url = format!("{}/ip?id={}&region={}", url, job_id, region);

    for attempt in 1..=IP_CHECK_RETRIES {
        info!(
            "Checking for IP address (attempt {}/{})",
            attempt, IP_CHECK_RETRIES
        );

        let resp = client.get(&ip_url).send().await;
        let Ok(response) = resp else {
            error!("Failed to connect to IP endpoint: {}", resp.unwrap_err());
            tokio::time::sleep(StdDuration::from_secs(IP_CHECK_INTERVAL)).await;
            continue;
        };

        // Get the status code
        let status = response.status();

        // Get text response first to log in case of error
        let text = response.text().await;
        let Ok(text_body) = text else {
            error!("Failed to read response body: {}", text.unwrap_err());
            tokio::time::sleep(StdDuration::from_secs(IP_CHECK_INTERVAL)).await;
            continue;
        };

        // Parse the JSON
        let json_result = serde_json::from_str::<serde_json::Value>(&text_body);
        let Ok(json) = json_result else {
            let err = json_result.unwrap_err();
            error!(
                "Failed to parse IP endpoint response (status: {}): {}. Raw response: {}",
                status, err, text_body
            );
            tokio::time::sleep(StdDuration::from_secs(IP_CHECK_INTERVAL)).await;
            continue;
        };

        last_response = json.to_string();

        info!("Response from IP endpoint: {}", last_response);

        // Check for IP in response
        if let Some(ip) = json.get("ip").and_then(|ip| ip.as_str())
            && !ip.is_empty()
        {
            return Ok(ip.to_string());
        }

        info!("IP not found yet, waiting {} seconds...", IP_CHECK_INTERVAL);
        tokio::time::sleep(StdDuration::from_secs(IP_CHECK_INTERVAL)).await;
    }

    Err(anyhow!(
        "IP address not found after {} attempts. Last response: {}",
        IP_CHECK_RETRIES,
        last_response
    ))
}

pub async fn ping_ip(ip: &str) -> bool {
    const TCP_CHECK_RETRIES: u32 = 10;
    const TCP_CHECK_INTERVAL: u64 = 15;
    let address = format!("{}:1300", ip);
    for attempt in 1..=TCP_CHECK_RETRIES {
        info!(
            "Attempting TCP connection to {} (attempt {}/{})",
            address, attempt, TCP_CHECK_RETRIES
        );
        match tokio::time::timeout(StdDuration::from_secs(2), TcpStream::connect(&address)).await {
            Ok(Ok(_)) => {
                return true;
            }
            Ok(Err(e)) => info!("TCP connection failed: {}", e),
            Err(_) => info!("TCP connection timed out"),
        }
        tokio::time::sleep(StdDuration::from_secs(TCP_CHECK_INTERVAL)).await;
    }
    info!("All TCP connection attempts failed");
    false
}

pub async fn check_reachability(ip: &str) -> bool {
    const ATTESTATION_RETRIES: u32 = 20;
    const ATTESTATION_INTERVAL: u64 = 15;
    // First check basic connectivity
    if !ping_ip(ip).await {
        error!("Failed to establish TCP connection to the instance");
        return false;
    }

    let client = reqwest::Client::new();
    let attestation_url = format!("http://{}:1300/attestation/raw", ip);

    for attempt in 1..=ATTESTATION_RETRIES {
        info!(
            "Checking reachability (attempt {}/{})",
            attempt, ATTESTATION_RETRIES
        );

        match client.get(&attestation_url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.bytes().await {
                        Ok(bytes) if !bytes.is_empty() => {
                            info!("Reachability check successful");
                            return true;
                        }
                        Ok(_) => info!("Empty attestation response"),
                        Err(e) => info!("Error reading attestation response: {}", e),
                    }
                }
            }
            Err(e) => info!("Failed to connect to attestation endpoint: {}", e),
        }

        info!(
            "Waiting {} seconds before next reachability check...",
            ATTESTATION_INTERVAL
        );
        tokio::time::sleep(StdDuration::from_secs(ATTESTATION_INTERVAL)).await;
    }

    false
}
