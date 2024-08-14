pub const CREATE_ORDER: &str = "CREATE_ORDER";
pub const CANCEL_ORDER: &str = "CANCEL_ORDER";
pub const ON_RAMP: &str = "ON_RAMP";
pub const GET_DEPTH: &str = "GET_DEPTH";
pub const GET_OPEN_ORDERS: &str = "GET_OPEN_ORDERS";

#[derive(Debug)]
pub enum MessageFromApi {
    CreateOrder {
        market: String,
        price: String,
        quantity: String,
        side: OrderSide,
        user_id: String,
    },
    CancelOrder {
        order_id: String,
        market: String,
    },
    OnRamp {
        amount: String,
        user_id: String,
        txn_id: String,
    },
    GetDepth {
        market: String,
    },
    GetOpenOrders {
        user_id: String,
        market: String,
    },
}

#[derive(Debug)]
pub enum OrderSide {
    Buy,
    Sell,
}
