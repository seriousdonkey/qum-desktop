use rusqlite::{Connection, Result};
use rusqlite_migration::{Migrations, M};

pub fn establish_connection(db_path: String) -> Result<Connection> {
    Connection::open(db_path)
}

pub fn run_migrations(conn: &mut Connection) {
    let migrations = Migrations::new(vec![
        M::up(
            "CREATE TABLE fees (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        base_fee REAL NOT NULL,
        price_per_unit REAL NOT NULL,
        monthly_discount REAL NOT NULL,
        date_start DATETIME NOT NULL,
        date_end DATETIME NOT NULL
      )",
        ),
        M::up(
            "CREATE TABLE meter_readings (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        value REAL NOT NULL,
        fee_id INTEGER NOT NULL,
          reading_date DATETIME NOT NULL,
          FOREIGN KEY (fee_id)
            REFERENCES fees (id)
      )",
        ),
    ]);

    match migrations.to_latest(conn) {
        Ok(()) => println!("migrations runs!"),
        Err(err) => panic!("{}", err.to_string()),
    }
}
