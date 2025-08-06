# 🪙 TenCoin (TEC) - Digital Currency

A complete implementation of blockchain and digital currency with Bitcoin-like halving algorithm, built with Rust.

## ✨ Key Features

- 🪙 **نام:** TenCoin (TEC)
- 💎 **حداکثر عرضه:** 10,000,000 TEC
- 🏆 **پاداش اولیه:** 50 TEC per block
- 📉 **هاوینگ:** هر 100,000 بلاک
- ⚙️ **سختی:** 5 صفر (00000...)
- 🔗 **آدرس‌دهی:** tc1q (bech32) و tc3 (p2sh)
- 🌐 **API RESTful:** کامل برای تعامل
- ⛏️ **ماینر Python:** برای استخراج بلاک‌ها

## 🏗️ معماری سیستم

### 🦀 **Rust Node**
- **Blockchain Core:** مدیریت زنجیره بلاک‌ها
- **Transaction Pool:** مدیریت تراکنش‌های در انتظار
- **API Server:** وب سرور برای درخواست‌ها
- **Address Generation:** تولید آدرس‌های مختلف

### 🐍 **Python Miner**
- **اتصال شبکه:** به نود اصلی
- **Proof of Work:** استخراج با SHA-256
- **Block Validation:** اعتبارسنجی بلاک‌ها

## 📊 مشخصات اقتصادی

### 💰 **توزیع توکن‌ها**
```
کل عرضه: 10,000,000 TEC
پاداش اولیه: 50 TEC
هاوینگ 1 (100K): 25 TEC
هاوینگ 2 (200K): 12.5 TEC
هاوینگ 3 (300K): 6.25 TEC
...
هاوینگ 8 (800K): 0.195 TEC ≈ 0
```

### 📈 **جدول زمانی هاوینگ**
| هاوینگ | بلاک | پاداش | تخمین زمان |
|---------|------|--------|------------|
| 0 | 0-99,999 | 50 TEC | شروع |
| 1 | 100,000 | 25 TEC | ~6 ماه |
| 2 | 200,000 | 12.5 TEC | ~1 سال |
| 3 | 300,000 | 6.25 TEC | ~1.5 سال |
| ... | ... | ... | ... |
| 8 | 800,000 | ~0 TEC | تمام |

## 🛠️ نصب و راه‌اندازی

### پیش‌نیازها

```bash
# نصب Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# نصب Python (برای ماینر)
sudo apt install python3 python3-pip
pip3 install requests
```

### اجرای نود

```bash
# کلون پروژه
git clone <repository-url>
cd tencoin

# اجرای نود
cargo run
```

### اجرای ماینر

```bash
# ماینر محلی
python3 miner.py

# ماینر با آدرس سفارشی
python3 miner.py 192.168.1.100 8333
```

## 🌐 API Endpoints

### 📊 **اطلاعات بلاک چین**
```bash
GET /blockchain/info
```

**پاسخ:**
```json
{
  "height": 1500,
  "total_supply": 75000.0,
  "max_supply": 10000000.0,
  "current_reward": 50.0,
  "difficulty": 5,
  "next_halving": 100000,
  "halving_countdown": 98500
}
```

### 👛 **ایجاد کیف پول**

**Bech32 (tc1q):**
```bash
GET /wallet/generate?type=bech32
```

**Multisig P2SH (tc3):**
```bash
GET /wallet/generate?type=p2sh&required=2&total=4
```

**پاسخ:**
```json
{
  "success": true,
  "wallet": {
    "address": "tc1q1a2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p7q8r9s0",
    "address_type": "Bech32",
    "private_keys": ["abc123..."],
    "public_keys": ["def456..."],
    "script": null
  }
}
```

### 💸 **ارسال تراکنش**
```bash
POST /transaction/send
```

**درخواست:**
```json
{
  "from_address": "tc1q...",
  "to_address": "tc1q...",
  "amount": 10.5,
  "fee": 0.001,
  "inputs": [],
  "outputs": []
}
```

### 💰 **موجودی**
```bash
GET /balance/tc1q1a2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p7q8r9s0
```

### ⛏️ **آمار استخراج**
```bash
GET /mining/stats
```

