
use db::{cron::cron_job, get_db_connection};
// use db::{cron::cron_job, get_db_connection};
use routes::get_all_routers;
use tokio::{io::{self, AsyncBufReadExt, AsyncRead, AsyncWrite}, net::{TcpListener, TcpStream}, task};
use futures_util::{stream::{SplitSink, SplitStream}, SinkExt, StreamExt};
// use redis::Commands;
use dotenvy::dotenv;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use ws::set_ws_stream;
pub mod common_struct;
pub mod database;
pub mod db;
pub mod depth;
pub mod klines;
pub mod order_book;
pub mod routes;
pub mod ticker;
pub mod trades;
pub mod types;
pub mod engine;
pub mod ws;

pub async fn register_name(write:&mut SplitSink<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>, Message>,name:String){
    let message=Message::Text(name);
    write.send(message).await.unwrap();
}

async fn handle_incoming_messages(mut read: SplitStream<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>>) {
    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => println!("Received a message: {}", msg),
            Err(e) => eprintln!("Error receiving message: {}", e),
        }
    }
}

async fn read_and_send_messages(mut write: SplitSink<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>, Message>) {
    let mut reader = io::BufReader::new(io::stdin()).lines();
    while let Some(line) = reader.next_line().await.expect("Failed to read line") {
        if !line.trim().is_empty() {
            write.send(Message::Text(line)).await.expect("Failed to send message");
        }
    }
}


#[tokio::main]
async fn main() {
    dotenv().expect("no .env file found");
    get_db_connection().await;
    //cron job to refresh the materialized view based on time
    // task::spawn(cron_job());
    let url="ws://localhost:6001";
    println!("Listening on: {}", url);

  
    
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let _app = axum::serve(listener, get_all_routers()).await.unwrap();
    // tokio::join!(read_handle,write_handle);
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
