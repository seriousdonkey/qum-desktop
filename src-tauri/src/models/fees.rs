use rusqlite::{params, Connection, Error};
use serde::{Deserialize, Serialize};

use crate::commands::fees::CreateFeeParams;

#[derive(Serialize, Deserialize, Debug)]
pub struct Fee {
    pub id: i32,
    #[serde(rename = "baseFee")]
    pub base_fee: f32,
    #[serde(rename = "pricePerUnit")]
    pub price_per_unit: f32,
    #[serde(rename = "monthlyDiscount")]
    pub monthly_discount: f32,
    #[serde(rename = "dateStart")]
    pub date_start: chrono::NaiveDateTime,
    #[serde(rename = "dateEnd")]
    pub date_end: chrono::NaiveDateTime,
}

impl Fee {
    pub fn create(conn: &mut Connection, fee: CreateFeeParams) -> Result<Fee, String> {
        let base_fee = fee.base_fee;
        let price_per_unit = fee.price_per_unit;
        let monthly_discount = fee.monthly_discount;
        let date_start = fee.date_start;
        let date_end = fee.date_end;

        let select_stmt = conn.prepare("SELECT id FROM fees WHERE (? BETWEEN date_start AND date_end) OR (? BETWEEN date_start AND date_end)");
        let mut select_stmt = match select_stmt {
            Ok(stmt) => stmt,
            Err(err) => panic!("{}", err),
        };

        let rows = select_stmt.query_map(params![date_start, date_end], |row| {
            row.get::<usize, i32>(0)
        });
        let rows = match rows {
            Ok(rows) => rows,
            Err(err) => panic!("{}", err),
        };

        if rows.count() > 0 {
            return Err("Fee already exist for date range".to_string());
        }

        conn.execute("INSERT INTO fees (base_fee, price_per_unit, monthly_discount, date_start, date_end) VALUES (?1, ?2, ?3, ?4, ?5)",
                 (base_fee, price_per_unit, monthly_discount, date_start, date_end)).unwrap();

        let last_id = conn.last_insert_rowid();
        let mut select_stmt = match conn.prepare("SELECT id, base_fee, price_per_unit, monthly_discount, date_start, date_end FROM fees WHERE id = ?") {
            Ok(stmt) => stmt,
            Err(err) => panic!("{}", err)
        };

        let rows = select_stmt.query_map(params![last_id], |row| {
            let id = row.get(0)?;
            let base_fee = row.get(1)?;
            let price_per_unit = row.get(2)?;
            let monthly_discount = row.get(3)?;

            let date_start: String = row.get(4)?;
            let date_start =
                chrono::NaiveDateTime::parse_from_str(date_start.as_str(), "%Y-%m-%dT%H:%M:%S%.fZ")
                    .unwrap();
            let date_end: String = row.get(5)?;
            let date_end =
                chrono::NaiveDateTime::parse_from_str(date_end.as_str(), "%Y-%m-%dT%H:%M:%S%.fZ")
                    .unwrap();

            let fee = Fee {
                id,
                base_fee,
                price_per_unit,
                monthly_discount,
                date_start,
                date_end,
            };

            Ok(fee)
        });

        let rows = match rows {
            Ok(rows) => rows,
            Err(err) => return Err(err.to_string()),
        };

        if let Some(last) = rows.last() {
            return match last {
                Ok(fee) => Ok(fee),
                Err(err) => Err(err.to_string()),
            };
        }

        Err("unknown error".to_string())
    }

    pub fn find_in_time_range(
        conn: &mut Connection,
        date_start: String,
        date_end: String,
    ) -> Option<Fee> {
        let mut select_stmt = match conn.prepare("SELECT id, base_fee, price_per_unit, monthly_discount, date_start, date_end FROM fees WHERE (? BETWEEN date_start AND date_end) OR (? BETWEEN date_start AND date_end)") {
            Ok(stmt) => stmt,
            Err(err) => panic!("{}", err)
        };

        let rows = select_stmt.query_map(params![date_start, date_end], |row| {
            let id = row.get(0)?;
            let base_fee = row.get(1)?;
            let price_per_unit = row.get(2)?;
            let monthly_discount = row.get(3)?;

            let date_start: String = row.get(4)?;
            let date_start =
                chrono::NaiveDateTime::parse_from_str(date_start.as_str(), "%Y-%m-%dT%H:%M:%S%.fZ")
                    .unwrap();
            let date_end: String = row.get(5)?;
            let date_end =
                chrono::NaiveDateTime::parse_from_str(date_end.as_str(), "%Y-%m-%dT%H:%M:%S%.fZ")
                    .unwrap();

            let fee = Fee {
                id,
                base_fee,
                price_per_unit,
                monthly_discount,
                date_start,
                date_end,
            };

            Ok(fee)
        });

        let mut rows = match rows {
            Ok(rows) => rows,
            Err(err) => return None,
        };

        let fee = match rows.next() {
            Some(fee) => fee,
            None => return None,
        };

        match fee {
            Ok(fee) => return Some(fee),
            Err(_) => return None,
        }
    }

