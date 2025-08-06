use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Digest, Sha256};
use ripemd::{Ripemd160, Digest as RipemdDigest};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddressType {
    Bech32,
    P2SH { required: usize, total: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub address: String,
    pub address_type: AddressType,
    pub private_keys: Vec<String>,
    pub public_keys: Vec<String>,
    pub script: Option<String>,
}

impl Wallet {
    pub fn get_address(&self) -> String {
        self.address.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletRequest {
    pub action: String,
    pub address_type: String,
    pub required: Option<usize>,
    pub total: Option<usize>,
}

pub struct AddressGenerator;

impl AddressGenerator {
    pub fn generate_wallet(wallet_type: AddressType) -> Result<Wallet, String> {
        match wallet_type {
            AddressType::Bech32 => Self::generate_bech32_wallet(),
            AddressType::P2SH { required, total } => Self::generate_p2sh_wallet(required, total),
        }
    }

    fn generate_bech32_wallet() -> Result<Wallet, String> {
        let (private_key, public_key) = Self::generate_keypair()?;
        let address = Self::create_bech32_address(&public_key)?;

        Ok(Wallet {
            address,
            address_type: AddressType::Bech32,
            private_keys: vec![private_key],
            public_keys: vec![public_key],
            script: None,
        })
    }

    fn generate_p2sh_wallet(required: usize, total: usize) -> Result<Wallet, String> {
        if required > total || required == 0 || total == 0 {
            return Err("تنظیمات multisig نامعتبر است".to_string());
        }

        let mut private_keys = Vec::new();
        let mut public_keys = Vec::new();

        // تولید کلیدهای مورد نیاز
        for _ in 0..total {
            let (priv_key, pub_key) = Self::generate_keypair()?;
            private_keys.push(priv_key);
            public_keys.push(pub_key);
        }

        // ایجاد script برای multisig
        let script = Self::create_multisig_script(required, &public_keys)?;
        let address = Self::create_p2sh_address(&script)?;

        Ok(Wallet {
            address,
            address_type: AddressType::P2SH { required, total },
            private_keys,
            public_keys,
            script: Some(script),
        })
    }

    fn generate_keypair() -> Result<(String, String), String> {
        let secp = Secp256k1::new();
        let mut rng = OsRng;
        
        // تولید کلید خصوصی
        let mut key_bytes = [0u8; 32];
        let private_key = loop {
            rng.fill_bytes(&mut key_bytes);
            if let Ok(private_key) = SecretKey::from_slice(&key_bytes) {
                break private_key;
            }
        };
        
        // تولید کلید عمومی
        let public_key = PublicKey::from_secret_key(&secp, &private_key);
        
        Ok((
            hex::encode(private_key.secret_bytes()),
            hex::encode(public_key.serialize()),
        ))
    }

    fn create_bech32_address(public_key_hex: &str) -> Result<String, String> {
        let public_key_bytes = hex::decode(public_key_hex)
            .map_err(|_| "کلید عمومی نامعتبر است".to_string())?;
        
        // SHA256 hash
        let mut sha_hasher = Sha256::new();
        sha_hasher.update(&public_key_bytes);
        let sha_hash = sha_hasher.finalize();
        
        // RIPEMD160 hash
        let mut ripemd_hasher = Ripemd160::new();
        ripemd_hasher.update(sha_hash);
        let hash160 = ripemd_hasher.finalize();
        
        // تبدیل به bech32 با پیشوند tc1q
        let address = format!("tc1q{}", hex::encode(hash160));
        
        Ok(address)
    }

    fn create_p2sh_address(script: &str) -> Result<String, String> {
        let script_bytes = hex::decode(script)
            .map_err(|_| "اسکریپت نامعتبر است".to_string())?;
        
        // SHA256 hash از script
        let mut sha_hasher = Sha256::new();
        sha_hasher.update(&script_bytes);
        let sha_hash = sha_hasher.finalize();
        
        // RIPEMD160 hash
        let mut ripemd_hasher = Ripemd160::new();
        ripemd_hasher.update(sha_hash);
        let hash160 = ripemd_hasher.finalize();
        
        // P2SH address با پیشوند tc3
        let address = format!("tc3{}", hex::encode(hash160));
        
        Ok(address)
    }

    fn create_multisig_script(required: usize, public_keys: &[String]) -> Result<String, String> {
        let mut script = Vec::new();
        
        // OP_M (تعداد امضای مورد نیاز)
        script.push(0x50 + required as u8); // OP_1 تا OP_16
        
        // اضافه کردن کلیدهای عمومی
        for pub_key_hex in public_keys {
            let pub_key_bytes = hex::decode(pub_key_hex)
                .map_err(|_| "کلید عمومی نامعتبر است".to_string())?;
            
            script.push(pub_key_bytes.len() as u8);
            script.extend(pub_key_bytes);
        }
        
        // OP_N (تعداد کل کلیدها)
        script.push(0x50 + public_keys.len() as u8);
        
        // OP_CHECKMULTISIG
        script.push(0xae);
        
        Ok(hex::encode(script))
    }

    pub fn validate_address(address: &str) -> bool {
        if address.starts_with("tc1q") {
            // Bech32 validation
            address.len() >= 42 && address.chars().all(|c| c.is_ascii_hexdigit() || c == 'q')
        } else if address.starts_with("tc3") {
            // P2SH validation
            address.len() >= 35 && address[3..].chars().all(|c| c.is_ascii_hexdigit())
        } else {
            false
        }
    }

    pub fn get_address_type(address: &str) -> Option<String> {
        if address.starts_with("tc1q") {
            Some("bech32".to_string())
        } else if address.starts_with("tc3") {
            Some("p2sh".to_string())
        } else {
            None
        }
    }
}

// ساختار برای مدیریت کیف پول‌ها
pub struct WalletManager {
    wallets: HashMap<String, Wallet>,
}

impl WalletManager {
    pub fn new() -> Self {
        WalletManager {
            wallets: HashMap::new(),
        }
    }

    pub fn create_wallet(&mut self, address_type: AddressType) -> Result<Wallet, String> {
        let wallet = AddressGenerator::generate_wallet(address_type)?;
        let address = wallet.address.clone();
        self.wallets.insert(address.clone(), wallet.clone());
        Ok(wallet)
    }

    pub fn get_wallet(&self, address: &str) -> Option<&Wallet> {
        self.wallets.get(address)
    }

    pub fn list_wallets(&self) -> Vec<&Wallet> {
        self.wallets.values().collect()
    }

    pub fn sign_transaction(&self, address: &str, data: &str) -> Result<String, String> {
        let wallet = self.wallets.get(address)
            .ok_or("کیف پول یافت نشد".to_string())?;

        if wallet.private_keys.is_empty() {
            return Err("کلید خصوصی یافت نشد".to_string());
        }

        // برای سادگی، از اولین کلید خصوصی استفاده می‌کنیم
        let private_key_hex = &wallet.private_keys[0];
        let private_key_bytes = hex::decode(private_key_hex)
            .map_err(|_| "کلید خصوصی نامعتبر است".to_string())?;

        let private_key = SecretKey::from_slice(&private_key_bytes)
            .map_err(|_| "کلید خصوصی نامعتبر است".to_string())?;

        // هش کردن داده
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let hash = hasher.finalize();

        let secp = Secp256k1::new();
        let message = secp256k1::Message::from_digest_slice(&hash)
            .map_err(|_| "خطا در ایجاد پیام".to_string())?;

        let signature = secp.sign_ecdsa(&message, &private_key);
        Ok(hex::encode(signature.serialize_compact()))
    }
}

impl Default for WalletManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bech32_address_generation() {
        let wallet = AddressGenerator::generate_bech32_wallet().unwrap();
        assert!(wallet.address.starts_with("tc1q"));
        assert_eq!(wallet.private_keys.len(), 1);
        assert_eq!(wallet.public_keys.len(), 1);
    }

    #[test]
    fn test_p2sh_address_generation() {
        let wallet = AddressGenerator::generate_p2sh_wallet(2, 3).unwrap();
        assert!(wallet.address.starts_with("tc3"));
        assert_eq!(wallet.private_keys.len(), 3);
        assert_eq!(wallet.public_keys.len(), 3);
        assert!(wallet.script.is_some());
    }

    #[test]
    fn test_address_validation() {
        assert!(AddressGenerator::validate_address("tc1q1234567890abcdef"));
        assert!(AddressGenerator::validate_address("tc31234567890abcdef"));
        assert!(!AddressGenerator::validate_address("invalid_address"));
    }
}