use std::collections::HashMap;
use std::io::{self, Write};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize)]
// Wallet struct means it is a structure created for the users wallet that stores
// Customers id, usd Balance, ethereum balance, also I'll add canadian dollars balance and bitcoin balance 
// And date and time when the transaction is made

pub struct Wallet {
    pub id: Uuid,
    pub usd_balance: Decimal,
    pub eth_balance: Decimal,
    pub created_at: DateTime<Utc>
}


#[derive(Debug, Clone, Serialize, Deserialize)]
// Only two  possible options for the order type
// Buy, Sell, CancelOrder

pub enum TransactionType{
    Buy, Sell, Cancel_Order
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Filled,
    Cancelled,
    Partial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub order_type: TransactionType,
    pub amount: Decimal,
    pub price: Decimal,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub filled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub order_id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    pub price: Decimal,
    pub total_value: Decimal,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct TradingSystem {
    pub wallets: HashMap<Uuid, Wallet>,
    pub orders: HashMap<Uuid, Order>,
    pub transactions: Vec<Transaction>,
    pub current_eth_price: Decimal,
}

impl Wallet {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            usd_balance: Decimal::from(10000),
            eth_balance: Decimal::ZERO,
            created_at: Utc::now(),
        }
    }

    pub fn can_afford_buy(&self, amount: Decimal, price: Decimal) -> bool {
        let total_cost = amount * price;
        self.usd_balance >= total_cost
    }

    pub fn can_afford_sell(&self, amount: Decimal) -> bool {
        self.eth_balance >= amount
    }

    pub fn buy_eth(&mut self, amount: Decimal, price: Decimal) -> Result<(), String> {
        let total_cost = amount * price;
        if !self.can_afford_buy(amount, price) {
            return Err("Insufficient USD balance".to_string());
        }
        
        self.usd_balance -= total_cost;
        self.eth_balance += amount;
        Ok(())
    }

    pub fn sell_eth(&mut self, amount: Decimal, price: Decimal) -> Result<(), String> {
        if !self.can_afford_sell(amount) {
            return Err("Insufficient ETH balance".to_string());
        }
        
        self.eth_balance -= amount;
        self.usd_balance += amount * price;
        Ok(())
    }

    pub fn get_total_value_usd(&self, eth_price: Decimal) -> Decimal {
        self.usd_balance + (self.eth_balance * eth_price)
    }
}

impl TradingSystem {
    pub fn new() -> Self {
        Self {
            wallets: HashMap::new(),
            orders: HashMap::new(),
            transactions: Vec::new(),
            current_eth_price: Decimal::from_str("2500").unwrap(), // Starting ETH price
        }
    }

    pub fn create_wallet(&mut self) -> Uuid {
        let wallet = Wallet::new();
        let wallet_id = wallet.id;
        self.wallets.insert(wallet_id, wallet);
        wallet_id
    }

    pub fn create_order(&mut self, wallet_id: Uuid, order_type: TransactionType, amount: Decimal, price: Decimal) -> Result<Uuid, String> {
        let wallet = self.wallets.get(&wallet_id).ok_or("Wallet not found")?;
        
        // Validate order
        match order_type {
            TransactionType::Buy => {
                if !wallet.can_afford_buy(amount, price) {
                    return Err("Insufficient USD balance for buy order".to_string());
                }
            },
            TransactionType::Sell => {
                if !wallet.can_afford_sell(amount) {
                    return Err("Insufficient ETH balance for sell order".to_string());
                }
            },
            TransactionType::Cancel_Order => {
                if order_type != TransactionType::Cancel_Order{
                    return Err("Cannot cancel order".to_string());
                }
            }
        }

        let order = Order {
            id: Uuid::new_v4(),
            wallet_id,
            order_type,
            amount,
            price,
            status: OrderStatus::Pending,
            created_at: Utc::now(),
            filled_at: None,
        };

        let order_id = order.id;
        self.orders.insert(order_id, order);
        Ok(order_id)
    }

