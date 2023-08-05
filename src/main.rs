mod dbsetup;
mod model;
mod handler;
mod response;

use warp::{http::Method, Filter, Rejection};

type WebResult<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=info");
    }
    pretty_env_logger::init();

    // create database and tables
    dbsetup::setup()
        .expect("Failed to setup database.");

    // setup API routes with CORS
    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origins(vec!["http://localhost:3000/", "http://localhost:8000/"])
        .allow_headers(vec!["content-type"])
        .allow_credentials(true);

    let health_checker = warp::path!("api" / "healthchecker")
        .and(warp::get())
        .and_then(handler::health_checker_handler);

    let order_router = warp::path!("api" / "orders" / ..);
    let order_routes = order_router
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handler::create_order_handler)
        .or(order_router
            .and(warp::get())
            .and(warp::path::param())
            .and_then(handler::get_order_handler));

    let routes = order_routes
        .with(cors)
        .with(warp::log("api"))
        .or(health_checker);

    println!("🚀 Server started successfully");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
