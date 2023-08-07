mod dbsetup;
mod model;
mod handler;
mod response;
mod repository;

use warp::{http::Method, Filter, Rejection};
use serde::{Deserialize, Serialize};
use confy::{self, ConfyError};
use once_cell::sync::OnceCell;

type WebResult<T> = std::result::Result<T, Rejection>;

// database filename used when initializing and interacting with database
pub static DB_FILENAME: OnceCell<String> = OnceCell::new();

/*
    Applicaion configuration settings
*/
#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    db_filename: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            db_filename: "restaurant.db".to_string(),
        }
    }
}

/*
    Loads application configuration file
*/
fn load_app_config() -> Result<AppConfig, ConfyError> {
    let cfg: AppConfig = confy::load_path("src/config/app_config.toml")?;
    Ok(cfg)
}


#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=info");
    }
    pretty_env_logger::init();

    let cfg = load_app_config().expect("Could not load application configuration.");

    // create database and tables
    DB_FILENAME.set(cfg.db_filename).unwrap();
    dbsetup::setup(DB_FILENAME.get().expect("Database name not provided."))
        .expect("Failed to setup database.");

    // setup API routes with CORS
    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origins(vec!["http://localhost:3000/", "http://localhost:8000/"])
        .allow_headers(vec!["content-type"])
        .allow_credentials(true);

    let health_checker = warp::path!("api" / "health")
        .and(warp::get())
        .and_then(handler::health_check_handler);

    let order_router = warp::path!("api" / "orders" / ..);
    let order_routes = order_router
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handler::create_order_handler)
        .or(order_router
            .and(warp::path("tables"))
            .and(warp::get())
            .and(warp::path::param::<i32>())
            .and_then(handler::get_orders_for_table_handler))
        .or(order_router
            .and(warp::get())
            .and(warp::path::param::<i32>())
            .and_then(handler::get_order_handler))
        .or(order_router
            .and(warp::delete())
            .and(warp::path::param::<i32>())
            .and_then(handler::delete_order_handler));

    let routes = order_routes
        .with(cors)
        .with(warp::log("api"))
        .or(health_checker);

    println!("🚀 Server started successfully");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}