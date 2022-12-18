use serde::{Deserialize, Serialize};

use crate::models::fees::Fee;
use crate::DbConnection;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFeeParams {
    #[serde(rename = "baseFee")]
    pub base_fee: f32,
    #[serde(rename = "pricePerUnit")]
    pub price_per_unit: f32,
    #[serde(rename = "monthlyDiscount")]
    pub monthly_discount: f32,
    #[serde(rename = "dateStart")]
    pub date_start: String,
    #[serde(rename = "dateEnd")]
    pub date_end: String,
}

#[tauri::command]
pub fn get_fees_list(conn: tauri::State<DbConnection>) -> Vec<Fee> {
    println!("get_fees_list called");
    let mut connection = conn.connection.lock().unwrap();
    if let Ok(fees) = Fee::list(&mut connection) {
        return fees;
    }

    return vec![];
}

#[tauri::command]
pub fn create_fee(
    conn: tauri::State<DbConnection>,
    params: CreateFeeParams,
) -> Result<Fee, String> {
    println!("received: {:?}", params);
    let mut connection = conn.connection.lock().unwrap();
    Fee::create(&mut connection, params)
}

#[tauri::command]
pub fn delete_fee(conn: tauri::State<DbConnection>, id: i32) {
    let mut connection = conn.connection.lock().unwrap();
    match Fee::delete(&mut connection, id) {
        Ok(()) => println!("deleted!"),
        Err(err) => println!("delete-error: {}", err),
    }
}

#[tauri::command]
pub fn find_in_time_range(
    conn: tauri::State<DbConnection>,
    date_start: String,
    date_end: String,
) -> Option<Fee> {
    println!("find_in_time_range called");
    let mut connection = conn.connection.lock().unwrap();
    Fee::find_in_time_range(&mut connection, date_start, date_end)
}
