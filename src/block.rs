use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use crate::transaction::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub merkle_root: String,
}

impl Block {
    pub fn new(
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
    ) -> Self {
        let timestamp = chrono::Utc::now();
        let merkle_root = Self::calculate_merkle_root(&transactions);
        
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
            merkle_root,
        };
        
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.previous_hash,
            self.merkle_root,
            self.nonce,
            serde_json::to_string(&self.transactions).unwrap_or_default()
        );
        
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        let start_time = std::time::Instant::now();
        
        println!("⛏️  شروع استخراج بلاک {} با سختی {}...", self.index, difficulty);
        
        loop {
            self.hash = self.calculate_hash();
            
            if self.hash.starts_with(&target) {
                let duration = start_time.elapsed();
                println!(
                    "✅ بلاک {} استخراج شد! Nonce: {}, زمان: {:.2}s, Hash: {}",
                    self.index,
                    self.nonce,
                    duration.as_secs_f64(),
                    &self.hash[..16]
                );
                break;
            }
            
            self.nonce += 1;
            
            // نمایش پیشرفت هر 100000 تلاش
            if self.nonce % 100000 == 0 {
                println!("🔄 تلاش {}: {}", self.nonce, &self.hash[..16]);
            }
        }
    }

    pub fn is_valid(&self, previous_hash: &str) -> bool {
        // بررسی hash قبلی
        if self.previous_hash != previous_hash {
            println!("❌ Hash قبلی اشتباه است");
            return false;
        }

        // بررسی hash فعلی
        if self.hash != self.calculate_hash() {
            println!("❌ Hash فعلی اشتباه است");
            return false;
        }

        // بررسی صحت تراکنش‌ها
        for transaction in &self.transactions {
            if !transaction.is_valid() {
                println!("❌ تراکنش نامعتبر پیدا شد");
                return false;
            }
        }

        // بررسی Merkle Root
        let calculated_merkle = Self::calculate_merkle_root(&self.transactions);
        if self.merkle_root != calculated_merkle {
            println!("❌ Merkle Root اشتباه است");
            return false;
        }

        true
    }

    fn calculate_merkle_root(transactions: &[Transaction]) -> String {
        if transactions.is_empty() {
            return String::new();
        }

        let mut hashes: Vec<String> = transactions
            .iter()
            .map(|tx| tx.calculate_hash())
            .collect();

        while hashes.len() > 1 {
            let mut new_hashes = Vec::new();
            
            for chunk in hashes.chunks(2) {
                let combined = if chunk.len() == 2 {
                    format!("{}{}", chunk[0], chunk[1])
                } else {
                    format!("{}{}", chunk[0], chunk[0]) // تکرار آخرین hash اگر فرد باشد
                };
                
                let mut hasher = Sha256::new();
                hasher.update(combined.as_bytes());
                new_hashes.push(hex::encode(hasher.finalize()));
            }
            
            hashes = new_hashes;
        }

        hashes[0].clone()
    }

    // بلاک Genesis
    pub fn genesis() -> Self {
        let genesis_transaction = Transaction::genesis_transaction();
        let mut genesis_block = Block::new(0, vec![genesis_transaction], String::new());
        genesis_block.mine_block(2); // سختی کم برای Genesis
        genesis_block
    }

    pub fn get_transaction_count(&self) -> usize {
        self.transactions.len()
    }

    pub fn get_total_amount(&self) -> f64 {
        self.transactions.iter().map(|tx| tx.amount).sum()
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "📦 بلاک #{}", self.index)?;
        writeln!(f, "🕐 زمان: {}", self.timestamp.format("%Y-%m-%d %H:%M:%S"))?;
        writeln!(f, "🔗 Hash قبلی: {}", if self.previous_hash.is_empty() { "Genesis" } else { &self.previous_hash[..16] })?;
        writeln!(f, "🆔 Hash: {}", &self.hash[..16])?;
        writeln!(f, "🎯 Nonce: {}", self.nonce)?;
        writeln!(f, "🌳 Merkle Root: {}", &self.merkle_root[..16])?;
        writeln!(f, "📊 تعداد تراکنش: {}", self.transactions.len())?;
        writeln!(f, "💰 مجموع مقدار: {} RustCoin", self.get_total_amount())?;
        
        for (i, tx) in self.transactions.iter().enumerate() {
            writeln!(f, "  {}. {}", i + 1, tx)?;
        }
        
        Ok(())
    }
}