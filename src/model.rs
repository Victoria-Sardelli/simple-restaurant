use serde::{Deserialize, Serialize};

// item details stored in db (includes generated id)
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
    pub item_id: i32,
    pub name: String,
}

// table details stored in db (includes generated id) 
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Table {
    pub table_id: i32,
    pub seats: i32,
}

// order details stored in db (includes generated id)
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Order {
    pub order_id: i32,
    pub table_id: i32,
    pub item_id: i32,
    pub cook_time_minutes: i32,
}

// order details before storing in database (no id generated yet)
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OrderNew {
    pub table_id: i32,
    pub item_id: i32,
    pub cook_time_minutes: i32,
}

// order details received from POST request (no id or cook time generated yet)
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OrderJson {
    pub table_id: i32,
    pub item_id: i32,
}