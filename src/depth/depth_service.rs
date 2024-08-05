use axum::response::{IntoResponse, Response};

pub async fn trial()->Response{
    "Workiing".to_string().into_response()
}