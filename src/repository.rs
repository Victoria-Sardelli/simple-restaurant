use crate::model::{Order, OrderNew};
use rusqlite::{Connection, Result};

/*
    Insert new row in ORDERS table using given order data and return generated order id
*/
pub fn insert_order(order: &OrderNew) -> Result<String> {
    let conn = Connection::open("restaurant.db")?;

    conn.execute(
        "INSERT INTO orders (table_id, item_id, cook_time_minutes) values (?1, ?2, ?3)",
        &[&order.table_id, &order.item_id, &order.cook_time_minutes],
    )?;
    let last_id: String = conn.last_insert_rowid().to_string();

    Ok(last_id)
}


/*
    Get row(s) from ORDERS table fulfilling given conditions return results as collection
*/
pub fn get_orders(order_id: i32, statement: &str) -> Result<Vec<Result<Order>>> {
    let conn = Connection::open("restaurant.db")?;

    let mut stmt = conn
        .prepare(statement)?;

    let orders_iter = stmt.query_map([order_id], |row| {
        Ok(Order {
            order_id: row.get(0)?,
            table_id: row.get(1)?,
            item_id: row.get(2)?,
            cook_time_minutes: row.get(3)?
        })
    }).unwrap();
    Ok(orders_iter.collect())
}


/*
    Delete row from ORDERS table using given order id and return number of deleted rwows
*/
pub fn delete_order(order_id: i32) -> Result<usize> {
    let conn = Connection::open("restaurant.db")?;

    let deleted_rows = conn.execute(
        "DELETE FROM orders WHERE order_id=?1",
        [order_id],
    )?;
    
    Ok(deleted_rows)
}