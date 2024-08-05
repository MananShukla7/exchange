pub mod cron;

use std::{env, time::Duration};
use cron::cron_job;
use sea_orm::{Database, DatabaseConnection};
use once_cell::sync::OnceCell;
use tokio::time::Interval;
// pub static DB_INITIALIZED: OnceCell<tokio::sync::Mutex<bool>> = OnceCell::new();
pub static DB_CONN: OnceCell<DatabaseConnection> = OnceCell::new();
pub async fn get_db_connection()->& 'static DatabaseConnection{
    let db_conn=DB_CONN.get();
    if let Some(db_conn) = db_conn{
        return db_conn;
    }
    else {
        let db_url=env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_conn=Database::connect(db_url).await.unwrap();
        DB_CONN.set(db_conn).unwrap();
        println!("db connected");
        return DB_CONN.get().unwrap();
    }
}

pub async fn refresh_view(){
    let cron_job_status=tokio::task::spawn(cron_job());
    // // println!("{:?}",cron_job_status);
    // if let Err(error) =  cron_job_status{
    //     println!("Error: {}", error.to_string());
    // }
    // else {
    //     println!("All of the materialized view refreshed");
    // }
    cron_job_status.await;
}

#[tokio::main]
pub async fn main(){
    refresh_view().await;
}