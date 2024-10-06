use axum::{
    routing::{get, post},
    Router,
};
use controller::{get_info_handler, login_handler};

mod controller;
mod model;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/login", post(login_handler))
        .route("/info", get(get_info_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening.....! localhost:3000 🚀🚀🚀");

    axum::serve(listener, app).await.unwrap();
}
