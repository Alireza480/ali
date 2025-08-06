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
    println!("🪙 TenCoin (TEC) - ارز دیجیتال ایرانی 🪙");
    println!("=========================================");
    println!("💎 کل عرضه: 10,000,000 TEC");
    println!("🏆 پاداش اولیه: 50 TEC");
    println!("📉 هاوینگ: هر 100,000 بلاک");
    println!("⚙️ سختی: 5 صفر");
    println!("=========================================");
    
    // ایجاد بلاک چین جدید
    let mut blockchain = Blockchain::new();
    
    // اضافه کردن بلاک Genesis
    blockchain.create_genesis_block();
    
    // تبدیل به Arc<Mutex> برای اشتراک‌گذاری بین thread ها
    let blockchain = Arc::new(Mutex::new(blockchain));
    
    // شروع API server
    let blockchain_clone = blockchain.clone();
    let api_handle = tokio::spawn(async move {
        start_api_server(blockchain_clone).await;
    });
    
    println!("🌐 API Server در حال اجرا در: http://0.0.0.0:8333");
    println!("📡 Network Node در حال اجرا در: 0.0.0.0:8334");
    println!();
    println!("🔗 نمونه درخواست‌های API:");
    println!("   GET  /blockchain/info - اطلاعات بلاک چین");
    println!("   POST /wallet/generate - ایجاد کیف پول جدید");
    println!("   POST /transaction/send - ارسال تراکنش");
    println!("   GET  /mining/stats - آمار استخراج");
    println!();
    println!("🛠️ برای ماینر، فایل miner.py را اجرا کنید:");
    println!("   python3 miner.py");
    println!();
    
    // انتظار برای API server
    let _ = api_handle.await;
}
