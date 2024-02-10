use rusqlite::{Connection, Result};

/** 创建数据库 */
pub fn create_db() -> Result<()> {
    let conn = Connection::open("bill.db")?;
    conn.execute(
        "create table if not exists bill (
             id integer primary key,
             date integer not null unique,
             total_amount integer not null
         )",
        (),
    )?;
    Ok(())
}
