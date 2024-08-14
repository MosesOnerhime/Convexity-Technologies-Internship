use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    fn new(username: String) -> Self {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::seconds(60 * 60))
            .expect("valid timestamp")
            .timestamp() as usize;

        Claims {
            sub: username,
            exp: expiration,
        }
    }
}

async fn signup(data: web::Json<SignupData>) -> impl Responder {
    // Hash the password
    let hashed_password = hash(&data.password, DEFAULT_COST).unwrap();

    // Save user with hashed password (here, just a simple print)
    println!("Saving user: {}, with password hash: {}", data.username, hashed_password);

    HttpResponse::Ok().body("User registered")
}

async fn login(data: web::Json<LoginData>) -> impl Responder {
    let password_is_correct = true; // You would check the password against your stored hash

    if password_is_correct {
        let claims = Claims::new(data.username.clone());
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret_key".as_ref())).unwrap();

        HttpResponse::Ok().json(JwtResponse { token })
    } else {
        HttpResponse::Unauthorized().body("Invalid credentials")
    }
}
