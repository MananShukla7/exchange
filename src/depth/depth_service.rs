use axum::{
    extract::Query,
    response::{IntoResponse, Json},
    Router,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::types::{GetDepthMessage, MessageToEngine};

#[derive(Deserialize)]
struct DepthQuery {
    symbol: String,
}

#[derive(Serialize)]
struct ResponseMessage<T> {
    message: String,
    data: T,
}

pub async fn depth_service(Query(params): Query<DepthQuery>) -> impl IntoResponse {
    let redis_connection = match RedisManager::get_instance().await {
        Ok(conn) => conn,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ResponseMessage {
            message: "Failed to connect to Redis".to_string(),
            data: e.to_string(),
        })).into_response(),
    };

    let mut redis_connection = match redis_connection.lock() {
        Ok(conn) => conn,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ResponseMessage {
            message: "Failed to lock Redis connection".to_string(),
            data: e.to_string(),
        })).into_response(),
    };

    let depth = GetDepthMessage { market: params.symbol };
    match redis_connection.send_and_await(MessageToEngine::GetDepth(depth)).await {
        Ok(data) => (StatusCode::OK, Json(ResponseMessage {
            message: "Successfully got the depth".to_string(),
            data,
        })).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ResponseMessage {
            message: "Failed to get the depth".to_string(),
            data: error.to_string(),
        })).into_response(),
    }
}
