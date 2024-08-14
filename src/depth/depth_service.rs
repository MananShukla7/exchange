use axum::{
    debug_handler,
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
    // let mut redis_connection = match RedisManager::get_instance().await.lock().await.unwrap() {
    //     Ok(conn) => conn,
    //     Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ResponseMessage {
    //         message: "Failed to connect to Redis".to_string(),
    //         data: e.to_string(),
    //     })).into_response(),
    // };

    let mut redis_connection = RedisManager::get_instance().await.lock().await;

    let depth = GetDepthMessage {
        market: params.symbol,
    };
    match redis_connection
        .send_and_await(MessageToEngine::GetDepth(depth))
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
