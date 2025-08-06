use std::io::{self, Write};
use crate::blockchain::Blockchain;
use crate::wallet::Wallet;
use crate::transaction::Transaction;

pub struct CLI {
    blockchain: Blockchain,
    wallets: Vec<Wallet>,
    current_wallet_index: usize,
}

impl CLI {
    pub fn new(blockchain: Blockchain, wallets: Vec<Wallet>) -> Self {
        CLI {
            blockchain,
            wallets,
            current_wallet_index: 0,
        }
    }

    pub async fn run(&mut self) {
        loop {
            self.show_menu();
            
            print!("انتخاب شما: ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            match input.trim() {
                "1" => self.show_blockchain_info(),
                "2" => self.show_wallet_info(),
                "3" => self.switch_wallet(),
                "4" => self.create_transaction().await,
                "5" => self.mine_block().await,
                "6" => self.show_transaction_history(),
                "7" => self.validate_blockchain(),
                "8" => self.create_new_wallet(),
                "9" => self.export_blockchain(),
                "0" => {
                    println!("👋 خداحافظ!");
                    break;
                }
                _ => println!("❌ انتخاب نامعتبر!"),
            }
            
            println!("\nبرای Continue Enter را فشار دهید...");
            let mut _temp = String::new();
            io::stdin().read_line(&mut _temp).unwrap();
        }
    }

    fn show_menu(&self) {
        println!("\n{}", "=".repeat(50));
        println!("🪙  منوی RustCoin");
        println!("{}", "=".repeat(50));
        println!("1️⃣  نمایش اطلاعات blockchain");
        println!("2️⃣  نمایش اطلاعات wallet فعلی");
        println!("3️⃣  تغییر wallet فعلی");
        println!("4️⃣  Creating transaction جدید");
        println!("5️⃣  mining block");
        println!("6️⃣  نمایش تاریخچه transaction‌ها");
        println!("7️⃣  اعتبارسنجی blockchain");
        println!("8️⃣  Creating wallet جدید");
        println!("9️⃣  صادر کRejectن blockchain");
        println!("0️⃣  خروج");
        println!("{}", "=".repeat(50));
        println!("wallet فعلی: {} (شماره {})", 
                 &self.wallets[self.current_wallet_index].get_address()[..16], 
                 self.current_wallet_index + 1);
    }

    fn show_blockchain_info(&self) {
        println!("\n📊 اطلاعات blockchain:");
        println!("{}", "=".repeat(40));
        let info = self.blockchain.get_chain_info();
        println!("📦 تعداد block‌ها: {}", info.total_blocks);
        println!("💸 تعداد transaction‌ها: {}", info.total_transactions);
        println!("💎 کل عرضه: {} RustCoin", info.total_supply);
        println!("⚙️  difficulty: {}", info.difficulty);
        println!("🏆 reward mining: {} RustCoin", info.mining_reward);
        println!("⏳ transaction‌های pending: {}", info.pending_transactions);
    }

    fn show_wallet_info(&self) {
        let wallet = &self.wallets[self.current_wallet_index];
        println!("\n💼 اطلاعات wallet:");
        println!("{}", "=".repeat(40));
        println!("📍 address کامل: {}", wallet.get_address());
        println!("💰 balance: {} RustCoin", self.blockchain.get_balance(&wallet.get_address()));
        
        // نمایش transaction‌های مربوط به این wallet
        let transactions = self.blockchain.get_transactions_for_address(&wallet.get_address());
        println!("📝 تعداد transaction‌ها: {}", transactions.len());
    }

    fn switch_wallet(&mut self) {
        println!("\n👛 wallet‌های موجود:");
        for (i, wallet) in self.wallets.iter().enumerate() {
            let balance = self.blockchain.get_balance(&wallet.get_address());
            let marker = if i == self.current_wallet_index { "👉" } else { "  " };
            println!("{} {}. {} | balance: {} RustCoin", 
                     marker, i + 1, &wallet.get_address()[..16], balance);
        }
        
        print!("شماره wallet موReject نظر (1-{}): ", self.wallets.len());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if let Ok(index) = input.trim().parse::<usize>() {
            if index > 0 && index <= self.wallets.len() {
                self.current_wallet_index = index - 1;
                println!("✅ wallet {} انتخاب شد", index);
            } else {
                println!("❌ شماره نامعتبر!");
            }
        } else {
            println!("❌ ورودی نامعتبر!");
        }
    }

