use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest,};
use mongodb::{bson::{doc, Document},  Client,};
use mongodb::bson::{ Bson};
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde_json::json;
use chrono::{Utc, Duration};
use actix_cors::Cors;  // เพิ่มการนำเข้า Cors
//use actix_files::Files;

//mod handler; 
//use crate::handler::upload_image; 
//mod websocket; // import WebSocket handler

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    phone: String,
    password: String,
    bank_account: String,
    balance: i64, 
    lottery_numbers: Option<Vec<Vec<i32>>>,
    
}


#[derive(Deserialize)]
struct RegisterRequest {
    name: String,
    phone: String,
    password: String,
    bank_account: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    phone: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    name: String,
    phone: String,
    bank_account: String, // เพิ่มฟิลด์นี้
    exp: usize, // expiration timestamp
}



const SECRET_KEY: &[u8] = b"your_secret_key";

async fn register(data: web::Json<RegisterRequest>, client: web::Data<Client>) -> impl Responder {
    let collection = client.database("test").collection::<User>("users");

    // ตรวจสอบว่ามีเบอร์โทรศัพท์หรือบัญชีธนาคารนี้ในระบบแล้วหรือไม่
    let filter = doc! {
        "$or": [
            { "phone": &data.phone },
            { "bank_account": &data.bank_account }
        ]
    };
    let existing_user = collection.find_one(filter, ).await.unwrap();

    if existing_user.is_some() {
        // ตอบกลับด้วยสถานะ 400 และข้อความแจ้งเตือน
        return HttpResponse::BadRequest().json(json!({
            "message": "เบอร์โทรศัพท์หรือบัญชีธนาคารนี้ได้ถูกใช้แล้ว กรุณาเข้าสู่ระบบ"
        }));
    }

    // เข้ารหัสรหัสผ่านด้วย bcrypt
    let hashed_password = hash(&data.password, DEFAULT_COST).unwrap();

    // สร้างเอกสารผู้ใช้ที่มีรหัสผ่านเข้ารหัสแล้ว
    let user_doc = User {
        name: data.name.clone(),
        phone: data.phone.clone(),
        password: hashed_password,
        bank_account: data.bank_account.clone(),
        balance: 0,
        lottery_numbers: None,
        
        
    };

    collection.insert_one(user_doc, ).await.unwrap();

    HttpResponse::Ok().json("Registered successfully!")
}


// ฟังก์ชันสำหรับเข้าสู่ระบบและสร้าง JWT
async fn login(data: web::Json<LoginRequest>, client: web::Data<Client>) -> impl Responder {
    let collection = client.database("test").collection::<User>("users");

    let filter = doc! { "phone": &data.phone };
    let user = collection.find_one(filter, ).await.unwrap();

    if let Some(user) = user {
        // ตรวจสอบรหัสผ่าน
        if verify(&data.password, &user.password).unwrap() {
            let expiration = Utc::now()
                .checked_add_signed(Duration::hours(24))
                .expect("valid timestamp")
                .timestamp() as usize;

            // สร้าง JWT
            let claims = Claims {
                name: user.name.clone(),
                phone: user.phone.clone(),
                bank_account: user.bank_account.clone(),
                exp: expiration,
            };

            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY)).unwrap();

            // ส่ง token กลับให้ client เก็บ
            return HttpResponse::Ok().json(json!({ "token": token }));
        }
    }
    HttpResponse::Unauthorized().json("Login failed!")
}

async fn home(req: HttpRequest, client: web::Data<Client>) -> impl Responder {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .map(|header| header.trim_start_matches("Bearer "))
        .unwrap_or("");

    // ตรวจสอบและถอดรหัส token
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    ) {
        Ok(token_data) => {
            let claims = token_data.claims;

            // ดึงข้อมูลจาก MongoDB ตาม phone หรือ bank_account ของผู้ใช้
            let collection = client.database("test").collection::<User>("users");
            let filter = doc! { "phone": claims.phone.clone() }; // ใช้ phone เพื่อค้นหาในฐานข้อมูล
            if let Ok(Some(user)) = collection.find_one(filter, ).await {
                // ส่งข้อมูลรวมถึง balance กลับไปยัง client
                return HttpResponse::Ok().json(json!({
                    "name": claims.name,
                    "phone": claims.phone,
                    "bank_account": claims.bank_account,
                    "balance": user.balance, // เพิ่ม balance ในการตอบกลับ
                }));
            }

            HttpResponse::NotFound().json("User not found")
        }
        Err(_) => HttpResponse::Unauthorized().json("Unauthorized access"),
    }
}






#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();

    HttpServer::new(move || {
        App::new()
            
            
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec!["Content-Type", "Authorization"])
                    .max_age(3600),
            )
            .app_data(web::Data::new(client.clone()))
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/home", web::get().to(home))
           
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
