use std::thread;

use sea_orm::{ConnectionTrait, DbBackend, Statement};

use super::DB_CONN;

pub async fn cron_job()->Result<(),String> {
    loop {
        
        let conn= match  DB_CONN.get() {
            Some(conn)=>conn,
            None=>return Err("DB not initialized".to_string())
        };
        if let Err(error)=  conn.execute(Statement::from_string(DbBackend::Postgres, "REFRESH MATERIALIZED VIEW klines_1m")).await{
            eprintln!("Error: {}", error);
            return Err(error.to_string());
        }
        if let Err(error)=  conn.execute(Statement::from_string(DbBackend::Postgres, "REFRESH MATERIALIZED VIEW klines_1h")).await{
            eprintln!("Error: {}", error);
            return Err(error.to_string());
        }if let Err(error)=  conn.execute(Statement::from_string(DbBackend::Postgres, "REFRESH MATERIALIZED VIEW klines_1w")).await{
            eprintln!("Error: {}", error);
            return Err(error.to_string());
        }
        println!("All of the materialized view refreshed");
        thread::sleep(std::time::Duration::from_secs(10));
    }
    
}

