mod db;

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_files::Files;
use serde::{Deserialize, Serialize};

use actix_identity::Identity;
use actix_session::CookieSession;
use actix_identity::{CookieIdentityPolicy, IdentityService};

use crate::db::DbPool;


use tera::{Tera, Context};

#[derive(Debug, Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct JwtResponse {
    token: String,
}

#[derive(Debug, Deserialize)]
struct SignupData {
    username: String,
    password: String,
}

async fn signup(tera: web::Data<Tera>) -> impl Responder {
    // Here you would add code to hash the password and save the user to a database
    let s = tera.render("signup.html", &Context::new()).unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}

async fn signup_process(
    form: web::Form<SignupData>,
    db_pool: web::Data<DbPool>,
    id: Identity
) -> impl Responder {
    let _ = db_pool.create_user(&form.username, &form.password);
    let login_result = db_pool.verify_user(&form.username, &form.password).unwrap();

    if login_result {
        id.remember(form.username.clone());
        HttpResponse::Found()
            .append_header(("Location", "/home?login=success"))
            .finish()
    } else {
        HttpResponse::Found()
            .append_header(("Location", "/signup?login=failed"))
            .finish()
    }
}


async fn login(/*data: web::Json<LoginData>,*/ tera: web::Data<Tera>) -> impl Responder {
    // Validate user and password, generate JWT token if valid
    let s = tera.render("login.html", &Context::new()).unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}

async fn login_process(
    form: web::Form<LoginData>,
    db_pool: web::Data<DbPool>,
    id: Identity
) -> impl Responder {
    let login_result = db_pool.verify_user(&form.username, &form.password).unwrap();

    if login_result {
        id.remember(form.username.clone());
        HttpResponse::Found()
            .append_header(("Location", "/home?login=success"))
            .finish()
    } else {
        HttpResponse::Found()
            .append_header(("Location", "/login?login=failed"))
            .finish()
    }
}


async fn home(tera: web::Data<Tera>) -> impl Responder {
    let s = tera.render("home.html", &Context::new()).unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}

async fn about(tera: web::Data<Tera>) -> impl Responder {
    let s = tera.render("about.html", &Context::new()).unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}

async fn history(tera: web::Data<Tera>) -> impl Responder {
    let s = tera.render("history.html", &Context::new()).unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}

async fn contact(tera: web::Data<Tera>) -> impl Responder {
    let s = tera.render("contact.html", &Context::new()).unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").unwrap();
    let db_pool = DbPool::new("mysql://root:morhano11@localhost:3306/test").unwrap();

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
            .service(Files::new("/static", "./static").show_files_listing())

            .route("/", web::get().to(login))
            .route("/home", web::get().to(home))
            .route("/about", web::get().to(about))
            .route("/history", web::get().to(history))
            .route("/contact", web::get().to(contact))
            .route("/signup", web::get().to(signup))
            .route("/signup_process", web::post().to(signup_process))
            .route("/login", web::get().to(login))
            .route("/login_process", web::post().to(login_process))
            //.service(Files::new("/static", "./static").show_files_listing()) // To serve static files
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
