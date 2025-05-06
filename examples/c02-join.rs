use pretty_sqlite::{print_rows, print_table};
use rusqlite::Connection;
use rust_xp_sqlite::db_utils::create_schema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -- Memory SQLite
    let conn = Connection::open_in_memory()?;

    // --Create Schema
    create_schema(&conn)?;

    // -- Insert Org
    let mut stmt = conn.prepare(
        "INSERT INTO org (name) 
            VALUES (?1) RETURNING id",
    )?;

    let org_id = stmt.query_row(["Acme, Inc"], |row| row.get::<_, i64>(0))?;

    // -- Insert Person
    let names = &["Jen", "Bob", "Sally", "Mike"];
    for (idx, name) in names.iter().enumerate() {
        let org_id = if idx % 2 == 0 { Some(org_id) } else { None };
        conn.execute(
            "INSERT INTO person (name, org_id, yob)
                VALUES (?1, ?2, ?3)",
            (name, &org_id, 2000),
        )?;
    }

    // -- Select Join
    let select_sql = "SELECT person.id, person.name, person.yob, org.name as org_name
        FROM person
        INNER JOIN org ON person.org_id = org.id
        WHERE org.id = :org_id";
    let mut stmt = conn.prepare(select_sql)?;
    let mut rows = stmt.query(&[(":org_id", &org_id)])?;

    print_rows(rows)?;

    Ok(())
}
