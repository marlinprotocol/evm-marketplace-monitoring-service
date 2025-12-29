mod reachability;
mod types;

use ethers::contract::abigen;
use ethers::prelude::*;
use ethers::providers::{Provider, Ws};
use log::{error, info};
use std::sync::Arc;
use std::time::Duration as StdDuration;

use reachability::check_reachability;
use types::Metadata;

use crate::reachability::wait_for_ip_address;

abigen!(MarketV1, "src/abis/oyster_market_abi.json");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get WebSocket RPC URL from environment
    let ws_url = std::env::var("WS_URL").expect("WS_URL must be set in .env file");

    // Connect to websocket
    let ws = Ws::connect(&ws_url).await?;
    let provider = Provider::new(ws);
    let provider = Arc::new(provider);

    // Get contract address from environment
    let contract_address_str =
        std::env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS must be set in .env file");
    let contract_addr: Address = contract_address_str.parse()?;
    let contract = MarketV1::new(contract_addr, provider.clone());

    // Subscribe to JobOpened events from a specific block
    let binding = contract.event::<JobOpenedFilter>();
    let mut stream = binding.stream().await?;
    info!("Listening for JobOpened events...");
    while let Some(Ok(event)) = stream.next().await {
        info!("JobOpened event found");
        let metadata_str = event.metadata;
        let owner = event.owner;
        let job = "0x".to_string() + &hex::encode(event.job);
        let operator = event.provider;
        let cp_url = contract.providers(operator).call().await?;

        // Parse metadata JSON into struct
        let metadata: Metadata = match serde_json::from_str(&metadata_str) {
            Ok(m) => m,
            Err(e) => {
                error!(
                    "Failed to parse metadata JSON: {} | raw: {}",
                    e, metadata_str
                );
                continue;
            }
        };

        tokio::spawn(async move {
            info!("Handling JobOpened event:");
            info!("job: {:?}", job);
            info!("metadata: {:?}", metadata);
            info!("owner: {:?}", owner);
            info!("operator: {:?}", operator);
            info!("cp_url: {:?}", cp_url);
            if let Some(instance) = &metadata.instance {
                info!("instance: {}", instance);
            }

            info!("Waiting for 3 minutes for enclave to start...");
            tokio::time::sleep(StdDuration::from_secs(180)).await;

            let instance_ip = match wait_for_ip_address(
                &cp_url,
                job.clone(),
                metadata.region.as_deref().unwrap_or(""),
            )
            .await
            {
                Ok(ip) => ip,
                Err(e) => {
                    error!("Failed to get IP address: {}", e);
                    return;
                }
            };

            info!("instance IP: {}", instance_ip);

            if check_reachability(&instance_ip).await {
                info!("Instance is reachable");
            } else {
                error!("Instance reachability test failed");
            }
        });
    }
    Ok(())
}
