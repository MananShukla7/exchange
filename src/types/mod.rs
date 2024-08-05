pub struct CreateOrderMessage {
    pub market: String,
    pub price: String,
    pub quantity: String,
    pub side: Side,
    pub user_id: String,
}

pub struct CancelOrderMessage {
    pub order_id: String,
    pub market: String,
}

pub struct OnRampMessage {
    pub amount: String,
    pub user_id: String,
    pub txn_id: String,
}

pub struct GetDepthMessage {
    pub market: String,
}

pub struct GetOpenOrdersMessage {
    pub user_id: String,
    pub market: String,
}

pub enum Side {
    Buy,
    Sell,
}