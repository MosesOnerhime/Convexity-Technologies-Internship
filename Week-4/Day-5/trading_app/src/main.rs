// src/main.rs
mod controllers;
mod db;

use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::CookieSession;
use actix_web::{web, App, HttpServer};
use tera::Tera;
use db::DbPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").unwrap();
    let db_pool = DbPool::new("mysql://Admin:morhano11@localhost:3306/trading_app").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(false),
            ))
            .service(fs::Files::new("/static", "./static").show_files_listing()) // Serve static files
            .route("/", web::get().to(controllers::login_form))
            .route("/login", web::post().to(controllers::login_process))
            .route("/create_user", web::get().to(controllers::create_user_form))
            .route("/create_user_process", web::post().to(controllers::create_user_process))
            .route("/trading", web::get().to(controllers::trading_form))
            .route("/trade", web::post().to(controllers::trade_process))
            .route("/order_history", web::get().to(controllers::order_history))
           
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
