use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use mongodb::{Client, bson::doc};
use serde::{Deserialize, Serialize};
use actix_cors::Cors;

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    phone: String,
    password: String,
    bank_account: String,
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

async fn register(data: web::Json<RegisterRequest>, client: web::Data<Client>) -> impl Responder {
    // ระบุประเภทของเอกสารเป็น User
    let collection = client.database("test").collection::<User>("users");

    let user_doc = User {
        name: data.name.clone(),
        phone: data.phone.clone(),
        password: data.password.clone(),
        bank_account: data.bank_account.clone(),
    };

    collection.insert_one(user_doc, None).await.unwrap();

    HttpResponse::Ok().json("Registered successfully!")
}

async fn login(data: web::Json<LoginRequest>, client: web::Data<Client>) -> impl Responder {
    // ระบุประเภทของเอกสารเป็น User
    let collection = client.database("test").collection::<User>("users");

    let filter = doc! {
        "phone": &data.phone,
        "password": &data.password,
    };

    let user = collection.find_one(filter, None).await.unwrap();

    if let Some(_) = user {
        HttpResponse::Ok().json("Login successful!")
    } else {
        HttpResponse::Unauthorized().json("Login failed!")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin() // อนุญาตทุก origin (หรือสามารถตั้งเป็น origin เฉพาะที่ต้องการ)
                    .allow_any_method() // อนุญาตทุก HTTP method
                    .allow_any_header() // อนุญาตทุก header
            )
            .route("/register", web::post().to(register))
            .app_data(web::Data::new(client.clone()))
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    })
    .bind("locolhost:8080")?
    .run()
    .await
}
