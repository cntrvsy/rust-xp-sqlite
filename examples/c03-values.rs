use pretty_sqlite::{print_rows, print_table};
use rusqlite::{Connection, ToSql, types::Value};
use rust_xp_sqlite::db_utils::create_schema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -- Memory SQLite
    let conn = Connection::open_in_memory()?;

    // -- Create Schema
    create_schema(&conn)?;

    // -- Seed users
    let names = &["Jen", "Bob", "Sally", "Mike"];
    for name in names.iter() {
        let org_id: Option<i64> = None;
        conn.execute(
            "INSERT INTO person (name, org_id, yob)
                VALUES (?1, ?2, ?3)",
            (name, &org_id, 2000),
        )?;
    }

    // -- Update
    let nv_list: Vec<(String, Value)> = vec![
        //
        ("org_id".to_string(), Value::Integer(123)),
        ("name".to_string(), Value::Text("New name 111".to_string())),
    ];

    // high key just use a sql builder
    let (cols, vals): (Vec<String>, Vec<Value>) = nv_list.into_iter().unzip();
    let cols = cols
        .iter()
        .map(|col| format!("\"{}\" = ?", col))
        .collect::<Vec<_>>()
        .join(", ");

    let sql = format!("UPDATE person SET {cols}");
    let mut values: Vec<&dyn ToSql> = vals.iter().map(|x| x as &dyn ToSql).collect();

    //build the where clause
    let sql = format!("{sql} WHERE id = ?");
    let person_id = Value::Integer(1);
    values.push(&person_id);

    // execute it
    let num_of_rows = conn.execute(&sql, &*values)?;
    println!("Number of rows updated: {}", num_of_rows);

    print_table(&conn, "person")?;

    Ok(())
}