    pub fn execute_order(&mut self, order_id: Uuid) -> Result<(), String> {
        let mut order = self.orders.get_mut(&order_id).ok_or("Order not found")?;
        
        if order.status != OrderStatus::Pending {
            return Err("Order is not pending".to_string());
        }

        let wallet = self.wallets.get_mut(&order.wallet_id).ok_or("Wallet not found")?;
        
        // Execute the trade
        match order.order_type {
            TransactionType::Buy => {
                wallet.buy_eth(order.amount, order.price)?;
            },
            TransactionType::Sell => {
                wallet.sell_eth(order.amount, order.price)?;
            },
            TransactionType::Cancel_Order => {
                if order.order_type != TransactionType::Cancel_Order{
                    return Err("Cannot cancel order".to_string());
                }
                return Ok(());
            }
        }

        // Create transaction record
        let transaction = Transaction {
            id: Uuid::new_v4(),
            wallet_id: order.wallet_id,
            order_id: order.id,
            transaction_type: order.order_type.clone(),
            amount: order.amount,
            price: order.price,
            total_value: order.amount * order.price,
            timestamp: Utc::now(),
        };

        self.transactions.push(transaction);
        
        // Update order status
        order.status = OrderStatus::Filled;
        order.filled_at = Some(Utc::now());
        
        Ok(())
    }

    pub fn get_wallet_balance(&self, wallet_id: Uuid) -> Option<(Decimal, Decimal, Decimal)> {
        let wallet = self.wallets.get(&wallet_id)?;
        let total_value = wallet.get_total_value_usd(self.current_eth_price);
        Some((wallet.usd_balance, wallet.eth_balance, total_value))
    }

    pub fn get_transaction_history(&self, wallet_id: Uuid) -> Vec<&Transaction> {
        self.transactions
            .iter()
            .filter(|t| t.wallet_id == wallet_id)
            .collect()
    }

    pub fn simulate_price_change(&mut self) {
        // Simple price simulation - random walk
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        Utc::now().timestamp().hash(&mut hasher);
        let random_factor = (hasher.finish() % 200) as i64 - 100; // -100 to +100
        
        let change_percent = Decimal::from(random_factor) / Decimal::from(10000); // 0.01% max change
        let price_change = self.current_eth_price * change_percent;
        self.current_eth_price += price_change;
        
        // Keep price reasonable (between $1000 and $5000)
        if self.current_eth_price < Decimal::from(1000) {
            self.current_eth_price = Decimal::from(1000);
        } else if self.current_eth_price > Decimal::from(5000) {
            self.current_eth_price = Decimal::from(5000);
        }
    }

    pub fn get_market_summary(&self) -> (Decimal, usize, usize) {
        let total_orders = self.orders.len();
        let filled_orders = self.orders.values().filter(|o| matches!(o.status, OrderStatus::Filled)).count();
        (self.current_eth_price, total_orders, filled_orders)
    }
}

