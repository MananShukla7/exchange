use axum::Router;
use serde::{Serialize,Deserialize};
pub fn klinesRouter() -> Router {
    
    Router::new()
}




#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct KlineRequest{
    pub market:String,
    pub interval:String,
    pub start_time:i64,
    pub end_time:i64
}