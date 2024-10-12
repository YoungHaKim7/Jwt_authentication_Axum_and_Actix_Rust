use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use tera::Tera;

// lazy_static! {
//     pub static ref TEMPLATES: Tera = {
//         let source = "templates/**/*";
//         let mut tera = match Tera::new(source) {
//             Ok(t) => t,
//             Err(e) => {
//                 eprintln!("Parsing error(s): {}", e);
//                 ::std::process::exit(1);
//             }
//         };
//         tera.autoescape_on(vec![".html", ".sql"]);
//         tera.register_filter("do_nothing", do_nothing_filter);
//         tera
//     };
// }

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
