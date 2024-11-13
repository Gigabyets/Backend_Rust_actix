use actix_web::{HttpResponse, Responder,web};
use mongodb::{Client, bson::doc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub phone: String,
    pub image_url: String,
}

pub async fn get_users(client: web::Data<Client>) -> impl Responder {
    let collection = client.database("test").collection::<User>("users");

    let cursor = collection.find(None, None).await.unwrap();
    let users: Vec<User> = cursor.filter_map(|doc| {
        doc.ok().map(|user| User {
            name: user.get_str("name").unwrap().to_string(),
            phone: user.get_str("phone").unwrap().to_string(),
            image_url: user.get_str("image_url").unwrap().to_string(),
        })
    }).collect().await;

    HttpResponse::Ok().json(users)
}
