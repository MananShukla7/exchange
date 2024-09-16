use serde::{Deserialize,Serialize};

pub const CREATE_ORDER: &str = "CREATE_ORDER";
pub const CANCEL_ORDER: &str = "CANCEL_ORDER";
pub const ON_RAMP: &str = "ON_RAMP";
pub const GET_DEPTH: &str = "GET_DEPTH";
pub const GET_OPEN_ORDERS: &str = "GET_OPEN_ORDERS";

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct CreateOrderData {
    pub market: String,
    pub price: String,
    pub quantity: String,
    pub side: OrderSide,
    pub user_id: String,
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct CancelOrderData {
    pub order_id: String,
    pub market: String,
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct OnRampData {
    pub amount: String,
    pub user_id: String,
    pub txn_id: String,
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct GetDepthData {
    pub market: String,
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct GetOpenOrdersData {
    pub user_id: String,
    pub market: String,
}


#[derive(Debug,Serialize,Deserialize,Clone)]
pub enum MessageFromApi {
    CreateOrder(CreateOrderData),
    CancelOrder(CancelOrderData),
    OnRamp(OnRampData),
    GetDepth(GetDepthData),
    GetOpenOrders(GetOpenOrdersData),
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub enum OrderSide {
    Buy,
    Sell,
}