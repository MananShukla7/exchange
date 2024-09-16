use redis::{Client, Commands, Connection};
use tokio::sync::Mutex;
use once_cell::sync::OnceCell;

use crate::db::types::DbMessage;

use super::types::to_api::MessageToApi;

static REDIS_ENGINE_CONN: OnceCell<Mutex<RedisManagerEngine>> = OnceCell::new();

pub struct RedisManagerEngine {
    pub client: Client,
    pub publisher: Connection,
}

impl RedisManagerEngine {
    pub async fn new() -> redis::RedisResult<Self> {
        let client = Client::open("redis://127.0.0.1/")?;
        let publisher = client.get_connection()?;
        Ok(RedisManagerEngine {
            client,
            publisher,
        })
    }

    pub async fn get_instance() -> &'static tokio::sync::Mutex<RedisManagerEngine> {
        if REDIS_ENGINE_CONN.get().is_none() {
            let _res = REDIS_ENGINE_CONN.set(Mutex::new(RedisManagerEngine::new().await.unwrap()));
            REDIS_ENGINE_CONN.get().unwrap()
        } else {
            REDIS_ENGINE_CONN.get().unwrap()
        }
    }
    pub async fn push_message(&mut self, message: DbMessage) -> redis::RedisResult<String> {
        let msg = serde_json::to_string(&message).unwrap();
        self.publisher.lpush("db_processor", msg)?;
        println!("Message pushed successfully");
        Ok("Message pushed successfully".to_string())
    }

    pub async fn publish_message(&mut self,channel:String, message: String) -> redis::RedisResult<String> {
        self.publisher.publish(channel, message)?;
        println!("Message published successfully");
        Ok("Message published successfully".to_string())
    }

    pub async fn send_to_api(&mut self, message: MessageToApi) -> redis::RedisResult<String> {
        let msg = serde_json::to_string(&message).unwrap();
        self.publisher.lpush("api_processor", msg)?;
        println!("Message pushed successfully");
        Ok("Message pushed successfully".to_string())
    }
}