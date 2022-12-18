use crate::commands::consumption::CreateMeterReadingParams;

use super::fees::Fee;
use chrono::NaiveDateTime;
use rusqlite::{params, Connection, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MeterReading {
    pub id: i32,
    pub value: f32,
    pub fee: Fee,
    pub date: chrono::NaiveDateTime,
}

fn parse_datetime(datetime: String) -> NaiveDateTime {
    chrono::NaiveDateTime::parse_from_str(datetime.as_str(), "%Y-%m-%dT%H:%M:%S%.fZ").unwrap()
}

impl MeterReading {
    pub fn list(conn: &mut Connection) -> Result<Vec<MeterReading>, Error> {
        println!("models: get list of measurements");
        let sql = "SELECT m.id, m.value, m.reading_date, f.id, f.base_fee, f.price_per_unit, f.monthly_discount, f.date_start, f.date_end FROM meter_readings m LEFT JOIN fees f ON f.id = m.fee_id";
        let mut stmt = conn.prepare(sql)?;

        let measurements_iter = stmt.query_map([], |row| {
            let meter_reading_id = row.get(0)?;
            let meter_reading_value = row.get(1)?;
            let meter_reading_date: String = row.get(2)?;
            let meter_reading_date = parse_datetime(meter_reading_date);

            let fee_id = row.get(3)?;
            let fee_base_fee = row.get(4)?;
            let fee_price_per_unit = row.get(5)?;
            let fee_monthly_discount = row.get(6)?;

            let fee_start_date: String = row.get(7)?;
            let fee_start_date = parse_datetime(fee_start_date);

            let fee_end_date: String = row.get(8)?;
            let fee_end_date = parse_datetime(fee_end_date);

            let fee = Fee {
                id: fee_id,
                base_fee: fee_base_fee,
                price_per_unit: fee_price_per_unit,
                monthly_discount: fee_monthly_discount,
                date_start: fee_start_date,
                date_end: fee_end_date,
            };

            let meter_reading = MeterReading {
                id: meter_reading_id,
                value: meter_reading_value,
                date: meter_reading_date,
                fee,
            };

            Ok(meter_reading)
        })?;

        let mut meter_readings: Vec<MeterReading> = vec![];
        for meter_reading in measurements_iter {
            if let Ok(meter_reading) = meter_reading {
                meter_readings.push(meter_reading);
            }
        }

        Ok(meter_readings)
    }

    pub fn create(
        conn: &mut Connection,
        meter_reading: CreateMeterReadingParams,
    ) -> Result<MeterReading, String> {
        conn.execute(
            "INSERT INTO meter_readings (value, fee_id, reading_date) VALUES (?, ?, ?)",
            (
                meter_reading.value,
                meter_reading.fee_id,
                meter_reading.reading_date,
            ),
        )
        .unwrap();

        let last_id = conn.last_insert_rowid();
        let mut select_stmt = match conn.prepare("SELECT m.id, m.value, m.reading_date, f.id, f.base_fee, f.price_per_unit, f.monthly_discount, f.date_start, f.date_end FROM meter_readings m LEFT JOIN fees f ON f.id = m.fee_id WHERE m.id = ?") {
            Ok(stmt) => stmt,
            Err(err) => panic!("{}", err)
        };

        let rows = select_stmt.query_map(params![last_id], |row| {
            let meter_reading_id = row.get(0)?;
            let meter_reading_value = row.get(1)?;
            let meter_reading_date: String = row.get(2)?;
            let meter_reading_date = parse_datetime(meter_reading_date);

            let fee_id = row.get(3)?;
            let fee_base_fee = row.get(4)?;
            let fee_price_per_unit = row.get(5)?;
            let fee_monthly_discount = row.get(6)?;

            let fee_start_date: String = row.get(7)?;
            let fee_start_date = parse_datetime(fee_start_date);

            let fee_end_date: String = row.get(8)?;
            let fee_end_date = parse_datetime(fee_end_date);

            let fee = Fee {
                id: fee_id,
                base_fee: fee_base_fee,
                price_per_unit: fee_price_per_unit,
                monthly_discount: fee_monthly_discount,
                date_start: fee_start_date,
                date_end: fee_end_date,
            };

            let meter_reading = MeterReading {
                id: meter_reading_id,
                value: meter_reading_value,
                date: meter_reading_date,
                fee,
            };

            Ok(meter_reading)
        });

        let rows = match rows {
            Ok(rows) => rows,
            Err(err) => return Err(err.to_string()),
        };

        if let Some(last) = rows.last() {
            return match last {
                Ok(meter_reading) => Ok(meter_reading),
                Err(err) => Err(err.to_string()),
            };
        }

        Err("unknown error".to_string())
    }
}
