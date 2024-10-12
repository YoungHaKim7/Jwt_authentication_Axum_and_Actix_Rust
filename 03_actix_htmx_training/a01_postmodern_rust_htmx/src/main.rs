use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>Hello, world! This is a title</h1><p>And this is a paragraph</p>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("localhost:8080 or 127.0.0.1 🚀 🚀 🚀 ");
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
