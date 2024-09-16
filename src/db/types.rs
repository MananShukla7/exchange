use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub enum DbMessage {
    TradeAdded(TradeAdd),
    OrderUpdate(OrderUpdate),
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct TradeAdd {
    pub id: String,
    pub is_buyer_maker: bool,
    pub price: String,
    pub quantity: String,
    pub quote_quantity: String,
    pub timestamp: i64,
    pub market: String,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct OrderUpdate {
    pub order_id: String,
    pub executed_qty: i64,
    pub market: Option<String>,
    pub price: Option<String>,
    pub quantity: Option<String>,
    pub side: Option<Side>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub enum Side {
    Buy,
    Sell,
}
