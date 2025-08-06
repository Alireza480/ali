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
    pub fn get_difficulty(&self) -> usize {
        self.difficulty
    }
    pub fn new() -> Self {
        Blockchain {
            chain: Vec::new(),
            difficulty: 5, // Difficulty 5 zeros
            pending_transactions: Vec::new(),
            mining_reward: 50.0, // Initial reward 50 TEC
            balances: HashMap::new(),
        }
    }

    pub fn create_genesis_block(&mut self) {
        if self.chain.is_empty() {
            println!("🌟 Creating Genesis block...");
            let genesis_block = Block::genesis();
            
            // Update balances from Genesis block
            self.update_balances_from_block(&genesis_block);
            
            self.chain.push(genesis_block);
            println!("✅ Genesis block created!");
            println!("💎 Current total supply: {} TEC", self.get_total_supply());
        }
    }

    // Calculate mining reward considering halving
    pub fn calculate_mining_reward(&self) -> f64 {
        let current_height = self.chain.len() as u64;
        let halving_interval = 100_000u64;
        let halvings = current_height / halving_interval;
        
        // If all halvings are completed, reward is zero
        if halvings >= 8 { // 50 / 2^8 ≈ 0.2, practically zero
            return 0.0;
        }
        
        // Calculate current reward
        let current_reward = self.mining_reward / (2.0_f64.powi(halvings as i32));
        
        // Check maximum supply (10 million)
        let max_supply = 10_000_000.0;
        let current_supply = self.get_total_supply();
        
        if current_supply >= max_supply {
            return 0.0; // No more mining rewards
        }
        
        // Ensure not exceeding maximum supply
        if current_supply + current_reward > max_supply {
            return max_supply - current_supply;
        }
        
        current_reward
    }

    // Check if halving has occurred
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
        // Validate transaction
        if !transaction.is_valid() {
            return Err("Invalid transaction".to_string());
        }

        // Check balance (for non-system transactions)
        if !transaction.from_address.is_empty() {
            let balance = self.get_balance(&transaction.from_address);
            if balance < transaction.amount {
                return Err(format!(
                    "Insufficient balance! Current: {}, Required: {}",
                    balance, transaction.amount
                ));
            }
        }

        self.pending_transactions.push(transaction);
        println!("📝 New transaction added to pending queue");
        Ok(())
    }

    pub async fn mine_pending_transactions(&mut self, mining_reward_address: &str) -> Result<(), String> {
        let current_reward = self.calculate_mining_reward();
        let mut transactions = Vec::new();

        // Add transaction reward mining (اگر هنوز reward وجود داReject)
        if current_reward > 0.0 {
            let reward_transaction = Transaction::mining_reward(
                mining_reward_address.to_string(),
                current_reward,
            );
            transactions.push(reward_transaction);
        }

        // Add transaction‌های pending
        transactions.extend(self.pending_transactions.clone());

        // اگر هیچ transactionی نباشد، block خالی نمی‌سازیم
        if transactions.is_empty() {
            return Err("هیچ transaction برای mining وجود نداReject".to_string());
        }

        // Create block جدید
        let previous_hash = self.get_latest_block()
            .map(|block| block.hash.clone())
            .unwrap_or_default();

        let mut new_block = Block::new(
            self.chain.len() as u64,
            transactions,
            previous_hash,
        );

        // Mining block (Proof of Work)
        println!("⛏️  Start mining block...");
        let start_time = std::time::Instant::now();
        
        // Mining در یک task جداگانه برای non-blocking بودن
        let difficulty = self.difficulty;
        new_block = tokio::task::spawn_blocking(move || {
            new_block.mine_block(difficulty); // difficulty 5 صفر
            new_block
        }).await.map_err(|_| "Error in mining block")?;

        let duration = start_time.elapsed();
        println!("⏱️  زمان mining: {:.2} ثانیه", duration.as_secs_f64());

        // Check صحت block
        let previous_hash = self.get_latest_block()
            .map(|block| block.hash.as_str())
            .unwrap_or("");

        if !new_block.is_valid(previous_hash) {
            return Err("block minedه is invalid".to_string());
        }

        // Update balance‌ها
        self.update_balances_from_block(&new_block);

        // Add block به زنجیره
        self.chain.push(new_block);

        // پاک کRejectن transaction‌های pending
        self.pending_transactions.clear();

        // Check halving
        if let Some(halving_count) = self.check_halving() {
            let new_reward = self.calculate_mining_reward();
            println!("🎉 halving #{} اتفاق افتاد! reward جدید: {} TEC", halving_count, new_reward);
        }

        println!("✅ block جدید successfully به زنجیره added!");
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

        // Check block Genesis
        if self.chain[0].index != 0 || !self.chain[0].previous_hash.is_empty() {
            println!("❌ block Genesis is invalid");
            return false;
        }

        // Check باقی block‌ها
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // Check صحت block فعلی
            if !current_block.is_valid(&previous_block.hash) {
                println!("❌ block {} is invalid", i);
                return false;
            }

            // Check اتصال با block قبلی
            if current_block.previous_hash != previous_block.hash {
                println!("❌ اتصال بین block‌های {} و {} قطع است", i - 1, i);
                return false;
            }

            // Check ترتیب index
            if current_block.index != previous_block.index + 1 {
                println!("❌ ترتیب block‌ها اشتباه است");
                return false;
            }
        }

        println!("✅ زنجیره block معتبر است");
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

    // صادر کRejectن زنجیره به JSON
    pub fn export_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self)
            .map_err(|_| "Error in صادر کRejectن زنجیره".to_string())
    }

    // واReject کRejectن زنجیره از JSON
    pub fn import_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|_| "Error in واReject کRejectن زنجیره".to_string())
    }

    // Receive transaction‌های یک address
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
    difficulty: usize,
    pub mining_reward: f64,
    pub pending_transactions: usize,
}

impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "⛓️  زنجیره block RustCoin")?;
        writeln!(f, "========================")?;
        writeln!(f, "📊 تعداد block‌ها: {}", self.chain.len())?;
        writeln!(f, "⚙️  difficulty: {}", self.difficulty)?;
        writeln!(f, "💰 reward mining: {} RustCoin", self.mining_reward)?;
        writeln!(f, "⏳ transaction‌های pending: {}", self.pending_transactions.len())?;
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