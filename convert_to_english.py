#!/usr/bin/env python3
"""
Convert Persian/Farsi text to English in TenCoin project
"""

import os
import re

# Dictionary for common translations
translations = {
    # Comments
    "// ایجاد": "// Create",
    "// اضافه کردن": "// Add",
    "// بررسی": "// Check",
    "// محاسبه": "// Calculate",
    "// تولید": "// Generate",
    "// ارسال": "// Send",
    "// دریافت": "// Receive",
    "// به‌روزرسانی": "// Update",
    "// حذف": "// Delete",
    "// تبدیل": "// Convert",
    "// شروع": "// Start",
    "// پایان": "// End",
    "// انتظار": "// Wait",
    "// اعتبارسنجی": "// Validation",
    "// امضا": "// Sign",
    "// رمزنگاری": "// Encryption",
    "// هش": "// Hash",
    "// استخراج": "// Mining",
    
    # Error messages
    "خطا در": "Error in",
    "نامعتبر است": "is invalid",
    "یافت نشد": "not found",
    "موجودی ناکافی": "Insufficient balance",
    "تراکنش نامعتبر": "Invalid transaction",
    "بلاک نامعتبر": "Invalid block",
    "کلید نامعتبر": "Invalid key",
    "آدرس نامعتبر": "Invalid address",
    "امضای نامعتبر": "Invalid signature",
    
    # Success messages
    "با موفقیت": "successfully",
    "ایجاد شد": "created",
    "اضافه شد": "added",
    "ارسال شد": "sent",
    "دریافت شد": "received",
    "استخراج شد": "mined",
    "تأیید شد": "confirmed",
    
    # UI text
    "شروع": "Start",
    "توقف": "Stop",
    "ادامه": "Continue",
    "لغو": "Cancel",
    "تأیید": "Confirm",
    "رد": "Reject",
    "بله": "Yes",
    "خیر": "No",
    
    # Blockchain terms
    "بلاک چین": "blockchain",
    "بلاک": "block",
    "تراکنش": "transaction",
    "کیف پول": "wallet",
    "آدرس": "address",
    "موجودی": "balance",
    "پاداش": "reward",
    "سختی": "difficulty",
    "هاوینگ": "halving",
    "استخراج": "mining",
    "ماینر": "miner",
    "نود": "node",
    "شبکه": "network",
    "همگام‌سازی": "synchronization",
    
    # Print statements
    "در حال اجرا": "running",
    "متوقف شد": "stopped",
    "شروع شد": "started",
    "تکمیل شد": "completed",
    "در انتظار": "pending",
    "فعال": "active",
    "غیرفعال": "inactive",
    
    # File operations
    "خواندن": "reading",
    "نوشتن": "writing",
    "ذخیره": "saving",
    "بارگذاری": "loading",
    "صادر کردن": "exporting",
    "وارد کردن": "importing",
}

def convert_file(filepath):
    """Convert Persian text to English in a file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply translations
        for persian, english in translations.items():
            content = content.replace(persian, english)
        
        # Additional regex replacements for common patterns
        patterns = [
            (r'println!\("([^"]*ایجاد[^"]*)"', lambda m: f'println!("{m.group(1).replace("ایجاد", "Creating")}"'),
            (r'println!\("([^"]*شروع[^"]*)"', lambda m: f'println!("{m.group(1).replace("شروع", "Starting")}"'),
            (r'println!\("([^"]*تکمیل[^"]*)"', lambda m: f'println!("{m.group(1).replace("تکمیل", "Completed")}"'),
        ]
        
        for pattern, replacement in patterns:
            content = re.sub(pattern, replacement, content)
        
        # Only write if content changed
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✅ Converted: {filepath}")
            return True
        else:
            print(f"ℹ️  No changes: {filepath}")
            return False
            
    except Exception as e:
        print(f"❌ Error processing {filepath}: {e}")
        return False

def main():
    """Main function"""
    src_dir = "src"
    converted_files = 0
    
    if not os.path.exists(src_dir):
        print(f"❌ Directory {src_dir} not found")
        return
    
    # Process all .rs files
    for filename in os.listdir(src_dir):
        if filename.endswith('.rs'):
            filepath = os.path.join(src_dir, filename)
            if convert_file(filepath):
                converted_files += 1
    
    # Process Python files
    for filename in os.listdir('.'):
        if filename.endswith('.py') and filename != 'convert_to_english.py':
            if convert_file(filename):
                converted_files += 1
    
    print(f"\n🎉 Conversion completed! {converted_files} files modified.")

if __name__ == "__main__":
    main()