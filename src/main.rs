use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize, Debug)]
struct User {
    id: u32,
    name: String,
}

#[get("/users")]
async fn get_users() -> impl Responder {
    let users = [
        User { id: 1, name: "asep".to_string() },
        User { id: 2, name: "bokir".to_string() },
        User { id: 3, name: "budi".to_string() },
    ];

    web::Json(users)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_users))
        .bind("127.0.0.1:8080")?
        .run()
        .await
} 