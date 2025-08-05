mod blockchain;
mod transaction;
mod block;
mod wallet;
mod network;
mod mining;
mod cli;

use blockchain::Blockchain;
use transaction::Transaction;
use wallet::Wallet;
use cli::CLI;

#[tokio::main]
async fn main() {
    println!("🚀 RustCoin - ارز دیجیتال ایرانی 🚀");
    println!("===============================");
    
    // ایجاد بلاک چین جدید
    let mut blockchain = Blockchain::new();
    
    // ایجاد کیف پول‌ها
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();
    
    println!("💼 کیف پول 1: {}", wallet1.get_address());
    println!("💼 کیف پول 2: {}", wallet2.get_address());
    
    // اضافه کردن بلاک Genesis
    blockchain.create_genesis_block();
    
    // ایجاد تراکنش اولیه برای کیف پول 1 (از سیستم)
    let initial_transaction = Transaction::mining_reward(wallet1.get_address(), 100.0);
    if let Err(e) = blockchain.add_transaction(initial_transaction) {
        println!("❌ خطا در اضافه کردن تراکنش اولیه: {}", e);
        return;
    }
    
    // استخراج بلاک برای تراکنش اولیه
    if let Err(e) = blockchain.mine_pending_transactions(&wallet1.get_address()).await {
        println!("❌ خطا در استخراج بلاک اولیه: {}", e);
        return;
    }
    
    println!("💰 موجودی اولیه کیف پول 1: {}", blockchain.get_balance(&wallet1.get_address()));
    
    // ایجاد تراکنش بین کیف پول‌ها
    let transaction = Transaction::new(
        wallet1.get_address(),
        wallet2.get_address(),
        50.0,
        &wallet1
    );
    
    if let Ok(tx) = transaction {
        if let Err(e) = blockchain.add_transaction(tx) {
            println!("❌ خطا در اضافه کردن تراکنش: {}", e);
            return;
        }
        
        // استخراج بلاک جدید
        if let Err(e) = blockchain.mine_pending_transactions(&wallet1.get_address()).await {
            println!("❌ خطا در استخراج: {}", e);
            return;
        }
        
        println!("⛏️  بلاک جدید استخراج شد!");
        println!("📊 تعداد بلاک‌ها: {}", blockchain.get_chain_length());
        println!("💰 موجودی کیف پول 1: {}", blockchain.get_balance(&wallet1.get_address()));
        println!("💰 موجودی کیف پول 2: {}", blockchain.get_balance(&wallet2.get_address()));
    }
    
    println!("\n🖥️  شروع رابط خط فرمان...");
    
    // شروع CLI
    let mut cli = CLI::new(blockchain, vec![wallet1, wallet2]);
    cli.run().await;
}
