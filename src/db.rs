use crate::{DnResult, RESET_DATABASE, DATABASE_PATH};
use rusqlite::{Connection, NO_PARAMS};
use std::{path::Path, fs::remove_file};
use tracing::info;

pub fn connect_to_database() -> DnResult<Connection> {
    
    let db_path = Path::new(DATABASE_PATH);

    if RESET_DATABASE {
        info!("Removing database");
        remove_file(db_path)?;
    }
    
    let is_empty = !db_path.exists();

    info!("Connected to database at {:?}", db_path);
    let mut conn = Connection::open(db_path)?;

    if is_empty {
        init_database(&mut conn)?;
    }

    Ok(conn)
}

fn init_database(conn: &mut Connection) -> DnResult<()> {
    info!("Initializing database");
    conn.execute(
        "create table if not exists cat_colors (
             id integer primary key,
             name text not null unique
         )",
        [],
    )?;

    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             name text not null,
             color_id integer not null references cat_colors(id)
         )",
        [],
    )?;

    Ok(())
}