    pub fn delete(conn: &mut Connection, id: i32) -> Result<(), Error> {
        let mut stmt = conn.prepare("DELETE FROM fees WHERE id = ?")?;
        stmt.execute([id])?;

        Ok(())
    }

    pub fn list(conn: &mut Connection) -> Result<Vec<Fee>, Error> {
        println!("Load Fee List");

        let mut stmt = conn.prepare(
            "SELECT id, base_fee, price_per_unit, monthly_discount, date_start, date_end FROM fees",
        )?;

        let fees_iter = stmt.query_map([], |row| {
            let id = row.get(0)?;
            let base_fee = row.get(1)?;
            let price_per_unit = row.get(2)?;
            let monthly_discount = row.get(3)?;

            let date_start: String = row.get(4)?;
            let date_start =
                chrono::NaiveDateTime::parse_from_str(date_start.as_str(), "%Y-%m-%dT%H:%M:%S%.fZ")
                    .unwrap();
            let date_end: String = row.get(5)?;
            let date_end =
                chrono::NaiveDateTime::parse_from_str(date_end.as_str(), "%Y-%m-%dT%H:%M:%S%.fZ")
                    .unwrap();

            let fee = Fee {
                id,
                base_fee,
                price_per_unit,
                monthly_discount,
                date_start,
                date_end,
            };
            Ok(fee)
        })?;

        let mut fees: Vec<Fee> = vec![];
        for fee in fees_iter {
            if let Ok(fee) = fee {
                fees.push(fee)
            }
        }

        Ok(fees)
    }
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use crate::{commands::fees::CreateFeeParams, db::connection::run_migrations};

    use super::Fee;

    #[test]
    fn list_is_empty() {
        let mut conn = Connection::open_in_memory().expect("could not create memory database");
        run_migrations(&mut conn);
        let fees = Fee::list(&mut conn).expect("failed to get fees list");
        assert_eq!(fees.len(), 0);
    }

    #[test]
    fn list_has_data() {
        let mut conn = Connection::open_in_memory().expect("could not create memory database");
        run_migrations(&mut conn);

        let base_fee: f32 = 10.0;
        let price_per_unit: f32 = 0.5;
        let monthly_discount: f32 = 45.0;
        let start_date = "2022-12-01T05:00:00.000Z".to_string();
        let end_date = "2022-12-01T05:00:00.000Z".to_string();
        conn.execute("INSERT INTO fees (base_fee, price_per_unit, monthly_discount, date_start, date_end) VALUES (?1, ?2, ?3, ?4, ?5)",
                 (base_fee, price_per_unit, monthly_discount, start_date, end_date)).expect("failed to save fee");

        let fees = Fee::list(&mut conn).expect("failed to get fees list");
        assert_eq!(fees.len(), 1);
        let fee = fees.get(0).expect("failed to retrieve fee");
        assert_eq!(fee.base_fee, base_fee);
        assert_eq!(fee.price_per_unit, price_per_unit);
        assert_eq!(fee.monthly_discount, monthly_discount);
    }

    #[test]
    fn create() {
        let mut conn = Connection::open_in_memory().expect("could not create memory database");
        run_migrations(&mut conn);

        let params = CreateFeeParams {
            base_fee: 10.0,
            price_per_unit: 0.5,
            monthly_discount: 45.0,
            date_start: "2022-12-01T05:00:00.000Z".to_string(),
            date_end: "2022-12-01T05:00:00.000Z".to_string(),
        };

        Fee::create(&mut conn, params).expect("failed to create fee");

        let last_inserted_id = conn.last_insert_rowid();
        assert_eq!(last_inserted_id, 1);
    }

    #[test]
    fn find_in_time_range() {
        let mut conn = Connection::open_in_memory().expect("could not create memory database");
        run_migrations(&mut conn);

        let base_fee: f32 = 10.0;
        let price_per_unit: f32 = 0.5;
        let monthly_discount: f32 = 45.0;
        let start_date = "2022-01-01T05:00:00.000Z".to_string();
        let end_date = "2022-12-01T05:00:00.000Z".to_string();
        conn.execute("INSERT INTO fees (base_fee, price_per_unit, monthly_discount, date_start, date_end) VALUES (?1, ?2, ?3, ?4, ?5)",
                 (base_fee, price_per_unit, monthly_discount, start_date, end_date)).expect("failed to save fee");

        let fee = Fee::find_in_time_range(
            &mut conn,
            "2022-03-01T05:00:00.000Z".to_string(),
            "2022-06-01T05:00:00.000Z".to_string(),
        );

        assert!(fee.is_some());
    }
}
