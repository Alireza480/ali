#!/usr/bin/env python3
"""
TenCoin Miner - miner Python برای TenCoin
اتصال به network TenCoin و mining block‌ها
"""

import socket
import json
import time
import hashlib
import requests
import threading
from datetime import datetime
import sys

class TenCoinMiner:
    def __init__(self, node_host="172.23.120.96", node_port=8333, miner_address=None):
        self.node_host = node_host
        self.node_port = node_port
        self.miner_address = miner_address or self.generate_miner_address()
        self.is_mining = False
        self.current_block = None
        self.blockchain_height = 0
        
        print(f"🪙 TenCoin Miner Start شد")
        print(f"🌐 اتصال به: {node_host}:{node_port}")
        print(f"⛏️  address miner: {self.miner_address}")
        print("=" * 50)

    def generate_miner_address(self):
        """تولید address miner"""
        import random
        import string
        # تولید address tc1q ساده برای miner
        random_part = ''.join(random.choices(string.ascii_lowercase + string.digits, k=32))
        return f"tc1q{random_part}"

    def connect_to_node(self):
        """اتصال به node TenCoin"""
        try:
            response = requests.get(f"http://{self.node_host}:{self.node_port}/blockchain/info", timeout=5)
            if response.status_code == 200:
                info = response.json()
                self.blockchain_height = info.get('height', 0)
                print(f"✅ اتصال برقرار شد! ارتفاع blockchain: {self.blockchain_height}")
                return True
            else:
                print(f"❌ Error in اتصال: HTTP {response.status_code}")
                return False
        except requests.exceptions.RequestException as e:
            print(f"❌ Error in اتصال: {e}")
            return False

    def get_mining_stats(self):
        """دریافت آمار mining"""
        try:
            response = requests.get(f"http://{self.node_host}:{self.node_port}/mining/stats", timeout=5)
            if response.status_code == 200:
                return response.json()
            return None
        except:
            return None

    def get_blockchain_info(self):
        """دریافت اطلاعات blockchain"""
        try:
            response = requests.get(f"http://{self.node_host}:{self.node_port}/blockchain/info", timeout=5)
            if response.status_code == 200:
                return response.json()
            return None
        except:
            return None

    def create_coinbase_transaction(self, reward):
        """ایجاد transaction coinbase برای reward miner"""
        return {
            "id": f"coinbase_{int(time.time())}",
            "from_address": "",  # از سیستم
            "to_address": self.miner_address,
            "amount": reward,
            "timestamp": datetime.utcnow().isoformat() + "Z",
            "signature": None
        }

    def calculate_merkle_root(self, transactions):
        """محاسبه Merkle Root"""
        if not transactions:
            return ""
        
        # تبدیل transaction‌ها به hash
        hashes = []
        for tx in transactions:
            tx_str = json.dumps(tx, sort_keys=True)
            tx_hash = hashlib.sha256(tx_str.encode()).hexdigest()
            hashes.append(tx_hash)
        
        # ساخت درخت مرکل
        while len(hashes) > 1:
            new_hashes = []
            for i in range(0, len(hashes), 2):
                if i + 1 < len(hashes):
                    combined = hashes[i] + hashes[i + 1]
                else:
                    combined = hashes[i] + hashes[i]  # تکرار آخرین hash
                
                new_hash = hashlib.sha256(combined.encode()).hexdigest()
                new_hashes.append(new_hash)
            
            hashes = new_hashes
        
        return hashes[0] if hashes else ""

    def create_block_template(self, transactions, previous_hash):
        """ایجاد قالب block برای mining"""
        block = {
            "index": self.blockchain_height + 1,
            "timestamp": datetime.utcnow().isoformat() + "Z",
            "transactions": transactions,
            "previous_hash": previous_hash,
            "hash": "",
            "nonce": 0,
            "merkle_root": self.calculate_merkle_root(transactions)
        }
        return block

    def calculate_block_hash(self, block):
        """محاسبه hash block"""
        block_str = f"{block['index']}{block['timestamp']}{block['previous_hash']}{block['merkle_root']}{block['nonce']}{json.dumps(block['transactions'], sort_keys=True)}"
        return hashlib.sha256(block_str.encode()).hexdigest()

    def mine_block(self, block, difficulty=5):
        """mining block با Proof of Work"""
        target = "0" * difficulty
        start_time = time.time()
        
        print(f"⛏️  Start mining block #{block['index']} با difficulty {difficulty}...")
        
        while True:
            block['hash'] = self.calculate_block_hash(block)
            
            if block['hash'].startswith(target):
                duration = time.time() - start_time
                print(f"✅ block #{block['index']} mined!")
                print(f"   Nonce: {block['nonce']}")
                print(f"   زمان: {duration:.2f}s")
                print(f"   Hash: {block['hash'][:16]}...")
                return block
            
            block['nonce'] += 1
            
            # نمایش پیشرفت
            if block['nonce'] % 50000 == 0:
                print(f"🔄 تلاش {block['nonce']}: {block['hash'][:16]}...")
            
            # بررسی Stop
            if not self.is_mining:
                return None

    def submit_block(self, block):
        """ارسال block minedه به network"""
        try:
            # در اینجا باید block را به API ارسال کنیم
            # فعلاً فقط لاگ می‌کنیم
            print(f"📤 ارسال block #{block['index']} به network...")
            print(f"   transaction‌ها: {len(block['transactions'])}")
            print(f"   Hash: {block['hash']}")
            return True
        except Exception as e:
            print(f"❌ Error in ارسال block: {e}")
            return False

    def mining_loop(self):
        """حلقه اصلی mining"""
        print("🚀 Start حلقه mining...")
        
        while self.is_mining:
            try:
                # دریافت اطلاعات فعلی
                blockchain_info = self.get_blockchain_info()
                mining_stats = self.get_mining_stats()
                
                if not blockchain_info or not mining_stats:
                    print("⚠️  نمی‌توان اطلاعات network را دریافت کReject")
                    time.sleep(10)
                    continue
                
                current_reward = mining_stats.get('current_reward', 0)
                if current_reward <= 0:
                    print("ℹ️  reward mining به صفر رسیده - فقط از fees درآمد کسب می‌شود")
                    time.sleep(30)
                    continue
                
                # ایجاد transaction coinbase
                coinbase_tx = self.create_coinbase_transaction(current_reward)
                transactions = [coinbase_tx]
                
                # دریافت آخرین hash
                previous_hash = "0" * 64  # ساده‌سازی شده
                
                # ایجاد block
                block_template = self.create_block_template(transactions, previous_hash)
                
                # mining block
                mined_block = self.mine_block(block_template)
                
                if mined_block and self.is_mining:
                    # ارسال block
                    if self.submit_block(mined_block):
                        self.blockchain_height += 1
                        print(f"🎉 block #{mined_block['index']} successfully mining و sent!")
                    else:
                        print("❌ Error in ارسال block")
                
                # استراحت کوتاه
                time.sleep(1)
                
            except KeyboardInterrupt:
                print("\n🛑 دریافت سیگنال Stop...")
                self.stop_mining()
                break
            except Exception as e:
                print(f"❌ Error in حلقه mining: {e}")
                time.sleep(5)

    def start_mining(self):
        """Start mining"""
        if self.is_mining:
            print("⚠️  mining در حال حاضر active است")
            return
        
        if not self.connect_to_node():
            print("❌ نمی‌توان به node متصل شد")
            return
        
        self.is_mining = True
        
        # Start thread mining
        mining_thread = threading.Thread(target=self.mining_loop)
        mining_thread.daemon = True
        mining_thread.start()
        
        print("✅ mining Start شد!")
        
        # نمایش آمار هر 30 ثانیه
        try:
            while self.is_mining:
                time.sleep(30)
                stats = self.get_mining_stats()
                if stats:
                    print(f"📊 آمار: reward فعلی: {stats.get('current_reward', 0)} TEC, "
                          f"باقی‌مانده: {stats.get('remaining_supply', 0)} TEC")
        except KeyboardInterrupt:
            self.stop_mining()

    def stop_mining(self):
        """Stop mining"""
        self.is_mining = False
        print("🛑 mining مStop شد")

def main():
    """تابع اصلی"""
    print("🪙 TenCoin Miner v1.0")
    print("=" * 30)
    
    # تنظیمات پیش‌فرض
    node_host = "172.23.120.96"  # address node
    node_port = 8333
    
    # بررسی آرگومان‌های خط فرمان
    if len(sys.argv) > 1:
        node_host = sys.argv[1]
    if len(sys.argv) > 2:
        node_port = int(sys.argv[2])
    
    # ایجاد miner
    miner = TenCoinMiner(node_host, node_port)
    
    try:
        # Start mining
        miner.start_mining()
    except KeyboardInterrupt:
        print("\n👋 خداحافظ!")
    except Exception as e:
        print(f"❌ خطای غیرمنتظره: {e}")
    finally:
        miner.stop_mining()

if __name__ == "__main__":
    main()