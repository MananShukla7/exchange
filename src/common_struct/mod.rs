use std::sync::Mutex;

use once_cell::sync::OnceCell;
use redis::{aio::PubSub, Client, Commands, Connection, ConnectionLike};
use serde::{Deserialize, Serialize};
use rand::Rng;

use crate::types::MessageToEngine;

pub static REDIS_CONN:OnceCell<Mutex<RedisManager>>=OnceCell::new();
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
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
    pub publisher: Connection,
    pub subscriber: PubSub

}

impl RedisManager {
    pub async fn new() -> redis::RedisResult<Self> {
        let client = Client::open("redis://127.0.0.1/")?;
        let publisher = client.get_connection()?;
        let subscriber=client.get_async_pubsub().await?;
        Ok(RedisManager {
            client,
            publisher,
            subscriber
        })
    }

    pub async fn get_instance()->&'static std::sync::Mutex<RedisManager> {
        
           if REDIS_CONN.get().is_none(){
               
               let _res=REDIS_CONN.set(Mutex::new(RedisManager::new().await.unwrap()));
               REDIS_CONN.get().unwrap()

           } 
           else {
            REDIS_CONN.get().unwrap()
        }  
        
        
    }


    pub async fn send_and_await(&mut self, message: MessageToEngine) -> redis::RedisResult<String> {
        println!("connection to the redis is active");
        let id = get_random_client_id();
        println!("Sending message: {:?}", id);
        // Use the publisher to create a PubSub connection and subscribe
        if self.client.is_open(){
          
            // Prepare the message
            let msg_with_id = serde_json::json!({
                "clientId": id,
                "message": message
            });
            // Push the message using the main connection
            self.publisher.lpush("messages", msg_with_id.to_string())?;
            
            // Parse and return the response
            // let response: MessageFromOrderbook = serde_json::from_str(&payload).unwrap();
            let response=msg_with_id.to_string();
            println!("Response: {:?}", msg_with_id.to_string());
            Ok(response)
        }
        else {
            // Ok("Can't connect to redis".to_string())
            if let Err(error) = Box::pin(self.send_and_await(message)).await {
                return Err(error);
            }
            else {
                return Ok("Retrying...".to_string());
            }
        }


    }
}



fn get_random_client_id() -> String {
    let mut rng = rand::thread_rng();
    let id = rng.gen_range(1..=5);
    id.to_string()
}


#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ResponseMessage<T> {
    pub message:String,
    pub data:T
}