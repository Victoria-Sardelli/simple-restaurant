use crate::{
    model::{Order, OrderNew, OrderJson},
    response::{GenericResponse, SingleOrderResponse, OrderListResponse, OrderData},
    repository,
    WebResult,
};
use warp::{http::StatusCode, reply::json, reply::with_status, Reply};
use rand::Rng;

/*
    Health check endpoint to confirm server is reachable and responsive
*/
pub async fn health_check_handler() -> WebResult<impl Reply> {
    const MESSAGE: &str = "Healthy";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(json(response_json))
}

/*
    Get all orders for table with gien table id
*/
pub async fn get_orders_for_table_handler(table_id: i32) -> WebResult<impl Reply> {
    // query database for orders with given table id
    match repository::get_orders_for_table(table_id) {
        Ok(orders) => {
            let order_data_list: Vec<OrderData> = orders.into_iter().map(|o| { 
                OrderData {
                    order: o.unwrap()
                }
            })
            .collect();
            let json_response = OrderListResponse {
                status: "success".to_string(),
                results: order_data_list.len(),
                orders: order_data_list
            };
            return Ok(with_status(json(&json_response), StatusCode::OK));
        },
        Err(error) => {
            // if error querying database, return error message to user
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("Could not get orders for table with id: {table_id}. Error: {error}"),
            };
            return Ok(with_status(json(&error_response), StatusCode::CONFLICT));
        }
    }
}

/*
    Get order stored in database using order id
*/
pub async fn get_order_handler(order_id: i32) -> WebResult<impl Reply> {
    // query database for order with given order id
    match repository::get_order_by_id(order_id) {
        Ok(orders) => {
            // if order successfuly found with given id, return order data
            for order in orders {
                let json_response = SingleOrderResponse {
                    status: "success".to_string(),
                    data: OrderData { order: order.unwrap() },
                };
                return Ok(with_status(json(&json_response), StatusCode::OK));
            }
            // if no orders found with given id, return error message
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("No order exists with given ID: {order_id}"),
            };
            return Ok(with_status(json(&error_response), StatusCode::CONFLICT));
        },
        Err(error) => {
            // if error querying database, return error message to user
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("Could not get order with id: {order_id}. Error: {error}"),
            };
            return Ok(with_status(json(&error_response), StatusCode::CONFLICT));
        }
    }
}


/*
    Delete order with given order id
*/
pub async fn delete_order_handler(order_id: i32) -> WebResult<impl Reply> {
    match repository::delete_order(order_id) {
        Ok(deleted_rows) => {
            let json_response = GenericResponse {
                status: "success".to_string(),
                message: format!("Deleted {deleted_rows} order(s) with id: {order_id}"),
            };
            return Ok(with_status(json(&json_response), StatusCode::OK))
        },
        Err(error) => {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("Could not delete order with id: {order_id}. Error: {error}"),
            };
            return Ok(with_status(json(&error_response), StatusCode::CONFLICT));
        }
    }

}


/*
    Store new order in database using order data from request
*/
pub async fn create_order_handler(body: OrderJson) -> WebResult<impl Reply> {
    // prepare order details to save
    let mut rng = rand::thread_rng();
    let order_new = OrderNew {
        table_id: body.table_id,
        item_id: body.item_id,
        cook_time_minutes: rng.gen_range(5..16),
    };

    // store order details in db and respond with result of operation
    match repository::insert_order(&order_new) {
        Ok(order_id) => {
            let order = Order {
                order_id: order_id.parse::<i32>().unwrap(),
                table_id: order_new.table_id,
                item_id: order_new.item_id,
                cook_time_minutes: order_new.cook_time_minutes
            };
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