use pretty_sqlite::print_table;
use rusqlite::Connection;
use rust_xp_sqlite::db_utils::create_schema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -- Memory SQLite
    let conn = Connection::open_in_memory()?;

    // --Create Schema
    create_schema(&conn)?;

    Ok(())
}
