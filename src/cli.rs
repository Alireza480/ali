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
            
            println!("\nبرای ادامه Enter را فشار دهید...");
            let mut _temp = String::new();
            io::stdin().read_line(&mut _temp).unwrap();
        }
    }

    fn show_menu(&self) {
        println!("\n{}", "=".repeat(50));
        println!("🪙  منوی RustCoin");
        println!("{}", "=".repeat(50));
        println!("1️⃣  نمایش اطلاعات بلاک چین");
        println!("2️⃣  نمایش اطلاعات کیف پول فعلی");
        println!("3️⃣  تغییر کیف پول فعلی");
        println!("4️⃣  ایجاد تراکنش جدید");
        println!("5️⃣  استخراج بلاک");
        println!("6️⃣  نمایش تاریخچه تراکنش‌ها");
        println!("7️⃣  اعتبارسنجی بلاک چین");
        println!("8️⃣  ایجاد کیف پول جدید");
        println!("9️⃣  صادر کردن بلاک چین");
        println!("0️⃣  خروج");
        println!("{}", "=".repeat(50));
        println!("کیف پول فعلی: {} (شماره {})", 
                 &self.wallets[self.current_wallet_index].get_address()[..16], 
                 self.current_wallet_index + 1);
    }

    fn show_blockchain_info(&self) {
        println!("\n📊 اطلاعات بلاک چین:");
        println!("{}", "=".repeat(40));
        let info = self.blockchain.get_chain_info();
        println!("📦 تعداد بلاک‌ها: {}", info.total_blocks);
        println!("💸 تعداد تراکنش‌ها: {}", info.total_transactions);
        println!("💎 کل عرضه: {} RustCoin", info.total_supply);
        println!("⚙️  سختی: {}", info.difficulty);
        println!("🏆 پاداش استخراج: {} RustCoin", info.mining_reward);
        println!("⏳ تراکنش‌های در انتظار: {}", info.pending_transactions);
    }

    fn show_wallet_info(&self) {
        let wallet = &self.wallets[self.current_wallet_index];
        println!("\n💼 اطلاعات کیف پول:");
        println!("{}", "=".repeat(40));
        println!("📍 آدرس کامل: {}", wallet.get_address());
        println!("💰 موجودی: {} RustCoin", self.blockchain.get_balance(&wallet.get_address()));
        
        // نمایش تراکنش‌های مربوط به این کیف پول
        let transactions = self.blockchain.get_transactions_for_address(&wallet.get_address());
        println!("📝 تعداد تراکنش‌ها: {}", transactions.len());
    }

    fn switch_wallet(&mut self) {
        println!("\n👛 کیف پول‌های موجود:");
        for (i, wallet) in self.wallets.iter().enumerate() {
            let balance = self.blockchain.get_balance(&wallet.get_address());
            let marker = if i == self.current_wallet_index { "👉" } else { "  " };
            println!("{} {}. {} | موجودی: {} RustCoin", 
                     marker, i + 1, &wallet.get_address()[..16], balance);
        }
        
        print!("شماره کیف پول مورد نظر (1-{}): ", self.wallets.len());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if let Ok(index) = input.trim().parse::<usize>() {
            if index > 0 && index <= self.wallets.len() {
                self.current_wallet_index = index - 1;
                println!("✅ کیف پول {} انتخاب شد", index);
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
        
        println!("\n💸 ایجاد تراکنش جدید");
        println!("موجودی فعلی: {} RustCoin", balance);
        
        // انتخاب گیرنده
        println!("\nگیرنده را انتخاب کنید:");
        println!("1. وارد کردن آدرس دستی");
        println!("2. انتخاب از کیف پول‌های موجود");
        
        print!("انتخاب: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let to_address = match input.trim() {
            "1" => {
                print!("آدرس گیرنده: ");
                io::stdout().flush().unwrap();
                let mut addr = String::new();
                io::stdin().read_line(&mut addr).unwrap();
                addr.trim().to_string()
            }
            "2" => {
                println!("کیف پول‌های موجود:");
                for (i, wallet) in self.wallets.iter().enumerate() {
                    if i != self.current_wallet_index {
                        println!("{}. {}", i + 1, &wallet.get_address()[..20]);
                    }
                }
                
                print!("شماره کیف پول: ");
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
        
        // مقدار تراکنش
        print!("مقدار (حداکثر {}): ", balance);
        io::stdout().flush().unwrap();
        
        let mut amount_input = String::new();
        io::stdin().read_line(&mut amount_input).unwrap();
        
        if let Ok(amount) = amount_input.trim().parse::<f64>() {
            if amount > 0.0 && amount <= balance {
                // ایجاد تراکنش
                match Transaction::new(
                    current_wallet.get_address(),
                    to_address.clone(),
                    amount,
                    current_wallet,
                ) {
                    Ok(transaction) => {
                        match self.blockchain.add_transaction(transaction) {
                            Ok(_) => {
                                println!("✅ تراکنش با موفقیت ایجاد شد!");
                                println!("📝 تراکنش به صف انتظار اضافه شد");
                            }
                            Err(e) => println!("❌ خطا در اضافه کردن تراکنش: {}", e),
                        }
                    }
                    Err(e) => println!("❌ خطا در ایجاد تراکنش: {}", e),
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
            println!("❌ هیچ تراکنش در انتظاری وجود ندارد!");
            return;
        }
        
        let current_wallet = &self.wallets[self.current_wallet_index];
        
        println!("⛏️  شروع استخراج...");
        
        match self.blockchain.mine_pending_transactions(&current_wallet.get_address()).await {
            Ok(_) => {
                println!("✅ بلاک با موفقیت استخراج شد!");
                let balance = self.blockchain.get_balance(&current_wallet.get_address());
                println!("💰 موجودی جدید: {} RustCoin", balance);
            }
            Err(e) => println!("❌ خطا در استخراج: {}", e),
        }
    }

    fn show_transaction_history(&self) {
        let current_wallet = &self.wallets[self.current_wallet_index];
        let transactions = self.blockchain.get_transactions_for_address(&current_wallet.get_address());
        
        println!("\n📜 تاریخچه تراکنش‌ها:");
        println!("{}", "=".repeat(50));
        
        if transactions.is_empty() {
            println!("هیچ تراکنشی یافت نشد.");
        } else {
            for (i, tx) in transactions.iter().enumerate() {
                println!("{}. {}", i + 1, tx);
            }
        }
    }

    fn validate_blockchain(&self) {
        println!("\n🔍 اعتبارسنجی بلاک چین...");
        
        if self.blockchain.is_chain_valid() {
            println!("✅ بلاک چین معتبر است!");
        } else {
            println!("❌ بلاک چین نامعتبر است!");
        }
    }

    fn create_new_wallet(&mut self) {
        let new_wallet = Wallet::new();
        println!("\n👛 کیف پول جدید ایجاد شد!");
        println!("📍 آدرس: {}", new_wallet.get_address());
        
        self.wallets.push(new_wallet);
        println!("✅ کیف پول به لیست اضافه شد (شماره {})", self.wallets.len());
    }

    fn export_blockchain(&self) {
        match self.blockchain.export_json() {
            Ok(json) => {
                println!("\n📤 صادر کردن بلاک چین:");
                println!("{}", "=".repeat(40));
                println!("{}", &json[..500.min(json.len())]);
                if json.len() > 500 {
                    println!("... (کل {} کاراکتر)", json.len());
                }
            }
            Err(e) => println!("❌ خطا در صادر کردن: {}", e),
        }
    }
}