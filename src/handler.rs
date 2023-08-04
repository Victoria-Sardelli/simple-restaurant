use crate::{
    model::{Order, OrderNew},
    response::{GenericResponse, SingleOrderResponse, OrderData},
    WebResult,
};
use rusqlite::{Connection, Result};
use warp::{http::StatusCode, reply::json, reply::with_status, Reply};
use rand::Rng;

// Health check endpoint to confirm server is reachable and responsive
pub async fn health_checker_handler() -> WebResult<impl Reply> {
    const MESSAGE: &str = "Healthy";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(json(response_json))
}

// Store new order in database using order data from request
pub async fn create_order_handler(body: OrderNew) -> WebResult<impl Reply> {
    // prepare order details to save
    let mut rng = rand::thread_rng();
    let order = Order {
        table_id: body.table_id,
        item_id: body.item_id,
        cook_time_minutes: rng.gen_range(5..16),
    };

    // store order details in db and respond with result of operation
    match insert_order(&order) {
        Ok(_) => {
            let json_response = SingleOrderResponse {
                status: "success".to_string(),
                data: OrderData { order },
            };
            return Ok(with_status(json(&json_response), StatusCode::CREATED))
        },
        Err(error) => {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("Could not create order: {error}"),
            };
            return Ok(with_status(json(&error_response), StatusCode::CONFLICT));
        }
    }
}

// Insert new row in ORDERS table using given order data
fn insert_order(order: &Order) -> Result<()> {
    let conn = Connection::open("restaurant.db")?;

    conn.execute(
        "INSERT INTO orders (table_id, item_id, cook_time_minutes) values (?1, ?2, ?3)",
        &[&order.table_id, &order.item_id, &order.cook_time_minutes],
    )?;

    Ok(())
}