    async fn create_transaction(&mut self) {
        let current_wallet = &self.wallets[self.current_wallet_index];
        let balance = self.blockchain.get_balance(&current_wallet.get_address());
        
        println!("\n💸 Creating transaction جدید");
        println!("balance فعلی: {} RustCoin", balance);
        
        // انتخاب گیرنده
        println!("\nگیرنده را انتخاب کنید:");
        println!("1. واReject کRejectن address دستی");
        println!("2. انتخاب از wallet‌های موجود");
        
        print!("انتخاب: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let to_address = match input.trim() {
            "1" => {
                print!("address گیرنده: ");
                io::stdout().flush().unwrap();
                let mut addr = String::new();
                io::stdin().read_line(&mut addr).unwrap();
                addr.trim().to_string()
            }
            "2" => {
                println!("wallet‌های موجود:");
                for (i, wallet) in self.wallets.iter().enumerate() {
                    if i != self.current_wallet_index {
                        println!("{}. {}", i + 1, &wallet.get_address()[..20]);
                    }
                }
                
                print!("شماره wallet: ");
                io::stdout().flush().unwrap();
                let mut wallet_input = String::new();
                io::stdin().read_line(&mut wallet_input).unwrap();
                
                if let Ok(index) = wallet_input.trim().parse::<usize>() {
                    if index > 0 && index <= self.wallets.len() && index - 1 != self.current_wallet_index {
                        self.wallets[index - 1].get_address()
                    } else {
                        println!("❌ انتخاب نامعتبر!");
                        return;
                    }
                } else {
                    println!("❌ ورودی نامعتبر!");
                    return;
                }
            }
            _ => {
                println!("❌ انتخاب نامعتبر!");
                return;
            }
        };
        
        // مقدار transaction
        print!("مقدار (حداکثر {}): ", balance);
        io::stdout().flush().unwrap();
        
        let mut amount_input = String::new();
        io::stdin().read_line(&mut amount_input).unwrap();
        
        if let Ok(amount) = amount_input.trim().parse::<f64>() {
            if amount > 0.0 && amount <= balance {
                // Create transaction
                match Transaction::new(
                    current_wallet.get_address(),
                    to_address.clone(),
                    amount,
                    current_wallet,
                ) {
                    Ok(transaction) => {
                        match self.blockchain.add_transaction(transaction) {
                            Ok(_) => {
                                println!("✅ transaction successfully created!");
                                println!("📝 transaction به صف انتظار added");
                            }
                            Err(e) => println!("❌ Error in اضافه کRejectن transaction: {}", e),
                        }
                    }
                    Err(e) => println!("❌ Error in Creating transaction: {}", e),
                }
            } else {
                println!("❌ مقدار نامعتبر!");
            }
        } else {
            println!("❌ ورودی نامعتبر!");
        }
    }

    async fn mine_block(&mut self) {
        if self.blockchain.get_pending_transactions_count() == 0 {
            println!("❌ هیچ transaction pendingی وجود نداReject!");
            return;
        }
        
        let current_wallet = &self.wallets[self.current_wallet_index];
        
        println!("⛏️  Start mining...");
        
        match self.blockchain.mine_pending_transactions(&current_wallet.get_address()).await {
            Ok(_) => {
                println!("✅ block successfully mined!");
                let balance = self.blockchain.get_balance(&current_wallet.get_address());
                println!("💰 balance جدید: {} RustCoin", balance);
            }
            Err(e) => println!("❌ Error in mining: {}", e),
        }
    }

    fn show_transaction_history(&self) {
        let current_wallet = &self.wallets[self.current_wallet_index];
        let transactions = self.blockchain.get_transactions_for_address(&current_wallet.get_address());
        
        println!("\n📜 تاریخچه transaction‌ها:");
        println!("{}", "=".repeat(50));
        
        if transactions.is_empty() {
            println!("هیچ transactionی not found.");
        } else {
            for (i, tx) in transactions.iter().enumerate() {
                println!("{}. {}", i + 1, tx);
            }
        }
    }

    fn validate_blockchain(&self) {
        println!("\n🔍 اعتبارسنجی blockchain...");
        
        if self.blockchain.is_chain_valid() {
            println!("✅ blockchain معتبر است!");
        } else {
            println!("❌ blockchain is invalid!");
        }
    }

    fn create_new_wallet(&mut self) {
        let new_wallet = Wallet::new();
        println!("\n👛 wallet جدید created!");
        println!("📍 address: {}", new_wallet.get_address());
        
        self.wallets.push(new_wallet);
        println!("✅ wallet به لیست added (شماره {})", self.wallets.len());
    }

    fn export_blockchain(&self) {
        match self.blockchain.export_json() {
            Ok(json) => {
                println!("\n📤 صادر کRejectن blockchain:");
                println!("{}", "=".repeat(40));
                println!("{}", &json[..500.min(json.len())]);
                if json.len() > 500 {
                    println!("... (کل {} کاراکتر)", json.len());
                }
            }
            Err(e) => println!("❌ Error in صادر کRejectن: {}", e),
        }
    }
}