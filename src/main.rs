mod blockchain;
mod transaction;
mod block;
mod network;
mod mining;
mod api;
mod address;

use blockchain::Blockchain;
use api::start_api_server;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    println!("🪙 TenCoin (TEC) - Digital Currency 🪙");
    println!("=======================================");
    println!("💎 Total Supply: 10,000,000 TEC");
    println!("🏆 Initial Reward: 50 TEC");
    println!("📉 Halving: Every 100,000 blocks");
    println!("⚙️ Difficulty: 5 zeros");
    println!("=======================================");
    
    // Create new blockchain
    let mut blockchain = Blockchain::new();
    
    // Add Genesis block
    blockchain.create_genesis_block();
    
    // Convert to Arc<Mutex> for sharing between threads
    let blockchain = Arc::new(Mutex::new(blockchain));
    
    // Start API server
    let blockchain_clone = blockchain.clone();
    let api_handle = tokio::spawn(async move {
        start_api_server(blockchain_clone).await;
    });
    
    println!("🌐 API Server running at: http://0.0.0.0:8333");
    println!("📡 Network Node running at: 0.0.0.0:8334");
    println!();
    println!("🔗 Sample API requests:");
    println!("   GET  /blockchain/info - Blockchain information");
    println!("   POST /wallet/generate - Generate new wallet");
    println!("   POST /transaction/send - Send transaction");
    println!("   GET  /mining/stats - Mining statistics");
    println!();
    println!("🛠️ To run miner, execute miner.py:");
    println!("   python3 miner.py");
    println!();
    
    // Wait for API server
    let _ = api_handle.await;
}
