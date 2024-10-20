mod db;
mod handlers;
mod models;
mod routes;
use warp::Filter;

#[tokio::main]
async fn main() {
    db::initialize_db();
    let routes = routes::restaurant_routes();

    println!("Running the server");
    warp::serve(routes.)
}
