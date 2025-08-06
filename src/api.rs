use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::address::{AddressGenerator, AddressType, Wallet, WalletManager};

#[derive(Debug, Deserialize)]
pub struct WalletGenerateRequest {
    #[serde(rename = "type")]
    pub wallet_type: String,
    pub required: Option<usize>,
    pub total: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct WalletGenerateResponse {
    pub success: bool,
    pub wallet: Option<Wallet>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionRequest {
    pub from_address: String,
    pub to_address: String,
    pub amount: f64,
    pub fee: Option<f64>,
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionInput {
    pub txid: String,
    pub vout: u32,
    pub script_sig: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionOutput {
    pub address: String,
    pub amount: f64,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub success: bool,
    pub txid: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BlockchainInfoResponse {
    pub height: usize,
    pub total_supply: f64,
    pub max_supply: f64,
    pub current_reward: f64,
    pub difficulty: usize,
    pub pending_transactions: usize,
    pub next_halving: u64,
    pub halving_countdown: u64,
}

#[derive(Debug, Serialize)]
pub struct MiningStatsResponse {
    pub current_reward: f64,
    pub next_halving_block: u64,
    pub blocks_until_halving: u64,
    pub total_mined: f64,
    pub remaining_supply: f64,
    pub difficulty: usize,
}

#[derive(Debug, Deserialize)]
pub struct BalanceRequest {
    pub address: String,
}

#[derive(Debug, Serialize)]
pub struct BalanceResponse {
    pub address: String,
    pub balance: f64,
    pub confirmed: f64,
    pub unconfirmed: f64,
}

pub async fn start_api_server(blockchain: Arc<Mutex<Blockchain>>) {
    let wallet_manager = Arc::new(Mutex::new(WalletManager::new()));

    // CORS headers
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]);

    // Clone برای استفاده در routes
    let blockchain_filter = warp::any().map(move || blockchain.clone());
    let wallet_filter = warp::any().map(move || wallet_manager.clone());

    // Route: GET /blockchain/info
    let blockchain_info = warp::path!("blockchain" / "info")
        .and(warp::get())
        .and(blockchain_filter.clone())
        .and_then(get_blockchain_info);

    // Route: POST /wallet/generate
    let wallet_generate = warp::path!("wallet" / "generate")
        .and(warp::post())
        .and(warp::body::json())
        .and(wallet_filter.clone())
        .and_then(generate_wallet);

    // Route: GET /wallet/generate (برای query parameters)
    let wallet_generate_query = warp::path!("wallet" / "generate")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and(wallet_filter.clone())
        .and_then(generate_wallet_query);

    // Route: POST /transaction/send
    let transaction_send = warp::path!("transaction" / "send")
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and(wallet_filter.clone())
        .and_then(send_transaction);

    // Route: GET /balance/{address}
    let balance = warp::path!("balance" / String)
        .and(warp::get())
        .and(blockchain_filter.clone())
        .and_then(get_balance);

    // Route: GET /mining/stats
    let mining_stats = warp::path!("mining" / "stats")
        .and(warp::get())
        .and(blockchain_filter.clone())
        .and_then(get_mining_stats);

    // Route: GET / (صفحه اصلی)
    let index = warp::path::end()
        .and(warp::get())
        .map(|| {
            warp::reply::html(
                r#"
                <html>
                <head><title>TenCoin API</title></head>
                <body>
                    <h1>🪙 TenCoin (TEC) API Server</h1>
                    <h2>Available Endpoints:</h2>
                    <ul>
                        <li><strong>GET /blockchain/info</strong> - اطلاعات بلاک چین</li>
                        <li><strong>POST /wallet/generate</strong> - ایجاد کیف پول</li>
                        <li><strong>GET /wallet/generate?type=bech32</strong> - ایجاد کیف پول با query</li>
                        <li><strong>POST /transaction/send</strong> - ارسال تراکنش</li>
                        <li><strong>GET /balance/{address}</strong> - موجودی آدرس</li>
                        <li><strong>GET /mining/stats</strong> - آمار استخراج</li>
                    </ul>
                    <h3>نمونه درخواست‌ها:</h3>
                    <pre>
# ایجاد کیف پول bech32
curl "http://localhost:8333/wallet/generate?type=bech32"

# ایجاد کیف پول multisig
curl "http://localhost:8333/wallet/generate?type=p2sh&required=2&total=4"

# اطلاعات بلاک چین
curl "http://localhost:8333/blockchain/info"
                    </pre>
                </body>
                </html>
                "#
            )
        });

    let routes = index
        .or(blockchain_info)
        .or(wallet_generate)
        .or(wallet_generate_query)
        .or(transaction_send)
        .or(balance)
        .or(mining_stats)
        .with(cors);

    println!("🌐 API Server شروع شد در: http://0.0.0.0:8333");
    warp::serve(routes).run(([0, 0, 0, 0], 8333)).await;
}

async fn get_blockchain_info(blockchain: Arc<Mutex<Blockchain>>) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.lock().await;
    let height = blockchain.get_chain_length();
    let total_supply = blockchain.get_total_supply();
    let current_reward = blockchain.calculate_mining_reward();
    let halving_interval = 100_000u64;
    let next_halving = ((height as u64 / halving_interval) + 1) * halving_interval;
    let halving_countdown = next_halving.saturating_sub(height as u64);

    let response = BlockchainInfoResponse {
        height,
        total_supply,
        max_supply: 10_000_000.0,
        current_reward,
        difficulty: blockchain.difficulty,
        pending_transactions: blockchain.get_pending_transactions_count(),
        next_halving,
        halving_countdown,
    };

    Ok(warp::reply::json(&response))
}

async fn generate_wallet(
    request: WalletGenerateRequest,
    wallet_manager: Arc<Mutex<WalletManager>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut manager = wallet_manager.lock().await;

    let address_type = match request.wallet_type.as_str() {
        "bech32" => AddressType::Bech32,
        "p2sh" => {
            let required = request.required.unwrap_or(2);
            let total = request.total.unwrap_or(3);
            AddressType::P2SH { required, total }
        }
        _ => {
            let response = WalletGenerateResponse {
                success: false,
                wallet: None,
                error: Some("نوع کیف پول نامعتبر است".to_string()),
            };
            return Ok(warp::reply::json(&response));
        }
    };

    match manager.create_wallet(address_type) {
        Ok(wallet) => {
            let response = WalletGenerateResponse {
                success: true,
                wallet: Some(wallet),
                error: None,
            };
            Ok(warp::reply::json(&response))
        }
        Err(error) => {
            let response = WalletGenerateResponse {
                success: false,
                wallet: None,
                error: Some(error),
            };
            Ok(warp::reply::json(&response))
        }
    }
}

async fn generate_wallet_query(
    params: HashMap<String, String>,
    wallet_manager: Arc<Mutex<WalletManager>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let wallet_type = params.get("type").unwrap_or(&"bech32".to_string()).clone();
    let required = params.get("required").and_then(|s| s.parse().ok()).unwrap_or(2);
    let total = params.get("total").and_then(|s| s.parse().ok()).unwrap_or(3);

    let request = WalletGenerateRequest {
        wallet_type,
        required: Some(required),
        total: Some(total),
    };

    generate_wallet(request, wallet_manager).await
}

async fn send_transaction(
    request: TransactionRequest,
    blockchain: Arc<Mutex<Blockchain>>,
    wallet_manager: Arc<Mutex<WalletManager>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    let wallet_manager = wallet_manager.lock().await;

    // بررسی آدرس فرستنده
    if let Some(wallet) = wallet_manager.get_wallet(&request.from_address) {
        // ایجاد تراکنش (ساده‌سازی شده)
        match Transaction::new_with_fee(
            request.from_address,
            request.to_address,
            request.amount,
            request.fee.unwrap_or(0.001),
            wallet,
        ) {
            Ok(transaction) => {
                let txid = transaction.id.clone();
                match blockchain.add_transaction(transaction) {
                    Ok(_) => {
                        let response = TransactionResponse {
                            success: true,
                            txid: Some(txid),
                            error: None,
                        };
                        Ok(warp::reply::json(&response))
                    }
                    Err(error) => {
                        let response = TransactionResponse {
                            success: false,
                            txid: None,
                            error: Some(error),
                        };
                        Ok(warp::reply::json(&response))
                    }
                }
            }
            Err(error) => {
                let response = TransactionResponse {
                    success: false,
                    txid: None,
                    error: Some(error),
                };
                Ok(warp::reply::json(&response))
            }
        }
    } else {
        let response = TransactionResponse {
            success: false,
            txid: None,
            error: Some("کیف پول فرستنده یافت نشد".to_string()),
        };
        Ok(warp::reply::json(&response))
    }
}

async fn get_balance(
    address: String,
    blockchain: Arc<Mutex<Blockchain>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.lock().await;
    let balance = blockchain.get_balance(&address);

    let response = BalanceResponse {
        address: address.clone(),
        balance,
        confirmed: balance,
        unconfirmed: 0.0, // برای سادگی
    };

    Ok(warp::reply::json(&response))
}

async fn get_mining_stats(blockchain: Arc<Mutex<Blockchain>>) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.lock().await;
    let height = blockchain.get_chain_length() as u64;
    let halving_interval = 100_000u64;
    let next_halving_block = ((height / halving_interval) + 1) * halving_interval;
    let blocks_until_halving = next_halving_block.saturating_sub(height);
    let total_mined = blockchain.get_total_supply();
    let remaining_supply = 10_000_000.0 - total_mined;

    let response = MiningStatsResponse {
        current_reward: blockchain.calculate_mining_reward(),
        next_halving_block,
        blocks_until_halving,
        total_mined,
        remaining_supply,
        difficulty: blockchain.difficulty,
    };

    Ok(warp::reply::json(&response))
}