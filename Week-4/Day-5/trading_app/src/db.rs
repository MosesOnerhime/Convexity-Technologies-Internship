// src/db.rs
use mysql::*;
use mysql::prelude::*;
use bcrypt::{hash, verify};

#[derive(Clone)]
pub struct DbPool {
    pool: Pool,
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
}
