use crate::{model::{Order, OrderNew}, DB_FILENAME};
use rusqlite::{Connection, Result, Params};

/*
    Get row(s) from ORDERS table with given table id
*/
pub fn get_orders_for_table(table_id: i32) -> Result<Vec<Result<Order>>> {
    let sql_statement = "SELECT * from orders WHERE table_id=?1;";
    get_orders([table_id], sql_statement)
}


/*
    Get row(s) from ORDERS table with given order id
*/
pub fn get_order_by_id(order_id: i32) -> Result<Vec<Result<Order>>> {
    let sql_statement = "SELECT * from orders WHERE order_id=?1;";
    get_orders([order_id], sql_statement)
}


/*
    Execute given sql statement on ORDERS table and return resulting rows as collection of orders
*/
pub fn get_orders(params: impl Params, statement: &str) -> Result<Vec<Result<Order>>> {
    let conn = Connection::open(DB_FILENAME.get().expect("Database name not provided."))?;

    let mut stmt = conn
        .prepare(statement)?;

    let orders_iter = stmt.query_map(params, |row| {
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
    Insert new row into ORDERS table using given order data and return generated order id
*/
pub fn insert_order(order: &OrderNew) -> Result<String> {
    let conn = Connection::open(DB_FILENAME.get().expect("Database name not provided."))?;

    conn.execute(
        "INSERT INTO orders (table_id, item_id, cook_time_minutes) values (?1, ?2, ?3)",
        &[&order.table_id, &order.item_id, &order.cook_time_minutes],
    )?;
    let last_id: String = conn.last_insert_rowid().to_string();

    Ok(last_id)
}


/*
    Delete row from ORDERS table using given order id and return number of deleted rwows
*/
pub fn delete_order(order_id: i32) -> Result<usize> {
    let conn = Connection::open(DB_FILENAME.get().expect("Database name not provided."))?;

    let deleted_rows = conn.execute(
        "DELETE FROM orders WHERE order_id=?1",
        [order_id],
    )?;
    
    Ok(deleted_rows)
}