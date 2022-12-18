#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use std::path::Path;
use std::sync::Mutex;

use rusqlite::Connection;
use tauri::generate_handler;

use crate::commands::consumption::{create_meter_reading, get_meter_readings};
use crate::commands::fees::{create_fee, delete_fee, find_in_time_range, get_fees_list};
use crate::db::connection::{establish_connection, run_migrations};

pub mod commands;
pub mod db;
pub mod models;

pub struct DbConnection {
    connection: Mutex<Connection>,
}

fn main() {
    let home_dir = dirs::home_dir();
    let home_dir = match home_dir {
        Some(dir) => dir,
        None => panic!("Can not read home dir"),
    };

    let home_dir_str = home_dir.into_os_string().into_string();
    let home_dir_str = match home_dir_str {
        Ok(str) => str,
        Err(_) => panic!("Can not read home dir"),
    };

    let database_folder = Path::new(home_dir_str.as_str()).join(".qum");
    fs::create_dir_all(&database_folder).expect("Can not create .qum folder");

    let database_folder = match database_folder.into_os_string().into_string() {
        Ok(path) => path,
        Err(_) => panic!("Can not read database_folder"),
    };

    let database_file = Path::new(database_folder.as_str())
        .join("qum")
        .with_extension("db");
    let database_file = match database_file.into_os_string().into_string() {
        Ok(path) => path,
        Err(_) => panic!("Can not read database file"),
    };

    println!("Connecting to {}", database_file);

    let connection = establish_connection(database_file);
    let mut connection = match connection {
        Ok(conn) => conn,
        Err(_) => panic!("failed to connect to database"),
    };

    run_migrations(&mut connection);

    tauri::Builder::default()
        .manage(DbConnection {
            connection: Mutex::new(connection),
        })
        .invoke_handler(generate_handler![
            get_fees_list,
            create_fee,
            delete_fee,
            find_in_time_range,
            get_meter_readings,
            create_meter_reading
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
