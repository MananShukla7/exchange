use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};

use crate::{
    common_struct::RedisManager,
    types::{GetDepthMessage, MessageToEngine},
};

#[derive(Deserialize)]
pub struct DepthQuery {
    symbol: String,
}

#[derive(Serialize)]
struct ResponseMessage<T> {
    message: String,
    data: T,

}

pub async fn depth_service(Query(params): Query<DepthQuery>) -> impl IntoResponse { 
    // println!("depth_service called");
    // let mut redis_connection = match RedisManager::get_instance().await.lock().await.unwrap() {
    //     Ok(conn) => conn,
    //     Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ResponseMessage {
    //         message: "Failed to connect to Redis".to_string(),
    //         data: e.to_string(),
    //     })).into_response(),
    // };

    let mutex_guard = RedisManager::get_instance().await.lock().await;
    let mutex_guard = mutex_guard;
    let mut redis_connection: tokio::sync::MutexGuard<'_, RedisManager> = mutex_guard;
    

    let depth = GetDepthMessage {
        market: params.symbol,
    };
    match redis_connection
        .send_and_await(MessageToEngine::GetDepth(depth),None)
        .await
    {
        Ok(data) => (
            StatusCode::OK,
            Json(ResponseMessage {
                message: "Successfully got the depth".to_string(),
                data,
            }),
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseMessage {
                message: "Failed to get the depth".to_string(),
                data: error.to_string(),
            }),
        )
            .into_response(),
    }
}
