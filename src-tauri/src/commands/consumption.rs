use serde::{Deserialize, Serialize};

use crate::{models::meter_reading::MeterReading, DbConnection};

#[derive(Serialize, Deserialize)]
pub struct CreateMeterReadingParams {
    pub value: f32,
    #[serde(rename = "feeId")]
    pub fee_id: i32,
    #[serde(rename = "readingDate")]
    pub reading_date: String,
}

#[tauri::command]
pub fn get_meter_readings(conn: tauri::State<DbConnection>) -> Vec<MeterReading> {
    println!("command: load meter readings");
    let mut connection = conn.connection.lock().unwrap();

    let list = MeterReading::list(&mut connection);
    if let Ok(measurements) = list {
        println!("found {} measurements", measurements.len());
        return measurements;
    }

    vec![]
}

#[tauri::command]
pub fn create_meter_reading(
    conn: tauri::State<DbConnection>,
    params: CreateMeterReadingParams,
) -> Result<MeterReading, String> {
    let mut connection = conn.connection.lock().unwrap();
    MeterReading::create(&mut connection, params)
}
