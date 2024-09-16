use tokio::sync::Mutex;

use once_cell::sync::OnceCell;
use rand::Rng;
use redis::{aio::{MultiplexedConnection, PubSub}, AsyncCommands, Client, Commands, Connection, ConnectionLike};
use serde::{Deserialize, Serialize};

use crate::types::MessageToEngine;

pub static REDIS_CONN: OnceCell<Mutex<RedisManager>> = OnceCell::new();
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageFromOrderbook {
    // Define the structure based on your needs
}

// #[derive(Debug)]
// #[derive(Serialize, Deserialize)]
// pub struct MessageToEngine {
//     // Define the structure based on your needs
//     pub msg:String
// }

pub struct RedisManager {
    pub client: Client,
    pub publisher: MultiplexedConnection,
    // pub subscriber: PubSub,
}

impl RedisManager {
    pub async fn new() -> redis::RedisResult<Self> {
        let client = Client::open("redis://127.0.0.1/")?;
        let publisher = client.get_multiplexed_async_connection().await?;
        // let subscriber = client.get_async_pubsub().await?;
        Ok(RedisManager {
            client,
            publisher,
            // subscriber,
        })
    }

    pub async fn get_instance() -> &'static tokio::sync::Mutex<RedisManager> {
        if REDIS_CONN.get().is_none() {
            let _res = REDIS_CONN.set(Mutex::new(RedisManager::new().await.unwrap()));
            REDIS_CONN.get().unwrap()
        } else {
            REDIS_CONN.get().unwrap()
        }
    }

    pub async fn send_and_await(&mut self, message: MessageToEngine,mut retry:Option<i32>) -> redis::RedisResult<String> {
        if retry.is_none(){
            retry=Some(1);
        }
        else {
            retry=Some(retry.unwrap()+1);
        }
        println!("connection to the redis is active");
        let id = get_random_client_id();
        println!("Sending message: {:?}", id);
        // Use the publisher to create a PubSub connection and subscribe
        if self.client.is_open() {
            // Prepare the message
            let msg_with_id = serde_json::json!({
                "clientId": id,
                "message": message
            });
            // Push the message using the main connection
            self.publisher.lpush("messages", msg_with_id.to_string()).await?;

            // Parse and return the response
            // let response: MessageFromOrderbook = serde_json::from_str(&payload).unwrap();
            let response = msg_with_id.to_string();
            println!("Response: {:?}", msg_with_id.to_string());
            Ok(response)
        } else {
            if retry.unwrap()>=5{
                return Ok("Can't connect to redis after retries".to_string().into());
            }
            // Ok("Can't connect to redis".to_string())
            if let Err(error) = Box::pin(self.send_and_await(message,retry)).await {
                return Err(error);
            } else {
                return Ok("Retrying...".to_string());
            }
        }
    }
    pub fn trial(&mut self, message: MessageToEngine,mut retry:Option<i32>){

    }
}

fn get_random_client_id() -> String {
    let mut rng = rand::thread_rng();
    let id = rng.gen_range(1..=5);
    id.to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseMessage<T> {
    pub message: String,
    pub data: T,
}
