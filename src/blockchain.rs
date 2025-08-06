use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
// use tokio::time::{sleep, Duration};

use crate::block::Block;
use crate::transaction::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
    pending_transactions: Vec<Transaction>,
    mining_reward: f64,
    balances: HashMap<String, f64>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: Vec::new(),
            difficulty: 5, // سختی 5 صفر
            pending_transactions: Vec::new(),
            mining_reward: 50.0, // پاداش اولیه 50 TEC
            balances: HashMap::new(),
        }
    }

    pub fn create_genesis_block(&mut self) {
        if self.chain.is_empty() {
            println!("🌟 ایجاد بلاک Genesis...");
            let genesis_block = Block::genesis();
            
            // به‌روزرسانی موجودی‌ها از بلاک Genesis
            self.update_balances_from_block(&genesis_block);
            
            self.chain.push(genesis_block);
            println!("✅ بلاک Genesis ایجاد شد!");
            println!("💎 کل عرضه فعلی: {} TEC", self.get_total_supply());
        }
    }

    // محاسبه پاداش استخراج با در نظر گیری هاوینگ
    pub fn calculate_mining_reward(&self) -> f64 {
        let current_height = self.chain.len() as u64;
        let halving_interval = 100_000u64;
        let halvings = current_height / halving_interval;
        
        // اگر تمام هاوینگ‌ها تمام شده باشد، پاداش صفر است
        if halvings >= 8 { // 50 / 2^8 ≈ 0.2, که عملاً صفر است
            return 0.0;
        }
        
        // محاسبه پاداش فعلی
        let current_reward = self.mining_reward / (2.0_f64.powi(halvings as i32));
        
        // بررسی حداکثر عرضه (10 میلیون)
        let max_supply = 10_000_000.0;
        let current_supply = self.get_total_supply();
        
        if current_supply >= max_supply {
            return 0.0; // دیگر پاداش استخراج نیست
        }
        
        // اطمینان از عدم تجاوز از حداکثر عرضه
        if current_supply + current_reward > max_supply {
            return max_supply - current_supply;
        }
        
        current_reward
    }

    // بررسی اینکه آیا هاوینگ اتفاق افتاده است
    pub fn check_halving(&self) -> Option<u64> {
        let current_height = self.chain.len() as u64;
        let halving_interval = 100_000u64;
        
        if current_height > 0 && current_height % halving_interval == 0 {
            Some(current_height / halving_interval)
        } else {
            None
        }
    }

    pub fn get_latest_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        // بررسی صحت تراکنش
        if !transaction.is_valid() {
            return Err("تراکنش نامعتبر است".to_string());
        }

        // بررسی موجودی (برای تراکنش‌های غیر سیستمی)
        if !transaction.from_address.is_empty() {
            let balance = self.get_balance(&transaction.from_address);
            if balance < transaction.amount {
                return Err(format!(
                    "موجودی ناکافی! موجودی فعلی: {}, مقدار درخواستی: {}",
                    balance, transaction.amount
                ));
            }
        }

        self.pending_transactions.push(transaction);
        println!("📝 تراکنش جدید به صف انتظار اضافه شد");
        Ok(())
    }

    pub async fn mine_pending_transactions(&mut self, mining_reward_address: &str) -> Result<(), String> {
        let current_reward = self.calculate_mining_reward();
        let mut transactions = Vec::new();

        // اضافه کردن تراکنش پاداش استخراج (اگر هنوز پاداش وجود دارد)
        if current_reward > 0.0 {
            let reward_transaction = Transaction::mining_reward(
                mining_reward_address.to_string(),
                current_reward,
            );
            transactions.push(reward_transaction);
        }

        // اضافه کردن تراکنش‌های در انتظار
        transactions.extend(self.pending_transactions.clone());

        // اگر هیچ تراکنشی نباشد، بلاک خالی نمی‌سازیم
        if transactions.is_empty() {
            return Err("هیچ تراکنش برای استخراج وجود ندارد".to_string());
        }

        // ایجاد بلاک جدید
        let previous_hash = self.get_latest_block()
            .map(|block| block.hash.clone())
            .unwrap_or_default();

        let mut new_block = Block::new(
            self.chain.len() as u64,
            transactions,
            previous_hash,
        );

        // استخراج بلاک (Proof of Work)
        println!("⛏️  شروع استخراج بلاک...");
        let start_time = std::time::Instant::now();
        
        // استخراج در یک task جداگانه برای non-blocking بودن
        let difficulty = self.difficulty;
        new_block = tokio::task::spawn_blocking(move || {
            new_block.mine_block(difficulty); // سختی 5 صفر
            new_block
        }).await.map_err(|_| "خطا در استخراج بلاک")?;

        let duration = start_time.elapsed();
        println!("⏱️  زمان استخراج: {:.2} ثانیه", duration.as_secs_f64());

        // بررسی صحت بلاک
        let previous_hash = self.get_latest_block()
            .map(|block| block.hash.as_str())
            .unwrap_or("");

        if !new_block.is_valid(previous_hash) {
            return Err("بلاک استخراج شده نامعتبر است".to_string());
        }

        // به‌روزرسانی موجودی‌ها
        self.update_balances_from_block(&new_block);

        // اضافه کردن بلاک به زنجیره
        self.chain.push(new_block);

        // پاک کردن تراکنش‌های در انتظار
        self.pending_transactions.clear();

        // بررسی هاوینگ
        if let Some(halving_count) = self.check_halving() {
            let new_reward = self.calculate_mining_reward();
            println!("🎉 هاوینگ #{} اتفاق افتاد! پاداش جدید: {} TEC", halving_count, new_reward);
        }

        println!("✅ بلاک جدید با موفقیت به زنجیره اضافه شد!");
        println!("💎 کل عرضه: {} TEC از 10,000,000 TEC", self.get_total_supply());
        
        Ok(())
    }

    fn update_balances_from_block(&mut self, block: &Block) {
        for transaction in &block.transactions {
            // کسر از فرستنده (اگر سیستمی نباشد)
            if !transaction.from_address.is_empty() {
                let sender_balance = self.balances.get(&transaction.from_address).unwrap_or(&0.0);
                self.balances.insert(
                    transaction.from_address.clone(),
                    sender_balance - transaction.amount,
                );
            }

            // اضافه به گیرنده
            let receiver_balance = self.balances.get(&transaction.to_address).unwrap_or(&0.0);
            self.balances.insert(
                transaction.to_address.clone(),
                receiver_balance + transaction.amount,
            );
        }
    }

    pub fn get_balance(&self, address: &str) -> f64 {
        *self.balances.get(address).unwrap_or(&0.0)
    }

    pub fn is_chain_valid(&self) -> bool {
        if self.chain.is_empty() {
            return true;
        }

        // بررسی بلاک Genesis
        if self.chain[0].index != 0 || !self.chain[0].previous_hash.is_empty() {
            println!("❌ بلاک Genesis نامعتبر است");
            return false;
        }

        // بررسی باقی بلاک‌ها
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // بررسی صحت بلاک فعلی
            if !current_block.is_valid(&previous_block.hash) {
                println!("❌ بلاک {} نامعتبر است", i);
                return false;
            }

            // بررسی اتصال با بلاک قبلی
            if current_block.previous_hash != previous_block.hash {
                println!("❌ اتصال بین بلاک‌های {} و {} قطع است", i - 1, i);
                return false;
            }

            // بررسی ترتیب index
            if current_block.index != previous_block.index + 1 {
                println!("❌ ترتیب بلاک‌ها اشتباه است");
                return false;
            }
        }

        println!("✅ زنجیره بلاک معتبر است");
        true
    }

    pub fn get_chain_length(&self) -> usize {
        self.chain.len()
    }

    pub fn get_pending_transactions_count(&self) -> usize {
        self.pending_transactions.len()
    }

    pub fn get_total_supply(&self) -> f64 {
        self.balances.values().sum()
    }

    pub fn get_chain_info(&self) -> BlockchainInfo {
        BlockchainInfo {
            total_blocks: self.chain.len(),
            total_transactions: self.chain.iter().map(|b| b.transactions.len()).sum(),
            total_supply: self.get_total_supply(),
            difficulty: self.difficulty,
            mining_reward: self.mining_reward,
            pending_transactions: self.pending_transactions.len(),
        }
    }

    // صادر کردن زنجیره به JSON
    pub fn export_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self)
            .map_err(|_| "خطا در صادر کردن زنجیره".to_string())
    }

    // وارد کردن زنجیره از JSON
    pub fn import_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|_| "خطا در وارد کردن زنجیره".to_string())
    }

    // دریافت تراکنش‌های یک آدرس
    pub fn get_transactions_for_address(&self, address: &str) -> Vec<&Transaction> {
        let mut transactions = Vec::new();
        
        for block in &self.chain {
            for transaction in &block.transactions {
                if transaction.from_address == address || transaction.to_address == address {
                    transactions.push(transaction);
                }
            }
        }
        
        transactions
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockchainInfo {
    pub total_blocks: usize,
    pub total_transactions: usize,
    pub total_supply: f64,
    pub difficulty: usize,
    pub mining_reward: f64,
    pub pending_transactions: usize,
}

impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "⛓️  زنجیره بلاک RustCoin")?;
        writeln!(f, "========================")?;
        writeln!(f, "📊 تعداد بلاک‌ها: {}", self.chain.len())?;
        writeln!(f, "⚙️  سختی: {}", self.difficulty)?;
        writeln!(f, "💰 پاداش استخراج: {} RustCoin", self.mining_reward)?;
        writeln!(f, "⏳ تراکنش‌های در انتظار: {}", self.pending_transactions.len())?;
        writeln!(f, "💎 کل عرضه: {} RustCoin", self.get_total_supply())?;
        writeln!(f, "========================")?;
        
        for block in &self.chain {
            writeln!(f, "{}", block)?;
            writeln!(f, "------------------------")?;
        }
        
        Ok(())
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}