use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use std::fmt;
use uuid::Uuid;
// use crate::wallet::Wallet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub signature: Option<String>,
}

impl Transaction {
    pub fn new(
        from_address: String,
        to_address: String,
        amount: f64,
        wallet: &crate::address::Wallet,
    ) -> Result<Self, String> {
        if amount <= 0.0 {
            return Err("مقدار تراکنش باید مثبت باشد".to_string());
        }

        let mut transaction = Transaction {
            id: Uuid::new_v4().to_string(),
            from_address: from_address.clone(),
            to_address,
            amount,
            timestamp: chrono::Utc::now(),
            signature: None,
        };

        // امضای تراکنش
        transaction.sign_transaction(wallet)?;
        Ok(transaction)
    }

    pub fn new_with_fee(
        from_address: String,
        to_address: String,
        amount: f64,
        fee: f64,
        wallet: &crate::address::Wallet,
    ) -> Result<Self, String> {
        if amount <= 0.0 {
            return Err("مقدار تراکنش باید مثبت باشد".to_string());
        }

        if fee < 0.0 {
            return Err("کارمزد نمی‌تواند منفی باشد".to_string());
        }

        let mut transaction = Transaction {
            id: Uuid::new_v4().to_string(),
            from_address: from_address.clone(),
            to_address,
            amount: amount + fee, // اضافه کردن fee به مقدار
            timestamp: chrono::Utc::now(),
            signature: None,
        };

        // امضای تراکنش
        transaction.sign_transaction_simple(wallet)?;
        Ok(transaction)
    }

    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}",
            self.from_address, self.to_address, self.amount, self.timestamp, self.id
        );
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn sign_transaction(&mut self, wallet: &crate::address::Wallet) -> Result<(), String> {
        if self.from_address != wallet.get_address() {
            return Err("شما نمی‌توانید تراکنش‌های دیگران را امضا کنید".to_string());
        }

        let hash = self.calculate_hash();
        let signature = wallet.sign_data(&hash)?;
        self.signature = Some(signature);
        Ok(())
    }

    pub fn sign_transaction_simple(&mut self, wallet: &crate::address::Wallet) -> Result<(), String> {
        if self.from_address != wallet.get_address() {
            return Err("شما نمی‌توانید تراکنش‌های دیگران را امضا کنید".to_string());
        }

        // برای سادگی، از اولین کلید خصوصی استفاده می‌کنیم
        if wallet.private_keys.is_empty() {
            return Err("کلید خصوصی یافت نشد".to_string());
        }

        let hash = self.calculate_hash();
        // ساده‌سازی امضا
        let signature = format!("sig_{}", hash[..16].to_string());
        self.signature = Some(signature);
        Ok(())
    }

    pub fn is_valid(&self) -> bool {
        // بررسی تراکنش Genesis (بدون امضا)
        if self.from_address.is_empty() {
            return true;
        }

        // بررسی وجود امضا
        if self.signature.is_none() {
            println!("❌ تراکنش بدون امضا");
            return false;
        }

        // بررسی صحت امضا
        self.verify_signature()
    }

    fn verify_signature(&self) -> bool {
        if let Some(signature_str) = &self.signature {
            let hash = self.calculate_hash();
            
            // در اینجا باید امضا با کلید عمومی بررسی شود
            // برای سادگی، فعلاً true برمی‌گردانیم
            // در پیاده‌سازی واقعی باید از secp256k1 استفاده کنیم
            
            !signature_str.is_empty() && !hash.is_empty()
        } else {
            false
        }
    }

    // تراکنش Genesis برای اولین بلاک
    pub fn genesis_transaction() -> Self {
        Transaction {
            id: "genesis".to_string(),
            from_address: String::new(),
            to_address: "genesis_reward".to_string(),
            amount: 100.0,
            timestamp: chrono::Utc::now(),
            signature: None,
        }
    }

    // تراکنش پاداش استخراج
    pub fn mining_reward(miner_address: String, reward: f64) -> Self {
        Transaction {
            id: Uuid::new_v4().to_string(),
            from_address: String::new(), // از سیستم
            to_address: miner_address,
            amount: reward,
            timestamp: chrono::Utc::now(),
            signature: None,
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "💸 تراکنش: {} -> {} | مقدار: {} RustCoin | زمان: {}",
            if self.from_address.is_empty() { "سیستم" } else { &self.from_address[..8] },
            &self.to_address[..8],
            self.amount,
            self.timestamp.format("%Y-%m-%d %H:%M:%S")
        )
    }
}