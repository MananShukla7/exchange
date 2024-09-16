use serde::{Deserialize, Serialize};

use crate::db::types::Side;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateOrderMessage {
    pub market: String,
    pub price: String,
    pub quantity: String,
    pub side: Side,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CancelOrderMessage {
    pub order_id: String,
    pub market: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnRampMessage {
    pub amount: String,
    pub user_id: String,
    pub txn_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetDepthMessage {
    pub market: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetOpenOrdersMessage {
    pub user_id: String,
    pub market: String,
}

// #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
// pub enum Side {
//     Buy,
//     Sell,
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageToEngine {
    CreateOrder(CreateOrderMessage),
    CancelOrder(CancelOrderMessage),
    OnRamp(OnRampMessage),
    GetDepth(GetDepthMessage),
    GetOpenOrders(GetOpenOrdersMessage),
}
