use rusqlite::{params, Connection, Result};

pub fn query() -> Result<String> {
    let db_path = "OfferDB.db";
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM OfferDB")?;
    let count: i64 = stmt.query_row(params![], |row| row.get(0))?;
    println!("Total number of observations in Offer table: {}", count);
    Ok("Success".to_string())
}