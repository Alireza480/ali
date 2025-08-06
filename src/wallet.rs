use secp256k1::{Secp256k1, SecretKey, PublicKey, Message};
use sha2::{Digest, Sha256};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    private_key: String,
    public_key: String,
    address: String,
}

impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng;
        
        // Generate کلید خصوصی
        let mut key_bytes = [0u8; 32];
        let private_key = loop {
            rng.fill_bytes(&mut key_bytes);
            if let Ok(private_key) = SecretKey::from_slice(&key_bytes) {
                break private_key;
            }
        };
        
        // Generate کلید عمومی
        let public_key = PublicKey::from_secret_key(&secp, &private_key);
        
        // Generate address از کلید عمومی
        let address = Self::generate_address(&public_key);
        
        Wallet {
            private_key: hex::encode(private_key.secret_bytes()),
            public_key: hex::encode(public_key.serialize()),
            address,
        }
    }

    pub fn from_private_key(private_key_hex: &str) -> Result<Self, String> {
        let private_key_bytes = hex::decode(private_key_hex)
            .map_err(|_| "کلید خصوصی is invalid".to_string())?;
        
        let private_key = SecretKey::from_slice(&private_key_bytes)
            .map_err(|_| "کلید خصوصی is invalid".to_string())?;
        
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &private_key);
        let address = Self::generate_address(&public_key);
        
        Ok(Wallet {
            private_key: private_key_hex.to_string(),
            public_key: hex::encode(public_key.serialize()),
            address,
        })
    }

    fn generate_address(public_key: &PublicKey) -> String {
        let public_key_bytes = public_key.serialize();
        let mut hasher = Sha256::new();
        hasher.update(&public_key_bytes);
        let hash = hasher.finalize();
        
        // استفاده از 20 بایت اول hash برای address
        let address_bytes = &hash[..20];
        format!("RC{}", hex::encode(address_bytes)) // RC = RustCoin
    }

    pub fn get_address(&self) -> String {
        self.address.clone()
    }

    pub fn get_public_key(&self) -> String {
        self.public_key.clone()
    }

    pub fn get_private_key(&self) -> String {
        self.private_key.clone()
    }

    pub fn sign_data(&self, data: &str) -> Result<String, String> {
        let private_key_bytes = hex::decode(&self.private_key)
            .map_err(|_| "کلید خصوصی is invalid".to_string())?;
        
        let private_key = SecretKey::from_slice(&private_key_bytes)
            .map_err(|_| "کلید خصوصی is invalid".to_string())?;
        
        // Hash کRejectن داده
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let hash = hasher.finalize();
        
        let secp = Secp256k1::new();
        let message = Message::from_digest_slice(&hash)
            .map_err(|_| "Error in ایجاد پیام".to_string())?;
        
        let signature = secp.sign_ecdsa(&message, &private_key);
        Ok(hex::encode(signature.serialize_compact()))
    }

    pub fn verify_signature(data: &str, signature_hex: &str, public_key_hex: &str) -> bool {
        match (
            hex::decode(signature_hex),
            hex::decode(public_key_hex)
        ) {
            (Ok(signature_bytes), Ok(public_key_bytes)) => {
                // Hash کRejectن داده
                let mut hasher = Sha256::new();
                hasher.update(data.as_bytes());
                let hash = hasher.finalize();
                
                match (
                    secp256k1::ecdsa::Signature::from_compact(&signature_bytes),
                    Message::from_digest_slice(&hash),
                    PublicKey::from_slice(&public_key_bytes)
                ) {
                    (Ok(signature), Ok(message), Ok(public_key)) => {
                        let secp = Secp256k1::new();
                        secp.verify_ecdsa(&message, &signature, &public_key).is_ok()
                    }
                    _ => false
                }
            }
            _ => false
        }
    }

    // Create wallet از seed phrase (برای آینده)
    pub fn from_seed(_seed: &str) -> Result<Self, String> {
        // پیاده‌سازی BIP39 برای آینده
        // فعلاً یک wallet تصادفی برمی‌گRejectانیم
        Ok(Self::new())
    }

    // صادر کRejectن wallet به فرمت JSON
    pub fn export_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self)
            .map_err(|_| "Error in صادر کRejectن wallet".to_string())
    }

    // واReject کRejectن wallet از فرمت JSON
    pub fn import_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|_| "Error in واReject کRejectن wallet".to_string())
    }
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "👛 wallet RustCoin")?;
        writeln!(f, "📍 address: {}", self.address)?;
        writeln!(f, "🔑 کلید عمومی: {}...", &self.public_key[..16])?;
        writeln!(f, "🔐 کلید خصوصی: {}... (محرمانه!)", &self.private_key[..16])?;
        Ok(())
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let wallet = Wallet::new();
        assert!(!wallet.get_address().is_empty());
        assert!(wallet.get_address().starts_with("RC"));
    }

    #[test]
    fn test_signature() {
        let wallet = Wallet::new();
        let data = "test message";
        
        let signature = wallet.sign_data(data).unwrap();
        let is_valid = Wallet::verify_signature(data, &signature, &wallet.get_public_key());
        
        assert!(is_valid);
    }

    #[test]
    fn test_wallet_export_import() {
        let wallet1 = Wallet::new();
        let json = wallet1.export_json().unwrap();
        let wallet2 = Wallet::import_json(&json).unwrap();
        
        assert_eq!(wallet1.get_address(), wallet2.get_address());
    }
}