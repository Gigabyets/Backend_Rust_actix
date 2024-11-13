use actix_web::{web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::stream::StreamExt;
use std::fs::File;
use std::io::Write;
use mongodb::{bson, Client};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    phone: String,
    image_url: String,
}

#[derive(Serialize)]
struct UploadResponse {
    url: String,
    user_id: String,
}

pub async fn upload_image(mut payload: Multipart, client: web::Data<Client>, user_id: web::Path<String>) -> impl Responder {
    while let Some(item) = payload.next().await {
        match item {
            Ok(mut field) => {
                // ตรวจสอบ content_disposition ว่ามีหรือไม่
                if let Some(content_disposition) = field.content_disposition() {
                    // ตรวจสอบว่า content_disposition มีชื่อไฟล์หรือไม่
                    if let Some(file_name) = content_disposition.get_filename() {
                        let file_path = format!("./uploads/{}", file_name);
                        
                        // สร้างไฟล์เพื่อบันทึกข้อมูล
                        let mut file = match File::create(&file_path) {
                            Ok(f) => f,
                            Err(_) => return HttpResponse::InternalServerError().body("Failed to create file"),
                        };

                        // เขียนข้อมูลไฟล์ที่อัปโหลด
                        let mut chunk_data = Vec::new();
                        while let Some(Ok(chunk)) = field.next().await {
                            chunk_data.extend_from_slice(&chunk);
                        }

                        // เขียนข้อมูลทั้งหมดลงไฟล์
                        if let Err(_) = file.write_all(&chunk_data) {
                            return HttpResponse::InternalServerError().body("Failed to write file");
                        }

                        // สร้าง URL ของไฟล์ที่อัปโหลด
                        let image_url = format!("/uploads/{}", file_name);

                        // สร้าง User struct
                        let user = User {
                            name: "User Name".to_string(),
                            phone: "1234567890".to_string(),
                            image_url: image_url.clone(),
                        };

                        // แปลง User struct เป็น BSON Document
                        let user_doc = match bson::to_document(&user) {
                            Ok(doc) => doc,
                            Err(_) => return HttpResponse::InternalServerError().body("Failed to convert user to BSON document"),
                        };

                        // บันทึกข้อมูลลงใน MongoDB
                        let collection = client.database("test").collection("users");
                        match collection.insert_one(user_doc, ).await {
                            Ok(_) => (),
                            Err(_) => return HttpResponse::InternalServerError().body("Failed to insert user data into MongoDB"),
                        };

                        // ส่งกลับ URL ของไฟล์ที่อัปโหลด
                        return HttpResponse::Ok().json(UploadResponse {
                            url: image_url,
                            user_id: user_id.into_inner(),
                        });
                    } else {
                        return HttpResponse::BadRequest().body("File name missing in content disposition");
                    }
                } else {
                    return HttpResponse::BadRequest().body("Content disposition missing");
                }
            }
            Err(_) => return HttpResponse::BadRequest().body("Error reading form data"),
        }
    }

    HttpResponse::InternalServerError().body("File upload failed")
}
