use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::block::Block;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    // درخواست اتصال
    Handshake {
        node_id: String,
        version: String,
        blockchain_height: u64,
    },
    // پاسخ اتصال
    HandshakeResponse {
        node_id: String,
        accepted: bool,
    },
    // انتشار تراکنش جدید
    NewTransaction(Transaction),
    // انتشار بلاک جدید
    NewBlock(Block),
    // درخواست همگام‌سازی بلاک چین
    SyncRequest {
        from_height: u64,
    },
    // پاسخ همگام‌سازی
    SyncResponse {
        blocks: Vec<Block>,
    },
    // درخواست peers
    GetPeers,
    // پاسخ peers
    PeersResponse {
        peers: Vec<String>,
    },
    // پینگ برای بررسی اتصال
    Ping,
    // پانگ پاسخ پینگ
    Pong,
}

#[derive(Debug, Clone)]
pub struct Peer {
    pub id: String,
    pub address: SocketAddr,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub blockchain_height: u64,
}

pub struct NetworkNode {
    node_id: String,
    listen_address: SocketAddr,
    peers: Arc<Mutex<HashMap<String, Peer>>>,
    blockchain: Arc<Mutex<Blockchain>>,
    is_running: Arc<Mutex<bool>>,
}

impl NetworkNode {
    pub fn new(
        node_id: String,
        listen_address: SocketAddr,
        blockchain: Arc<Mutex<Blockchain>>,
    ) -> Self {
        NetworkNode {
            node_id,
            listen_address,
            peers: Arc::new(Mutex::new(HashMap::new())),
            blockchain,
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn start(&self) -> Result<(), String> {
        {
            let mut is_running = self.is_running.lock().await;
            if *is_running {
                return Err("شبکه در حال حاضر فعال است".to_string());
            }
            *is_running = true;
        }

        println!("🌐 شروع نود شبکه {} در آدرس {}", self.node_id, self.listen_address);

        let listener = TcpListener::bind(self.listen_address)
            .await
            .map_err(|e| format!("خطا در باز کردن پورت: {}", e))?;

        println!("👂 در حال گوش دادن به اتصالات...");

        // شروع loop برای پذیرش اتصالات
        let peers = self.peers.clone();
        let blockchain = self.blockchain.clone();
        let node_id = self.node_id.clone();
        let is_running = self.is_running.clone();

        tokio::spawn(async move {
            while *is_running.lock().await {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        println!("🔗 اتصال جدید از: {}", addr);
                        
                        let peers_clone = peers.clone();
                        let blockchain_clone = blockchain.clone();
                        let node_id_clone = node_id.clone();
                        
                        tokio::spawn(async move {
                            Self::handle_connection(
                                stream,
                                addr,
                                peers_clone,
                                blockchain_clone,
                                node_id_clone,
                            ).await;
                        });
                    }
                    Err(e) => {
                        println!("❌ خطا در پذیرش اتصال: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn stop(&self) {
        let mut is_running = self.is_running.lock().await;
        *is_running = false;
        println!("🛑 نود شبکه متوقف شد");
    }

    pub async fn connect_to_peer(&self, peer_address: &str) -> Result<(), String> {
        let addr: SocketAddr = peer_address.parse()
            .map_err(|_| "آدرس نامعتبر".to_string())?;

        println!("🔗 تلاش برای اتصال به: {}", addr);

        match TcpStream::connect(addr).await {
            Ok(mut stream) => {
                // ارسال پیام Handshake
                let blockchain_height = {
                    let blockchain = self.blockchain.lock().await;
                    blockchain.get_chain_length() as u64
                };

                let handshake = NetworkMessage::Handshake {
                    node_id: self.node_id.clone(),
                    version: "1.0".to_string(),
                    blockchain_height,
                };

                let message_json = serde_json::to_string(&handshake)
                    .map_err(|_| "خطا در سریال کردن پیام".to_string())?;

                stream.write_all(message_json.as_bytes()).await
                    .map_err(|_| "خطا در ارسال پیام".to_string())?;

                println!("✅ اتصال به {} برقرار شد", addr);
                Ok(())
            }
            Err(e) => {
                Err(format!("خطا در اتصال: {}", e))
            }
        }
    }

    async fn handle_connection(
        mut stream: TcpStream,
        addr: SocketAddr,
        peers: Arc<Mutex<HashMap<String, Peer>>>,
        blockchain: Arc<Mutex<Blockchain>>,
        node_id: String,
    ) {
        let mut buffer = vec![0; 4096];
        
        loop {
            match stream.read(&mut buffer).await {
                Ok(0) => {
                    println!("🔌 اتصال {} قطع شد", addr);
                    break;
                }
                Ok(n) => {
                    let data = String::from_utf8_lossy(&buffer[..n]);
                    
                    match serde_json::from_str::<NetworkMessage>(&data) {
                        Ok(message) => {
                            Self::handle_message(
                                message,
                                &mut stream,
                                addr,
                                peers.clone(),
                                blockchain.clone(),
                                node_id.clone(),
                            ).await;
                        }
                        Err(e) => {
                            println!("❌ خطا در پارس پیام: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("❌ خطا در خواندن داده: {}", e);
                    break;
                }
            }
        }
    }

    async fn handle_message(
        message: NetworkMessage,
        stream: &mut TcpStream,
        addr: SocketAddr,
        peers: Arc<Mutex<HashMap<String, Peer>>>,
        blockchain: Arc<Mutex<Blockchain>>,
        node_id: String,
    ) {
        match message {
            NetworkMessage::Handshake { node_id: peer_id, version: _, blockchain_height } => {
                println!("🤝 دریافت Handshake از {}: {}", addr, peer_id);
                
                // اضافه کردن peer جدید
                let peer = Peer {
                    id: peer_id.clone(),
                    address: addr,
                    last_seen: chrono::Utc::now(),
                    blockchain_height,
                };
                
                peers.lock().await.insert(peer_id, peer);
                
                // ارسال پاسخ
                let response = NetworkMessage::HandshakeResponse {
                    node_id,
                    accepted: true,
                };
                
                if let Ok(response_json) = serde_json::to_string(&response) {
                    let _ = stream.write_all(response_json.as_bytes()).await;
                }
            }
            
            NetworkMessage::NewTransaction(transaction) => {
                println!("📨 دریافت تراکنش جدید: {}", transaction.id);
                
                // اضافه کردن تراکنش به blockchain
                let mut blockchain = blockchain.lock().await;
                if let Err(e) = blockchain.add_transaction(transaction) {
                    println!("❌ خطا در اضافه کردن تراکنش: {}", e);
                }
            }
            
            NetworkMessage::NewBlock(block) => {
                println!("📦 دریافت بلاک جدید: #{}", block.index);
                
                // بررسی و اضافه کردن بلاک (پیاده‌سازی کامل نیاز به منطق همگام‌سازی دارد)
                // فعلاً فقط لاگ می‌کنیم
            }
            
            NetworkMessage::Ping => {
                println!("🏓 دریافت Ping از {}", addr);
                
                let pong = NetworkMessage::Pong;
                if let Ok(pong_json) = serde_json::to_string(&pong) {
                    let _ = stream.write_all(pong_json.as_bytes()).await;
                }
            }
            
            NetworkMessage::GetPeers => {
                println!("👥 درخواست لیست peers از {}", addr);
                
                let peers_list: Vec<String> = peers.lock().await
                    .values()
                    .map(|p| p.address.to_string())
                    .collect();
                
                let response = NetworkMessage::PeersResponse {
                    peers: peers_list,
                };
                
                if let Ok(response_json) = serde_json::to_string(&response) {
                    let _ = stream.write_all(response_json.as_bytes()).await;
                }
            }
            
            _ => {
                println!("📨 دریافت پیام: {:?}", message);
            }
        }
    }

    pub async fn broadcast_transaction(&self, transaction: &Transaction) -> Result<(), String> {
        let message = NetworkMessage::NewTransaction(transaction.clone());
        self.broadcast_message(message).await
    }

    pub async fn broadcast_block(&self, block: &Block) -> Result<(), String> {
        let message = NetworkMessage::NewBlock(block.clone());
        self.broadcast_message(message).await
    }

    async fn broadcast_message(&self, message: NetworkMessage) -> Result<(), String> {
        let peers = self.peers.lock().await;
        let message_json = serde_json::to_string(&message)
            .map_err(|_| "خطا در سریال کردن پیام".to_string())?;

        for peer in peers.values() {
            if let Ok(mut stream) = TcpStream::connect(peer.address).await {
                let _ = stream.write_all(message_json.as_bytes()).await;
                println!("📡 پیام به {} ارسال شد", peer.id);
            }
        }

        Ok(())
    }

    pub async fn get_peers_count(&self) -> usize {
        self.peers.lock().await.len()
    }

    pub async fn get_network_stats(&self) -> NetworkStats {
        let peers = self.peers.lock().await;
        let blockchain = self.blockchain.lock().await;
        
        NetworkStats {
            node_id: self.node_id.clone(),
            listen_address: self.listen_address,
            connected_peers: peers.len(),
            blockchain_height: blockchain.get_chain_length() as u64,
            is_running: *self.is_running.lock().await,
        }
    }
}

#[derive(Debug)]
pub struct NetworkStats {
    pub node_id: String,
    pub listen_address: SocketAddr,
    pub connected_peers: usize,
    pub blockchain_height: u64,
    pub is_running: bool,
}

impl std::fmt::Display for NetworkStats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "🌐 آمار شبکه")?;
        writeln!(f, "==============")?;
        writeln!(f, "🆔 شناسه نود: {}", self.node_id)?;
        writeln!(f, "📍 آدرس: {}", self.listen_address)?;
        writeln!(f, "👥 peers متصل: {}", self.connected_peers)?;
        writeln!(f, "📏 ارتفاع بلاک چین: {}", self.blockchain_height)?;
        writeln!(f, "🔄 وضعیت: {}", if self.is_running { "فعال" } else { "غیرفعال" })?;
        Ok(())
    }
}