use rusqlite::{Connection, Result};
use rand::Rng;

// Setup database and initialize with starter data for restaurant tables and menu items
pub fn setup() -> Result<()> {
    let conn = Connection::open("restaurant.db")?;

    create_tables(&conn)
        .expect("Failed to create database tables.");
    
    initialize_items(&conn)
        .expect("Failed to initialize items.");

    initialize_tables(&conn)
        .expect("Failed to initialize restaurant tables.");

    Ok(())
}

// creates db tables 
fn create_tables(conn: &Connection) -> Result<()> {
    // drop tables if they already exist so we can have fresh start (only for testing purposes)
    conn.execute("drop table if exists orders", ())?;
    conn.execute("drop table if exists items", ())?;
    conn.execute("drop table if exists tables", ())?;

    // create db tables to store item, table, and order information
    conn.execute(
        "create table if not exists items (
            item_id integer primary key,
            name text not null unique
        )",
        (),
    )?;
    conn.execute(
        "create table if not exists tables (
            table_id integer primary key,
            seats integer not null
        )",
        (),
    )?;
    conn.execute(
        "create table if not exists orders (
            order_id integer primary key,
            table_id integer not null references tables(table_id),
            item_id integer not null references items(item_id),
            cook_time_minutes integer not null
        )",
        (),
    )?;

    Ok(())
}

// inserts menu items into ITEMS table
fn initialize_items(conn: &Connection) -> Result<()> {
    let items = vec!["Fish", "Meat", "Spaghetti", "Bread"];

    for item in items {
        conn.execute(
            "insert into items (name) values (?1)",
            [item],        
        )?;
    }

    Ok(())
}

// inserts restaurant tables into TABLES table
fn initialize_tables(conn: &Connection) -> Result<()> {
    let mut rng = rand::thread_rng();

    for _number in 1..101 {
        conn.execute(
            "insert into tables (seats) values (?1)",
            [rng.gen_range(1..4)],
        )?;
    }

    Ok(())
}