pub mod cron;
pub mod types;

use crate::{common_struct::RedisManager, database::tata_prices};
use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use cron::cron_job;
use once_cell::sync::OnceCell;
use redis::Commands;
use sea_orm::ActiveModelTrait;
use sea_orm::{Database, DatabaseConnection, Set};
use std::env;
use types::DbMessage;
// pub static DB_INITIALIZED: OnceCell<tokio::sync::Mutex<bool>> = OnceCell::new();
pub static DB_CONN: OnceCell<DatabaseConnection> = OnceCell::new();
pub async fn get_db_connection() -> &'static DatabaseConnection {
    let db_conn = DB_CONN.get();
    if let Some(db_conn) = db_conn {
        return db_conn;
    } else {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_conn = Database::connect(db_url).await.unwrap();
        DB_CONN.set(db_conn).unwrap();
        println!("db connected");
        return DB_CONN.get().unwrap();
    }
}

pub async fn refresh_view() {
    let cron_job_status = tokio::task::spawn(cron_job());
    cron_job_status.await;
}

pub async fn trade_adder() {
    let redis_manager = RedisManager::get_instance().await;

    let response: Result<String, redis::RedisError> =
        redis_manager.lock().await.client.rpop("db_processor", None);
    let response = serde_json::from_str(&response.unwrap());
    if response.is_ok() {
        let data = response.unwrap();
        match data {
            DbMessage::TradeAdded(data) => {
                println!("Adding the recent trade data to db");
                println!("{:?}", data);
                let price = data.price.to_owned();
                let timestamp = data.timestamp;

                // Convert timestamp to DateTime<FixedOffset>
                let datetime: DateTime<FixedOffset> = Utc
                    .timestamp_millis_opt(timestamp)
                    .unwrap()
                    .with_timezone(&FixedOffset::east_opt(0).unwrap());

                let db_client = get_db_connection().await;
                let tata_price = tata_prices::ActiveModel {
                    time: Set(datetime),
                    price: Set(Some(price.parse::<f64>().unwrap())),
                    ..Default::default()
                };

                let res = tata_price.save(db_client).await.unwrap();
                dbg!(res);
                // let res=
                // println!("{:?}", res);
            }
            DbMessage::OrderUpdate(data) => {
                println!("Adding the recent order data to db")
            }
        }
    }
}
