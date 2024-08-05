use axum::Router;

use crate::{depth::depthRouter, klines::klinesRouter, order_book::orderRouter, ticker::tickerRouter, trades::tradesRouter};

pub fn get_all_routers()->Router{
    Router::new().merge(depthRouter()).merge(klinesRouter()).merge(orderRouter()).merge(tickerRouter()).merge(tradesRouter())
}
