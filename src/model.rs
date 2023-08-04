use serde::{Deserialize, Serialize};

// item details to be read from db
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
    pub item_id: i32,
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Table {
    pub seats: i32,
}

// order details to insert in db
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Order {
    pub table_id: i32,
    pub item_id: i32,
    pub cook_time_minutes: i32,
}

// order details from POST request
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OrderNew {
    pub table_id: i32,
    pub item_id: i32,
}