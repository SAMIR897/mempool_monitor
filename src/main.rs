use dotenv::dotenv;
use ethers::prelude::*;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    // Default to a public RPC if no environment variable is provided.
    // However, public websockets might be unstable or limited.
    let ws_url = env::var("WS_URL")
        .unwrap_or_else(|_| "wss://ethereum-rpc.publicnode.com".to_string());
        
    println!("Connecting to WebSocket: {} ...", ws_url);
    
    let provider = match Provider::<Ws>::connect(&ws_url).await {
        Ok(p) => p,
        Err(e) => {
            println!("Failed to connect to Websocket: {}", e);
            return Err(e.into());
        }
    };
    
    let client = Arc::new(provider);
    
    println!("Connected successfully! Monitoring pending transactions...");
    
    let mut stream = client.subscribe_pending_txs().await?;
    
    println!("Listening for transactions with value > 1.0 ETH...");
    
    // For every pending transaction hash
    while let Some(tx_hash) = stream.next().await {
        let client_clone = Arc::clone(&client);
        
        // Spawn a background task so we don't block the stream
        tokio::spawn(async move {
            // Attempt to get the full transaction details
            if let Ok(Some(tx)) = client_clone.get_transaction(tx_hash).await {
                // Check if the transaction value is greater than 1 ETH (10^18 wei)
                let one_eth = U256::from(1_000_000_000_000_000_000u64);
                
                if tx.value >= one_eth {
                    let value_eth = ethers::utils::format_units(tx.value, "ether").unwrap_or_else(|_| "0".to_string());
                    
                    let to_address = tx.to.map(|addr| format!("{:?}", addr)).unwrap_or_else(|| "Contract Creation".to_string());
                    
                    println!(
                        "🐋 Large Tx -> Hash: {:?} | Value: {} ETH | From: {:?} | To: {}", 
                        tx.hash, value_eth, tx.from, to_address
                    );
                }
            }
        });
    }
    
    Ok(())
}
