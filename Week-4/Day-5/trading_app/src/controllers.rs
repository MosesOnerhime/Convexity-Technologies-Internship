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
    pub order_type: String,  // "buy" or "sell"
    pub quantity: u32,
}

pub async fn home_form(tera: web::Data<Tera>) -> impl Responder {
    let s = tera.render("home.html", &Context::new()).unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
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

pub async fn trading_form(id: Identity, tera: web::Data<Tera>, db_pool: web::Data<DbPool>) -> impl Responder {
    if let Some(username) = id.identity() {
        let mut context = Context::new();

        // Fetch the current balance
        let current_balance = match db_pool.get_balance(&username) {
            Ok(balance) => balance,
            Err(_) => 0.0,
        };
        context.insert("new_balance", &current_balance);
        context.insert("username", &username);

        // Fetch the order history
        let orders = db_pool.get_order_history(&username).unwrap();
        context.insert("orders", &orders);

        let s = tera.render("trading.html", &context).unwrap();
        HttpResponse::Ok().content_type("text/html").body(s)
    } else {
        HttpResponse::Unauthorized().body("Please log in to view this page")
    }
}

pub async fn trade_process(
    form: web::Form<TradeForm>, 
    tera: web::Data<Tera>,
    id: Identity,
    db_pool: web::Data<DbPool>,
) -> impl Responder {
    if let Some(username) = id.identity() {
        let mut context = Context::new();
        context.insert("username", &username);

        // Fetch the current balance
        let current_balance = match db_pool.get_balance(&username) {
            Ok(balance) => balance,
            Err(_) => 0.0,
        };
        context.insert("current_balance", &current_balance);

        

        // API call to fetch the price
        let api_key = "HX7K0WCKTZZ1KEJY";
        let url = format!("https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol={}&interval=1min&apikey={}", form.symbol, api_key);

        // Handle potential errors in the API call
        let response = match reqwest::get(&url).await {
            Ok(resp) => match resp.json::<serde_json::Value>().await {
                Ok(json) => Some(json),
                Err(_) => None,
            },
            Err(_) => None,
        };

        if let Some(response) = response {
            let price_result = response["Time Series (1min)"]
                .as_object()
                .and_then(|data| data.values().next())
                .and_then(|value| value.as_object())
                .and_then(|obj| obj.get("1. open"))
                .and_then(|price| price.as_str())
                .and_then(|price_str| price_str.parse::<f64>().ok());

            match price_result {
                Some(price) => {
                    let total_value = price * form.quantity as f64;

                    if current_balance >= total_value && &form.order_type == "buy" {     //buy option
                        let new_balance = current_balance - total_value;
                        if let Err(_) = db_pool.update_balance(&username, new_balance) {
                            context.insert("error", "Failed to update balance!");
                        }

                        let _ = db_pool.place_buy_order(&username, &form.symbol.to_uppercase(), form.quantity as i32, total_value);
                        // Trigger order matching
                        let _ = db_pool.match_orders();
                        if let Err(_) = db_pool.save_order(&username, &form.symbol.to_uppercase(), &form.order_type, form.quantity as i32, price) {
                            context.insert("error", "Failed to save order!");
                        } else {
                            context.insert("price", &price);
                            context.insert("order_type", &form.order_type);
                            context.insert("quantity", &form.quantity);
                            context.insert("total_value", &total_value);
                            context.insert("new_balance", &new_balance);
                        }
                    } else if &form.order_type == "sell" {     // sell option
                        let new_balance = current_balance + total_value;
                        if let Err(_) = db_pool.update_balance(&username, new_balance) {
                            context.insert("error", "Failed to update balance!");
                        }

                        let _ = db_pool.place_sell_order(&username, &form.symbol, form.quantity as i32, total_value);
                        // Trigger order matching
                        let _ = db_pool.match_orders();
                        if let Err(_) = db_pool.save_order(&username, &form.symbol.to_uppercase(), &form.order_type, form.quantity as i32, price) {
                            context.insert("error", "Failed to save order!");
                        } else {
                            context.insert("price", &price);
                            context.insert("order_type", &form.order_type);
                            context.insert("quantity", &form.quantity);
                            context.insert("total_value", &total_value);
                            context.insert("new_balance", &new_balance);
                        }
                    } else {
                        context.insert("error", "Insufficient balance!");
                    }

                    // Fetch the order history
                    let orders = db_pool.get_order_history(&username).unwrap();
                    context.insert("orders", &orders);
                    
                },
                None => {
                    context.insert("error", "Symbol not found!");
                }
            }
        } else {
            context.insert("error", "Failed to retrieve price data!");
        }


        // Render the template
        match tera.render("trading.html", &context) {
            Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
            Err(err) => {
                eprintln!("Template rendering error: {:?}", err);
                HttpResponse::InternalServerError().body("Error rendering template")
            }
        }
    } else {
        HttpResponse::Unauthorized().body("Please log in to perform trading")
    }
}


pub async fn order_history(
    id: Identity,
    tera: web::Data<Tera>,
    db_pool: web::Data<DbPool>
) -> impl Responder {
    if let Some(username) = id.identity() {
        let orders = db_pool.get_order_history(&username).unwrap();
        
        let mut context = Context::new();
        context.insert("username", &username);
        context.insert("orders", &orders);

        let s = tera.render("order_history.html", &context).unwrap();
        HttpResponse::Ok().content_type("text/html").body(s)
    } else {
        HttpResponse::Unauthorized().body("Please log in to view your order history")
    }
}

pub async fn check_balance(
    id: Identity,
    db_pool: web::Data<DbPool>
) -> impl Responder {
    if let Some(username) = id.identity() {
        let balance = db_pool.get_balance(&username).unwrap();
        HttpResponse::Ok().body(format!("Your balance is: ${:.2}", balance))
    } else {
        HttpResponse::Unauthorized().body("Please log in to check your balance")
    }
}

/*
pub async fn place_buy_order(
    form: web::Form<PlaceBuyOrderForm>,
    id: Identity,
    db_pool: web::Data<DbPool>,
) -> impl Responder {
    if let Some(username) = id.identity() {
        let _ = db_pool.place_buy_order(&username, &form.symbol, form.quantity as i32, form.max_price);
        // Trigger order matching
        let _ = db_pool.match_orders();
        HttpResponse::Found().append_header(("Location", "/trading")).finish()
    } else {
        HttpResponse::Unauthorized().body("Please log in to place an order")
    }
}
*/

/*
pub async fn place_sell_order(
    form: web::Form<PlaceSellOrderForm>,
    id: Identity,
    db_pool: web::Data<DbPool>,
) -> impl Responder {
    if let Some(username) = id.identity() {
        let _ = db_pool.place_sell_order(&username, &form.symbol, form.quantity as i32, form.min_price);
        // Trigger order matching
        let _ = db_pool.match_orders();
        HttpResponse::Found().append_header(("Location", "/trading")).finish()
    } else {
        HttpResponse::Unauthorized().body("Please log in to place an order")
    }
}
*/