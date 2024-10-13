use actix_files as fs;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use tera::Tera;

mod words;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "templates/**/*";
        let tera = Tera::new(source).unwrap();
        tera
    };
}

#[get("/")]
async fn index() -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("message_from_rust", "Hello from Rust!");
    let page_content = TEMPLATES.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[get("/info")]
async fn info() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("info.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[get("/words")]
async fn words_endpoint() -> impl Responder {
    let (word1, word2) = words::get_random_word_pair();
    let mut context = tera::Context::new();
    context.insert("word1", &word1);
    context.insert("word2", &word2);
    let page_content = TEMPLATES.render("words.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[get("/word-pair")]
async fn word_pair_endpoint() -> impl Responder {
    let (word1, word2) = words::get_random_word_pair();
    let mut context = tera::Context::new();
    context.insert("word1", &word1);
    context.insert("word2", &word2);
    let page_content = TEMPLATES.render("word_pair.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("localhost:8080 or 127.0.0.1:8080 \t🚀 🚀 🚀 ");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(info)
            .service(words_endpoint)
            .service(word_pair_endpoint)
            .service(fs::Files::new("/assets", "./assets"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
