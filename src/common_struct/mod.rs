use redis::{Client, Commands, Connection, PubSub};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use rand::Rng;
use lazy_static::lazy_static;


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct MessageFromOrderbook {
    // Define the structure based on your needs
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct MessageToEngine {
    // Define the structure based on your needs
    pub msg:String
}

pub struct RedisManager {
    client: Client,
    publisher: Connection,
}

impl RedisManager {
    fn new() -> redis::RedisResult<Self> {
        let client = Client::open("redis://127.0.0.1/")?;
        let publisher = client.get_connection()?;
        Ok(RedisManager { client, publisher })
    }

    pub fn get_instance()->Arc<Mutex<RedisManager>> {
        lazy_static! {
            static ref INSTANCE: Arc<Mutex<RedisManager>> = Arc::new(Mutex::new(
                RedisManager::new().expect("Failed to create RedisManager")
            ));
        }
        
        INSTANCE.clone()
        
    }


    pub fn send_and_await(&mut self, message: MessageToEngine) -> redis::RedisResult<MessageFromOrderbook> {
        let id = get_random_client_id();
        println!("Sending message: {:?}", id);
        // Use the publisher to create a PubSub connection and subscribe
        let mut conn=self.client.get_connection()?;
        let mut pubsub = conn.as_pubsub();
        pubsub.subscribe(&id)?;
        
        // Prepare the message
        let msg_with_id = serde_json::json!({
            "clientId": id,
            "message": message
        });
        // Push the message using the main connection
        self.publisher.lpush("messages", msg_with_id.to_string())?;
        

        // Wait for and process the response
        let msg = pubsub.get_message()?;
        println!("here");

        println!("Received message: {:?}", msg);
        let payload: String = msg.get_payload()?;
        
        // Unsubscribe
        pubsub.unsubscribe(&id)?;
        
        // Parse and return the response
        let response: MessageFromOrderbook = serde_json::from_str(&payload).unwrap();
        println!("Response: {:?}", response);
        Ok(response)
    }
    
}


fn get_random_client_id() -> String {
    let mut rng = rand::thread_rng();
    let id = rng.gen_range(1..=5);
    id.to_string()
}