fn main() {
    let mut trading_system = TradingSystem::new();
    let mut current_wallet_id: Option<Uuid> = None;

    println!("🚀 Welcome to the Ethereum Trading System! 🚀");
    println!("Current ETH Price: ${}", trading_system.current_eth_price);

    loop {
        println!("\n=== Trading Menu ===");
        println!("1. Create Wallet");
        println!("2. View Balance");
        println!("3. Buy Ethereum");
        println!("4. Sell Ethereum");
        println!("5. View Transaction History");
        println!("6. View Market Summary");
        println!("7. Simulate Price Change");
        println!("8. Exit");

        print!("\nEnter your choice (1-8): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => {
                let wallet_id = trading_system.create_wallet();
                current_wallet_id = Some(wallet_id);
                println!("✅ Wallet created successfully! Wallet ID: {}", wallet_id);
            },
            "2" => {
                if let Some(wallet_id) = current_wallet_id {
                    if let Some((usd, eth, total)) = trading_system.get_wallet_balance(wallet_id) {
                        println!("\n💰 Wallet Balance:");
                        println!("USD: ${:.2}", usd);
                        println!("ETH: {:.6}", eth);
                        println!("Total Value: ${:.2}", total);
                    } else {
                        println!("❌ Wallet not found!");
                    }
                } else {
                    println!("❌ Please create a wallet first!");
                }
            },
            "3" => {
                if let Some(wallet_id) = current_wallet_id {
                    println!("\n💵 Buy Ethereum");
                    println!("Current ETH Price: ${}", trading_system.current_eth_price);
                    
                    print!("Enter amount of ETH to buy: ");
                    io::stdout().flush().unwrap();
                    let mut amount_str = String::new();
                    io::stdin().read_line(&mut amount_str).expect("Failed to read input");
                    
                    if let Ok(amount) = amount_str.trim().parse::<f64>() {
                        let amount_decimal = Decimal::from_f64_retain(amount).unwrap_or(Decimal::ZERO);
                        let price = trading_system.current_eth_price;
                        
                        match trading_system.create_order(wallet_id, TransactionType::Buy, amount_decimal, price) {
                            Ok(order_id) => {
                                match trading_system.execute_order(order_id) {
                                    Ok(_) => {
                                        println!("✅ Successfully bought {:.6} ETH at ${:.2}!", amount, price);
                                        println!("Total cost: ${:.2}", amount_decimal * price);
                                    },
                                    Err(e) => println!("❌ Error executing buy order: {}", e),
                                }
                            },
                            Err(e) => println!("❌ Error creating buy order: {}", e),
                        }
                    } else {
                        println!("❌ Invalid amount entered!");
                    }
                } else {
                    println!("❌ Please create a wallet first!");
                }
            },
            "4" => {
                if let Some(wallet_id) = current_wallet_id {
                    println!("\n💸 Sell Ethereum");
                    println!("Current ETH Price: ${}", trading_system.current_eth_price);
                    
                    print!("Enter amount of ETH to sell: ");
                    io::stdout().flush().unwrap();
                    let mut amount_str = String::new();
                    io::stdin().read_line(&mut amount_str).expect("Failed to read input");
                    
                    if let Ok(amount) = amount_str.trim().parse::<f64>() {
                        let amount_decimal = Decimal::from_f64_retain(amount).unwrap_or(Decimal::ZERO);
                        let price = trading_system.current_eth_price;
                        
                        match trading_system.create_order(wallet_id, TransactionType::Sell, amount_decimal, price) {
                            Ok(order_id) => {
                                match trading_system.execute_order(order_id) {
                                    Ok(_) => {
                                        println!("✅ Successfully sold {:.6} ETH at ${:.2}!", amount, price);
                                        println!("Total received: ${:.2}", amount_decimal * price);
                                    },
                                    Err(e) => println!("❌ Error executing sell order: {}", e),
                                }
                            },
                            Err(e) => println!("❌ Error creating sell order: {}", e),
                        }
                    } else {
                        println!("❌ Invalid amount entered!");
                    }
                } else {
                    println!("❌ Please create a wallet first!");
                }
            },
            "5" => {
                if let Some(wallet_id) = current_wallet_id {
                    let transactions = trading_system.get_transaction_history(wallet_id);
                    if transactions.is_empty() {
                        println!("📋 No transactions found.");
                    } else {
                        println!("\n📋 Transaction History:");
                        for (i, tx) in transactions.iter().enumerate() {
                            let tx_type = match tx.transaction_type {
                                TransactionType::Buy => "BUY",
                                TransactionType::Sell => "SELL",
                                TransactionType::CancelOrder => "ORDER CANCELLED"
                            };
                            println!("{}. {} {} {:.6} ETH @ ${:.2} (Total: ${:.2}) - {}",
                                i + 1, tx_type, tx.amount, tx.price, tx.total_value, tx.timestamp.format("%Y-%m-%d %H:%M:%S"));
                        }
                    }
                } else {
                    println!("❌ Please create a wallet first!");
                }
            },
            "6" => {
                let (price, total_orders, filled_orders) = trading_system.get_market_summary();
                println!("\n📊 Market Summary:");
                println!("Current ETH Price: ${:.2}", price);
                println!("Total Orders: {}", total_orders);
                println!("Filled Orders: {}", filled_orders);
            },
            "7" => {
                trading_system.simulate_price_change();
                println!("📈 Price updated! New ETH Price: ${:.2}", trading_system.current_eth_price);
            },
            "8" => {
                println!("👋 Thank you for using the Ethereum Trading System!");
            break;
            },
            _ => {
                println!("❌ Invalid choice! Please enter 1-8.");
            }
        }
    }
}
