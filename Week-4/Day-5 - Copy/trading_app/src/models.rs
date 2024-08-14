// src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct MarketData {
    pub symbol: String,
    pub price: f64,
}