## 🔧 نمونه استفاده

### 1️⃣ **ایجاد کیف پول**
```bash
curl "http://localhost:8333/wallet/generate?type=bech32"
```

### 2️⃣ **بررسی اطلاعات شبکه**
```bash
curl "http://localhost:8333/blockchain/info"
```

### 3️⃣ **شروع ماینر**
```bash
python3 miner.py localhost 8333
```

### 4️⃣ **ارسال تراکنش**
```bash
curl -X POST http://localhost:8333/transaction/send \
  -H "Content-Type: application/json" \
  -d '{
    "from_address": "tc1q...",
    "to_address": "tc1q...",
    "amount": 5.0,
    "fee": 0.001
  }'
```

## 🔐 امنیت

### 🛡️ **رمزنگاری**
- **Hash Algorithm:** SHA-256
- **Digital Signature:** ECDSA secp256k1
- **Address Format:** Bech32 و P2SH
- **Proof of Work:** 5 leading zeros

### 🔒 **اعتبارسنجی**
- **Block Validation:** Hash، Merkle Root، Signatures
- **Transaction Validation:** Balance، Signature، Format
- **Chain Validation:** Previous Hash، Index Order

## 📊 نمونه خروجی

```
🪙 TenCoin (TEC) - ارز دیجیتال ایرانی 🪙
=========================================
💎 کل عرضه: 10,000,000 TEC
🏆 پاداش اولیه: 50 TEC
📉 هاوینگ: هر 100,000 بلاک
⚙️ سختی: 5 صفر
=========================================
🌟 ایجاد بلاک Genesis...
⛏️  شروع استخراج بلاک 0 با سختی 2...
✅ بلاک 0 استخراج شد! Nonce: 324, زمان: 0.00s
✅ بلاک Genesis ایجاد شد!
💎 کل عرضه فعلی: 100 TEC

🌐 API Server در حال اجرا در: http://0.0.0.0:8333
📡 Network Node در حال اجرا در: 0.0.0.0:8334

🔗 نمونه درخواست‌های API:
   GET  /blockchain/info - اطلاعات بلاک چین
   POST /wallet/generate - ایجاد کیف پول جدید
   POST /transaction/send - ارسال تراکنش
   GET  /mining/stats - آمار استخراج

🛠️ برای ماینر، فایل miner.py را اجرا کنید:
   python3 miner.py
```

## 🧪 تست

```bash
# تست نود
cargo test

# تست API
curl http://localhost:8333/blockchain/info

# تست ماینر
python3 miner.py localhost 8333
```

## 🚀 ویژگی‌های پیشرفته

### 🔄 **هاوینگ خودکار**
- هر 100,000 بلاک پاداش نصف می‌شود
- پس از 8 هاوینگ، فقط fees باقی می‌ماند
- مشابه مکانیزم بیتکوین

### 🏠 **آدرس‌های مختلف**
- **tc1q...**: Bech32 (SegWit native)
- **tc3...**: P2SH (Multi-signature)

### ⚡ **API کامل**
- RESTful endpoints
- JSON responses
- CORS enabled
- Error handling

## 📋 مقایسه با سایر ارزها

| ویژگی | TenCoin | Bitcoin | Litecoin |
|--------|---------|---------|----------|
| حداکثر عرضه | 10M | 21M | 84M |
| پاداش اولیه | 50 | 50 | 50 |
| هاوینگ | 100K blocks | 210K blocks | 840K blocks |
| سختی | 5 zeros | متغیر | متغیر |
| زبان | Rust | C++ | C++ |
| آدرس | tc1q/tc3 | bc1/3 | ltc1/M |

## 🤝 مشارکت

1. Fork کنید
2. Feature branch ایجاد کنید
3. تغییرات را commit کنید
4. Pull Request ارسال کنید

## 📜 مجوز

MIT License

## 👨‍💻 توسعه‌دهندگان

ساخته شده با ❤️ و Rust

---

**⚠️ هشدار:** این پروژه برای اهداف آموزشی و تست طراحی شده است. برای استفاده تولیدی نیاز به بهینه‌سازی‌های امنیتی بیشتری دارد.