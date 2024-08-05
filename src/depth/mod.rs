mod depth_service;
use axum::{routing::get, Router};
use depth_service::trial;

pub fn depthRouter()->Router{
    Router::new().route("/", get(trial))
}