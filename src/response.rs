use crate::model::Order;
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct OrderData {
    pub order: Order,
}

#[derive(Serialize, Debug)]
pub struct SingleOrderResponse {
    pub status: String,
    pub data: OrderData,
}