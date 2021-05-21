use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn index(_info: web::Path<()>) -> impl Responder {
    format!("Hello, World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Actix-Web, http://127.0.0.1:8080");

    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
