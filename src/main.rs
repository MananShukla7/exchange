use db::{cron::cron_job, get_db_connection};
// use db::{cron::cron_job, get_db_connection};
use routes::get_all_routers;
use tokio::{net::TcpListener, task};
// use redis::Commands;
use dotenvy::dotenv;
mod depth;
mod order_book;
mod routes;
mod trades;
mod ticker;
mod klines;
mod db;
mod types;
mod common_struct;
#[tokio::main]
async fn main() {

    dotenv().expect("no .env file found");
    get_db_connection().await;
    //cron job to refresh the materialized view based on time
    task::spawn(cron_job());
    let listener=TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let _app=axum::serve(listener, get_all_routers()).await.unwrap();

}


// fn fetch_an_integer() -> redis::RedisResult<isize> {
    // connect to redis
    // let client = redis::Client::open("redis://127.0.0.1/")?;
    // let c=Asybc
    // let mut con = client.get_multiplexed_async_connection_with_config().await?;
    // // throw away the result, just make sure it does not fail
    // let _: () = con.set("my_key", 42)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.

    // Example usage
    
// }

// 


    // println!("Hello, world!");
    // let x=fetch_an_integer().unwrap();
    // println!("{}",x);

    // let manager = RedisManager::get_instance();
    // let message = MessageToEngine  { msg:"heelo".to_string()};
    
    // if let Ok(response) = manager.lock().unwrap().send_and_await(message) {
    //     println!("Received response: {:?}", response);
    // } else {
    //     println!("Failed to send and receive message");
    // };