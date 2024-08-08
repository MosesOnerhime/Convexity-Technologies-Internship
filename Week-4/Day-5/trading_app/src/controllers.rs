// src/controllers.rs
use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use tera::{Tera, Context};
use crate::db::DbPool;

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserForm {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct TradeForm {
    pub symbol: String,
}

pub async fn login_form(tera: web::Data<Tera>) -> impl Responder {
    let s = tera.render("login.html", &Context::new()).unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}

pub async fn login_process(
    form: web::Form<LoginForm>,
    db_pool: web::Data<DbPool>,
    id: Identity
) -> impl Responder {
    let login_result = db_pool.verify_user(&form.username, &form.password).unwrap();

    if login_result {
        id.remember(form.username.clone());
        HttpResponse::Found()
            .append_header(("Location", "/trading?login=success"))
            .finish()
    } else {
        HttpResponse::Found()
            .append_header(("Location", "/?login=failed"))
            .finish()
    }
}

pub async fn create_user_form(tera: web::Data<Tera>) -> impl Responder {
    let s = tera.render("create_user.html", &Context::new()).unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}

pub async fn create_user_process(
    form: web::Form<CreateUserForm>,
    db_pool: web::Data<DbPool>,
    id: Identity
) -> impl Responder {
    let _ = db_pool.create_user(&form.username, &form.password);
    let login_result = db_pool.verify_user(&form.username, &form.password).unwrap();

    if login_result {
        id.remember(form.username.clone());
        HttpResponse::Found()
            .append_header(("Location", "/trading?login=success"))
            .finish()
    } else {
        HttpResponse::Found()
            .append_header(("Location", "/create_user?login=failed"))
            .finish()
    }
}

pub async fn trading_form(id: Identity, tera: web::Data<Tera>) -> impl Responder {
    if let Some(username) = id.identity() {
        let mut context = Context::new();
        context.insert("username", &username);
        let s = tera.render("trading.html", &context).unwrap();
        HttpResponse::Ok().content_type("text/html").body(s)
    } else {
        HttpResponse::Unauthorized().body("Please log in to view this page")
    }
}

pub async fn trade_process(
    form: web::Form<TradeForm>, 
    tera: web::Data<Tera>,
    id: Identity
) -> impl Responder {
    if let Some(username) = id.identity() {
        // Call the Alpha Vantage API to get market data
        let api_key = "HX7K0WCKTZZ1KEJY";
        let url = format!("https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol={}&interval=1min&apikey={}", form.symbol, api_key);
        let response = reqwest::get(&url).await.unwrap().json::<serde_json::Value>().await.unwrap();

        // Check if the response contains valid data
        let price_result = response["Time Series (1min)"]
            .as_object()
            .and_then(|data| data.values().next())
            .and_then(|value| value.as_object())
            .and_then(|obj| obj.get("1. open"))
            .and_then(|price| price.as_str())
            .and_then(|price_str| price_str.parse::<f64>().ok());

        let mut context = Context::new();
        context.insert("username", &username);

        match price_result {
            Some(price) => {
                context.insert("price", &price);
            },
            None => {
                context.insert("error", "Symbol not found!");
            }
        }

        let s = tera.render("trading.html", &context).unwrap();
        HttpResponse::Ok().content_type("text/html").body(s)
    } else {
        HttpResponse::Unauthorized().body("Please log in to perform trading")
    }
}
