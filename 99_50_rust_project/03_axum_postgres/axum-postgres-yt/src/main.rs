use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use dotenv::dotenv;
use route::create_router;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod handler;
mod model;
mod route;
mod schema;

pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!(" ✅ Connection to the database is successfull");
            pool
        }
        Err(err) => {
            eprintln!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState { db: pool.clone() })).layer(cors);
    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
