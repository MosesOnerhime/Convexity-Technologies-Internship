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
        let mut conn = self.get_conn()?;
        conn.exec_drop(
            r"INSERT INTO orders (username, symbol, order_type, quantity, price) VALUES (:username, :symbol, :order_type, :quantity, :price)",
            params! {
                "username" => username,
                "symbol" => symbol,
                "order_type" => order_type,
                "quantity" => quantity,
                "price" => price,
            }
        )?;
        Ok(())
    }

    // get the order history for a user
    pub fn get_order_history(&self, username: &str) -> Result<Vec<Order>> {
        let mut conn = self.get_conn()?;
        let result: Vec<Order> = conn.exec_map(
            r"SELECT symbol, order_type, quantity, price, created_at FROM orders WHERE username = :username ORDER BY created_at DESC",
            params! {
                "username" => username,
            },
            |(symbol, order_type, quantity, price, created_at)| Order {
                symbol,
                order_type,
                quantity,
                price,
                created_at,
            }
        )?;
        Ok(result)
    }
}


