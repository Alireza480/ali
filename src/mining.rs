use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

use crate::blockchain::Blockchain;
// use crate::wallet::Wallet;

pub struct Miner {
    wallet: crate::address::Wallet,
    blockchain: Arc<Mutex<Blockchain>>,
    is_mining: Arc<Mutex<bool>>,
}

impl Miner {
    pub fn new(wallet: crate::address::Wallet, blockchain: Arc<Mutex<Blockchain>>) -> Self {
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
                return Err("mining در حال حاضر active است".to_string());
            }
            *is_mining = true;
        }

        println!("⛏️  Start mining توسط: {}", self.wallet.get_address());

        loop {
            {
                let is_mining = self.is_mining.lock().await;
                if !*is_mining {
                    break;
                }
            }

            // Check وجود transaction‌های pending
            let has_pending = {
                let blockchain = self.blockchain.lock().await;
                blockchain.get_pending_transactions_count() > 0
            };

            if has_pending {
                // Mining block
                match self.mine_block().await {
                    Ok(_) => {
                        println!("✅ block successfully mined!");
                        
                        // نمایش balance فعلی
                        let balance = {
                            let blockchain = self.blockchain.lock().await;
                            blockchain.get_balance(&self.wallet.get_address())
                        };
                        println!("💰 balance فعلی: {} RustCoin", balance);
                    }
                    Err(e) => {
                        println!("❌ Error in mining: {}", e);
                    }
                }
            }

            // استراحت کوتاه قبل از بررسی مجدد
            sleep(Duration::from_secs(5)).await;
        }

        println!("⏹️  mining مStop شد");
        Ok(())
    }

    pub async fn stop_mining(&self) {
        let mut is_mining = self.is_mining.lock().await;
        *is_mining = false;
        println!("🛑 درخواست Stop mining sent");
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

    pub fn add_miner(&mut self, wallet: crate::address::Wallet) -> Arc<Miner> {
        let miner = Arc::new(Miner::new(wallet, self.blockchain.clone()));
        self.miners.push(miner.clone());
        println!("👷 mining‌کننده جدید added: {}", miner.get_miner_address());
        miner
    }

    pub async fn start_all_miners(&self) {
        println!("🚀 Start همه mining‌کننده‌ها...");
        
        let mut handles = Vec::new();
        
        for miner in &self.miners {
            let miner_clone = miner.clone();
            let handle = tokio::spawn(async move {
                if let Err(e) = miner_clone.start_mining().await {
                    println!("❌ Error in mining‌کننده {}: {}", miner_clone.get_miner_address(), e);
                }
            });
            handles.push(handle);
        }

        // Wait برای تمام mining‌کننده‌ها
        for handle in handles {
            let _ = handle.await;
        }
    }

    pub async fn stop_all_miners(&self) {
        println!("🛑 Stop همه mining‌کننده‌ها...");
        
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
        writeln!(f, "📊 آمار mining")?;
        writeln!(f, "==================")?;
        writeln!(f, "👷 کل mining‌کننده‌ها: {}", self.total_miners)?;
        writeln!(f, "⚡ mining‌کننده‌های active: {}", self.active_miners)?;
        writeln!(f, "📦 کل block‌ها: {}", self.total_blocks)?;
        writeln!(f, "⏳ transaction‌های pending: {}", self.pending_transactions)?;
        Ok(())
    }
}