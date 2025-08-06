# 🪙 TenCoin (TEC) - Project Summary

## 🎯 Project Overview

TenCoin is a complete digital currency implementation built with Rust, featuring a Bitcoin-like halving mechanism and modern blockchain architecture.

## 📊 Technical Specifications

### 🪙 **Currency Details**
- **Name:** TenCoin (TEC)
- **Symbol:** TEC
- **Max Supply:** 10,000,000 TEC
- **Initial Reward:** 50 TEC per block
- **Halving Interval:** Every 100,000 blocks
- **Mining Difficulty:** 5 leading zeros
- **Address Prefixes:** tc1q (bech32), tc3 (p2sh)

### 🏗️ **Architecture**

#### **Rust Node (Core)**
- **Blockchain Management:** Complete blockchain implementation
- **Transaction Pool:** Pending transaction management
- **Mining System:** Proof-of-Work with adjustable difficulty
- **Address Generation:** Bech32 and P2SH address support
- **RESTful API:** Full HTTP API for interaction
- **Halving System:** Automatic reward halving every 100,000 blocks

#### **Python Miner**
- **Network Connection:** Connects to Rust node at configurable IP:port
- **Block Mining:** SHA-256 based Proof-of-Work
- **Real-time Stats:** Mining statistics and network monitoring
- **Auto-discovery:** Automatic blockchain height detection

## 🔧 Key Components

### 📦 **Core Modules**
1. **main.rs** - Application entry point and server startup
2. **blockchain.rs** - Core blockchain logic with halving system
3. **block.rs** - Block structure and mining implementation
4. **transaction.rs** - Transaction handling and validation
5. **address.rs** - Address generation (bech32/p2sh) and wallet management
6. **api.rs** - RESTful API server with warp framework
7. **network.rs** - P2P networking foundation
8. **mining.rs** - Mining pool and miner management

### 🌐 **API Endpoints**
```
GET  /blockchain/info     - Blockchain information
POST /wallet/generate     - Generate new wallet
GET  /balance/{address}   - Check address balance
POST /transaction/send    - Send transaction
GET  /mining/stats        - Mining statistics
```

### 🐍 **Python Miner Features**
- Connects to network node (default: 172.23.120.96:8333)
- Real-time mining with progress indicators
- Automatic reward calculation based on current height
- Network statistics monitoring
- Graceful shutdown handling

## 💰 Economic Model

### 📈 **Halving Schedule**
| Halving | Block Height | Reward | Est. Time |
|---------|--------------|--------|-----------|
| 0 | 0-99,999 | 50 TEC | Start |
| 1 | 100,000 | 25 TEC | ~6 months |
| 2 | 200,000 | 12.5 TEC | ~1 year |
| 3 | 300,000 | 6.25 TEC | ~1.5 years |
| ... | ... | ... | ... |
| 8 | 800,000 | ~0 TEC | End |

### 💎 **Supply Distribution**
- **Total Supply:** 10,000,000 TEC (fixed cap)
- **Mining Rewards:** Decreases by 50% every 100,000 blocks
- **Transaction Fees:** Miners receive fees after all coins are mined
- **No Premine:** Fair distribution through mining only

## 🔐 Security Features

### 🛡️ **Cryptography**
- **Hash Algorithm:** SHA-256 (same as Bitcoin)
- **Digital Signatures:** ECDSA with secp256k1 curve
- **Address Format:** Bech32 (native SegWit) and P2SH (multisig)
- **Proof of Work:** 5 leading zeros difficulty

### ✅ **Validation**
- **Block Validation:** Hash verification, Merkle root, signatures
- **Transaction Validation:** Balance checks, signature verification
- **Chain Validation:** Previous hash links, sequential block heights
- **Network Consensus:** Longest valid chain rule

## 🚀 Usage Examples

### 1. **Start TenCoin Node**
```bash
cargo run
```

### 2. **Generate Wallet**
```bash
# Bech32 wallet
curl "http://localhost:8333/wallet/generate?type=bech32"

# Multisig wallet (2-of-4)
curl "http://localhost:8333/wallet/generate?type=p2sh&required=2&total=4"
```

### 3. **Check Blockchain Info**
```bash
curl "http://localhost:8333/blockchain/info"
```

### 4. **Start Mining**
```bash
# Local mining
python3 miner.py

# Remote mining
python3 miner.py 192.168.1.100 8333
```

### 5. **Send Transaction**
```bash
curl -X POST http://localhost:8333/transaction/send \
  -H "Content-Type: application/json" \
  -d '{
    "from_address": "tc1q...",
    "to_address": "tc1q...",
    "amount": 10.0,
    "fee": 0.001
  }'
```

## 📊 Sample Output

```
🪙 TenCoin (TEC) - Digital Currency 🪙
=======================================
💎 Total Supply: 10,000,000 TEC
🏆 Initial Reward: 50 TEC
📉 Halving: Every 100,000 blocks
⚙️ Difficulty: 5 zeros
=======================================
🌟 Creating Genesis block...
⛏️  Start mining block 0 with difficulty 2...
✅ Block 0 mined! Nonce: 324, Time: 0.00s
✅ Genesis block created!
💎 Current total supply: 100 TEC

🌐 API Server running at: http://0.0.0.0:8333
📡 Network Node running at: 0.0.0.0:8334
```

## 🎯 Project Achievements

### ✅ **Completed Features**
- [x] Complete blockchain implementation
- [x] Halving system (every 100,000 blocks)
- [x] Mining difficulty: 5 zeros
- [x] Address generation (bech32/p2sh with tc1q/tc3 prefixes)
- [x] RESTful API server
- [x] Python miner with network connectivity
- [x] Transaction fee system
- [x] All text converted to English
- [x] Comprehensive documentation

### 🏆 **Technical Highlights**
- **Performance:** Efficient Rust implementation
- **Security:** Industry-standard cryptography
- **Compatibility:** Bitcoin-like address formats
- **Scalability:** Modular architecture
- **Usability:** Simple API and miner setup
- **Documentation:** Complete English documentation

## 🛠️ Development Setup

### **Requirements**
- Rust 1.82+ (for node)
- Python 3.8+ with `requests` (for miner)
- 4GB RAM minimum
- Network connectivity for P2P

### **Quick Start**
```bash
# Clone and build
git clone <repository>
cd tencoin
cargo build --release

# Run node
cargo run

# Run miner (separate terminal)
python3 miner.py
```

## 📈 Future Enhancements

### 🔮 **Roadmap**
- [ ] Web-based wallet interface
- [ ] Mobile mining app
- [ ] Advanced P2P networking
- [ ] Smart contracts support
- [ ] Cross-chain bridges
- [ ] Decentralized exchange integration

## 🏁 Conclusion

TenCoin successfully demonstrates a complete digital currency implementation with:
- ✅ **Bitcoin-like economics** with halving mechanism
- ✅ **Modern architecture** using Rust and Python
- ✅ **Professional API** for easy integration
- ✅ **Security-first design** with proven cryptography
- ✅ **Developer-friendly** setup and documentation

The project serves as both a functional cryptocurrency and an educational resource for blockchain development.

---

**Built with ❤️ using Rust & Python**