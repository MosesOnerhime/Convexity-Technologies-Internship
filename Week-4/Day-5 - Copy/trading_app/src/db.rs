// src/db.rs
use mysql::*;
use mysql::prelude::*;
use bcrypt::{hash, verify};
use serde::Serialize;

#[derive(Clone)]
pub struct DbPool {
    pool: Pool,
}

#[derive(Debug, Serialize)]
pub struct Order {
    pub symbol: String,
    pub order_type: String,
    pub quantity: i32,
    pub price: f64,
    pub total: f64,
    pub created_at: chrono::NaiveDateTime,
}

impl DbPool {
    pub fn new(url: &str) -> Result<Self> {
        let pool = Pool::new(url)?;
        Ok(DbPool { pool })
    }

    pub fn get_conn(&self) -> Result<PooledConn> {
        self.pool.get_conn()
    }

    pub fn create_user(&self, username: &str, password: &str) -> Result<()> {
        let mut conn = self.get_conn()?;
        let hashed_password = hash(password, 4).unwrap(); // Hash password with bcrypt
        conn.exec_drop(
            r"INSERT INTO users (username, password_hash) VALUES (:username, :password_hash)",
            params! {
                "username" => username,
                "password_hash" => hashed_password,
            }
        )?;
        Ok(())
    }

    // Method to get user's current balance
    pub fn get_balance(&self, username: &str) -> Result<f64> {
        let mut conn = self.get_conn()?;
        let balance: f64 = conn.exec_first(
            r"SELECT account_balance FROM users WHERE username = :username",
            params! {
                "username" => username,
            }
        )?.unwrap_or(0.0); // Return 0.0 if user not found
        Ok(balance)
    }

    // Method to update user's balance after a trade
    pub fn update_balance(&self, username: &str, new_balance: f64) -> Result<()> {
        let mut conn = self.get_conn()?;
        conn.exec_drop(
            r"UPDATE users SET account_balance = :new_balance WHERE username = :username",
            params! {
                "username" => username,
                "new_balance" => new_balance,
            }
        )?;
        Ok(())
    }

    pub fn verify_user(&self, username: &str, password: &str) -> Result<bool> {
        let mut conn = self.get_conn()?;
        let result: Option<String> = conn.exec_first(
            r"SELECT password_hash FROM users WHERE username = :username",
            params! {
                "username" => username,
            }
        )?;
        
        if let Some(password_hash) = result {
            Ok(verify(password, &password_hash).unwrap())
        } else {
            Ok(false)
        }
    }

    // save a new order in the database
    pub fn save_order(&self, username: &str, symbol: &str, order_type: &str, quantity: i32, price: f64) -> Result<()> {
        let total = price * quantity as f64; // Calculate total price
        let mut conn = self.get_conn()?;
        conn.exec_drop(
            r"INSERT INTO orders (username, symbol, order_type, quantity, price, total) VALUES (:username, :symbol, :order_type, :quantity, :price, :total)",
            params! {
                "username" => username,
                "symbol" => symbol,
                "order_type" => order_type,
                "quantity" => quantity,
                "price" => price,
                "total" => total,
            }
        )?;
        Ok(())
    }
    

    // get the order history for a user
    pub fn get_order_history(&self, username: &str) -> Result<Vec<Order>> {
        let mut conn = self.get_conn()?;
        let result: Vec<Order> = conn.exec_map(
            r"SELECT symbol, order_type, quantity, price, total, created_at FROM orders WHERE username = :username ORDER BY created_at DESC",
            params! {
                "username" => username,
            },
            |(symbol, order_type, quantity, price, total, created_at)| Order {
                symbol,
                order_type,
                quantity,
                price,
                total, 
                created_at,
            }
        )?;
        Ok(result)
    }
    
    // Place a buy order
    pub fn place_buy_order(&self, username: &str, symbol: &str, quantity: i32, max_price: f64) -> Result<()> {
        let mut conn = self.get_conn()?;
        conn.exec_drop(
            r"INSERT INTO buyers (username, symbol, quantity, max_price) VALUES (:username, :symbol, :quantity, :max_price)",
            params! {
                "username" => username,
                "symbol" => symbol,
                "quantity" => quantity,
                "max_price" => max_price,
            }
        )?;
        Ok(())
    }

    // Place a sell order
    pub fn place_sell_order(&self, username: &str, symbol: &str, quantity: i32, min_price: f64) -> Result<()> {
        let mut conn = self.get_conn()?;
        conn.exec_drop(
            r"INSERT INTO sellers (username, symbol, quantity, min_price) VALUES (:username, :symbol, :quantity, :min_price)",
            params! {
                "username" => username,
                "symbol" => symbol,
                "quantity" => quantity,
                "min_price" => min_price,
            }
        )?;
        Ok(())
    }

    pub fn match_orders(&self) -> Result<()> {
        let mut conn = self.get_conn()?;
        // Find matching sell orders for each buy order
        let buy_orders: Vec<(i32, String, String, i32, f64)> = conn.exec_map(
            r"SELECT id, username, symbol, quantity, max_price FROM buyers",
            (),
            |(id, username, symbol, quantity, max_price)| (id, username, symbol, quantity, max_price)
        )?;

        for (buy_id, buyer_username, symbol, mut buy_quantity, max_price) in buy_orders {
            let sell_orders: Vec<(i32, String, String, i32, f64)> = conn.exec_map(
                r"SELECT id, username, symbol, quantity, min_price FROM sellers WHERE symbol = :symbol AND min_price <= :max_price",
                params! {
                    "symbol" => &symbol,
                    "max_price" => max_price,
                },
                |(id, seller_username, symbol, quantity, min_price)| (id, seller_username, symbol, quantity, min_price)
            )?;

            // Process each matching sell order
            for (sell_id, seller_username, _symbol, sell_quantity, _min_price) in sell_orders {
                let trade_quantity = std::cmp::min(buy_quantity, sell_quantity);
                let trade_price = _min_price;

                // Execute trade
                conn.exec_drop(
                    r"INSERT INTO trade_orders (buyer_username, seller_username, symbol, quantity, price) VALUES (:buyer_username, :seller_username, :symbol, :quantity, :price)",
                    params! {
                        "buyer_username" => buyer_username.clone(),
                        "seller_username" => seller_username.clone(),
                        "symbol" => symbol.clone(),
                        "quantity" => trade_quantity,
                        "price" => trade_price,
                    }
                )?;

                // Update order quantities
                conn.exec_drop(
                    r"UPDATE buyers SET quantity = quantity - :quantity WHERE id = :id",
                    params! {
                        "quantity" => trade_quantity,
                        "id" => buy_id,
                    }
                )?;

                conn.exec_drop(
                    r"UPDATE sellers SET quantity = quantity - :quantity WHERE id = :id",
                    params! {
                        "quantity" => trade_quantity,
                        "id" => sell_id,
                    }
                )?;

                // Check if buyer's or seller's order is fully fulfilled and remove it if so
                conn.exec_drop(
                    r"DELETE FROM buyers WHERE id = :id AND quantity <= 0",
                    params! {
                        "id" => buy_id,
                    }
                )?;

                conn.exec_drop(
                    r"DELETE FROM sellers WHERE id = :id AND quantity <= 0",
                    params! {
                        "id" => sell_id,
                    }
                )?;

                // Adjust remaining quantities
                buy_quantity -= trade_quantity;
                if buy_quantity <= 0 {
                    break;
                }
            }
        }

        Ok(())
    }


}


