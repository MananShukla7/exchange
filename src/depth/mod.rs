mod depth_service;
mod depth_types;
use axum::{routing::{get, post}, Router};
use depth_service::depth_service;

pub fn depthRouter()->Router{
    Router::new().route("/", get(depth_service))
}