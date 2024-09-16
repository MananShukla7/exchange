// Assuming Order is defined in the `trade` module within `orderbook.rs`
// Define the path to the module here

use crate::engine::trade::order_book::Order;
use serde::{Deserialize, Serialize};

// Define constants equivalent to the action types in TypeScript
pub const CREATE_ORDER: &str = "CREATE_ORDER";
pub const CANCEL_ORDER: &str = "CANCEL_ORDER";
pub const ON_RAMP: &str = "ON_RAMP";

pub const GET_DEPTH: &str = "GET_DEPTH";

// Define the structures and enums based on the TypeScript code

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct DepthPayload {
    pub bids: Vec<(String, String)>,
    pub asks: Vec<(String, String)>,
}

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct OrderPlacedPayload {
    pub order_id: String,
    pub executed_qty: f64,
    pub fills: Vec<Fill>,
}

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Fill {
    pub price: String,
    pub qty: f64,
    pub trade_id: u64,
}

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct OrderCancelledPayload {
    pub order_id: String,
    pub executed_qty: f64,
    pub remaining_qty: f64,
}


#[derive(Debug, Clone,Serialize,Deserialize)]
pub enum MessageToApi {
    Depth {
        payload: DepthPayload,
    },
    OrderPlaced {
        payload: OrderPlacedPayload,
    },
    OrderCancelled {
        payload: OrderCancelledPayload,
    },
    OpenOrders {
        payload: Vec<Order>,
    },
}

