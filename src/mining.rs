use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

use crate::blockchain::Blockchain;
use crate::wallet::Wallet;

pub struct Miner {
    wallet: Wallet,
    blockchain: Arc<Mutex<Blockchain>>,
    is_mining: Arc<Mutex<bool>>,
}

impl Miner {
    pub fn new(wallet: Wallet, blockchain: Arc<Mutex<Blockchain>>) -> Self {
        Miner {
            wallet,
            blockchain,
            is_mining: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn start_mining(&self) -> Result<(), String> {
        {
            let mut is_mining = self.is_mining.lock().await;
            if *is_mining {
                return Err("استخراج در حال حاضر فعال است".to_string());
            }
            *is_mining = true;
        }

        println!("⛏️  شروع استخراج توسط: {}", self.wallet.get_address());

        loop {
            {
                let is_mining = self.is_mining.lock().await;
                if !*is_mining {
                    break;
                }
            }

            // بررسی وجود تراکنش‌های در انتظار
            let has_pending = {
                let blockchain = self.blockchain.lock().await;
                blockchain.get_pending_transactions_count() > 0
            };

            if has_pending {
                // استخراج بلاک
                match self.mine_block().await {
                    Ok(_) => {
                        println!("✅ بلاک با موفقیت استخراج شد!");
                        
                        // نمایش موجودی فعلی
                        let balance = {
                            let blockchain = self.blockchain.lock().await;
                            blockchain.get_balance(&self.wallet.get_address())
                        };
                        println!("💰 موجودی فعلی: {} RustCoin", balance);
                    }
                    Err(e) => {
                        println!("❌ خطا در استخراج: {}", e);
                    }
                }
            }

            // استراحت کوتاه قبل از بررسی مجدد
            sleep(Duration::from_secs(5)).await;
        }

        println!("⏹️  استخراج متوقف شد");
        Ok(())
    }

    pub async fn stop_mining(&self) {
        let mut is_mining = self.is_mining.lock().await;
        *is_mining = false;
        println!("🛑 درخواست توقف استخراج ارسال شد");
    }

    async fn mine_block(&self) -> Result<(), String> {
        let mut blockchain = self.blockchain.lock().await;
        blockchain.mine_pending_transactions(&self.wallet.get_address()).await
    }

    pub async fn is_mining(&self) -> bool {
        *self.is_mining.lock().await
    }

    pub fn get_miner_address(&self) -> String {
        self.wallet.get_address()
    }
}

pub struct MiningPool {
    miners: Vec<Arc<Miner>>,
    blockchain: Arc<Mutex<Blockchain>>,
}

impl MiningPool {
    pub fn new(blockchain: Arc<Mutex<Blockchain>>) -> Self {
        MiningPool {
            miners: Vec::new(),
            blockchain,
        }
    }

    pub fn add_miner(&mut self, wallet: Wallet) -> Arc<Miner> {
        let miner = Arc::new(Miner::new(wallet, self.blockchain.clone()));
        self.miners.push(miner.clone());
        println!("👷 استخراج‌کننده جدید اضافه شد: {}", miner.get_miner_address());
        miner
    }

    pub async fn start_all_miners(&self) {
        println!("🚀 شروع همه استخراج‌کننده‌ها...");
        
        let mut handles = Vec::new();
        
        for miner in &self.miners {
            let miner_clone = miner.clone();
            let handle = tokio::spawn(async move {
                if let Err(e) = miner_clone.start_mining().await {
                    println!("❌ خطا در استخراج‌کننده {}: {}", miner_clone.get_miner_address(), e);
                }
            });
            handles.push(handle);
        }

        // انتظار برای تمام استخراج‌کننده‌ها
        for handle in handles {
            let _ = handle.await;
        }
    }

    pub async fn stop_all_miners(&self) {
        println!("🛑 توقف همه استخراج‌کننده‌ها...");
        
        for miner in &self.miners {
            miner.stop_mining().await;
        }
    }

    pub fn get_miners_count(&self) -> usize {
        self.miners.len()
    }

    pub async fn get_mining_stats(&self) -> MiningStats {
        let mut active_miners = 0;
        
        for miner in &self.miners {
            if miner.is_mining().await {
                active_miners += 1;
            }
        }

        let blockchain = self.blockchain.lock().await;
        
        MiningStats {
            total_miners: self.miners.len(),
            active_miners,
            total_blocks: blockchain.get_chain_length(),
            pending_transactions: blockchain.get_pending_transactions_count(),
        }
    }
}

#[derive(Debug)]
pub struct MiningStats {
    pub total_miners: usize,
    pub active_miners: usize,
    pub total_blocks: usize,
    pub pending_transactions: usize,
}

impl std::fmt::Display for MiningStats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "📊 آمار استخراج")?;
        writeln!(f, "==================")?;
        writeln!(f, "👷 کل استخراج‌کننده‌ها: {}", self.total_miners)?;
        writeln!(f, "⚡ استخراج‌کننده‌های فعال: {}", self.active_miners)?;
        writeln!(f, "📦 کل بلاک‌ها: {}", self.total_blocks)?;
        writeln!(f, "⏳ تراکنش‌های در انتظار: {}", self.pending_transactions)?;
        Ok(())
    }
}