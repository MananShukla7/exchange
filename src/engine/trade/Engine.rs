pub const BASE_CURRENCY: &str = "INR";
use std::{collections::HashMap, error::Error};

use chrono::Utc;
use redis::{Commands, RedisError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::fs;

use crate::{
    db::{
        self,
        types::{DbMessage, OrderUpdate, Side, TradeAdd},
    },
    depth::depth_types::MessageFromApi,
    engine::redisManager::RedisManagerEngine,
    types::CreateOrderMessage,
};

use super::order_book::{Fill, Order, OrderBook};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserBalances {
    pub available: f64,
    pub locked: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Engine {
    order_book: Vec<OrderBook>,
    balances: HashMap<String, HashMap<String, UserBalances>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SnapShotData {
    pub orderbook: Vec<OrderBook>,
    pub balances: Vec<(String, HashMap<String, UserBalances>)>,
}

// pub struct OrderBook {
//     pub bids: Vec<Order>,
//     pub asks: Vec<Order>,
//     pub base_asset: String,
//     pub quote_asset: String,
//     pub last_trade_id: i64,
//     pub current_price: i32,
// }

impl Engine {
    pub async fn new() -> Engine {
        let order_book;
        let mut balances: HashMap<String, HashMap<String, UserBalances>> = HashMap::new();
        if let Ok(snapshot) = fs::read_to_string("./snapshot.json").await {
            let snapshot_data: SnapShotData = serde_json::from_str(&snapshot).unwrap();
            order_book = snapshot_data.orderbook;

            snapshot_data.balances.iter().for_each(|x| {
                balances.insert(x.0.to_string(), x.1.clone());
            });
        } else {
            order_book = vec![OrderBook::new(vec![], vec![], "TATA".to_string(), 0, 0)];
            balances = HashMap::new();
        }
        let mut engine = Engine {
            order_book,
            balances,
        };
        engine.set_base_balances();
        return engine;
        //interval code remaining need to de it where the actual engine code is used
    }

    pub fn set_base_balances(&mut self) {
        let base_currency = "INR";
        let base_amount = 10_000_000.0;

        let mut user_balances = HashMap::new();
        user_balances.insert(
            base_currency.to_string(),
            UserBalances {
                available: base_amount,
                locked: 0.0,
            },
        );
        user_balances.insert(
            "TATA".to_string(),
            UserBalances {
                available: base_amount,
                locked: 0.0,
            },
        );

        for user_id in ["1", "2", "5"].iter() {
            self.balances
                .insert(user_id.to_string(), user_balances.clone());
        }
    }

    async fn save_snapshot(&self) {
        let snap = SnapShotData {
            orderbook: self.order_book.clone(),
            balances: self
                .balances
                .clone()
                .into_iter()
                .map(|(key, val)| (key, val))
                .collect::<Vec<(String, HashMap<String, UserBalances>)>>(),
        };
        fs::write("./snapshot.json", serde_json::to_string(&snap).unwrap())
            .await
            .expect("Unable to write file");
    }
    async fn process(&self, message: MessageFromApi, client_id: String) {
        // match message {
        //     MessageFromApi::CreateOrder { market, price, quantity, side, user_id } => {

        // }
    }

    async fn create_order(&self, create_order: CreateOrderMessage) {
        let orderbook = self
            .order_book
            .iter()
            .find(|x| x.ticker() == create_order.market);
        if orderbook.is_none() {
            return;
        }
        let orderbook = orderbook.unwrap();
        let base_asset = create_order.market.split("_").collect::<Vec<&str>>()[0];
        let quote_asset = create_order.market.split("_").collect::<Vec<&str>>()[1];
    }

    async fn check_and_lock_funds(
        &mut self,
        create_order: CreateOrderMessage,
        base_asset: String,
        quote_asset: String,
    ) {
        match create_order.side {
            Side::Buy => {
                let user_balance = self
                    .balances
                    .entry(create_order.user_id.clone())
                    .or_insert_with(HashMap::new)
                    .entry(quote_asset.clone())
                    .or_insert_with(|| UserBalances {
                        available: 0.0,
                        locked: 0.0,
                    });

                let required_amount = create_order.quantity.parse::<f64>().unwrap()
                    * create_order.price.parse::<f64>().unwrap();

                if user_balance.available < required_amount {   
                    panic!("Insufficient funds");
                }

                user_balance.available -= required_amount;
                user_balance.locked += required_amount;
            }
            Side::Sell => {
                let user_balance = self
                    .balances
                    .entry(create_order.user_id.clone())
                    .or_insert_with(HashMap::new)
                    .entry(base_asset.clone())
                    .or_insert_with(|| UserBalances {
                        available: 0.0,
                        locked: 0.0,
                    });

                let required_amount = create_order.quantity.parse::<f64>().unwrap();

                if user_balance.available < required_amount {
                    panic!("Insufficient funds");
                }

                user_balance.available -= required_amount;
                user_balance.locked += required_amount;
            }
        }
    }

    async fn on_ramp(&mut self, user_id: String, amount: f64) {
        let user_balance = self.balances.get_mut(&user_id);
        if user_balance.is_none() {
            let mut hmap = HashMap::new();
            hmap.insert(
                BASE_CURRENCY.to_string(),
                UserBalances {
                    available: amount,
                    locked: 0.0,
                },
            );
            self.balances.insert(user_id, hmap);
        } else {
            let user_balance = user_balance.unwrap();
            user_balance.entry(BASE_CURRENCY.to_string()).and_modify(
                |user_balance: &mut UserBalances| {
                    user_balance.available += amount;
                },
            );
        }
    }

    async fn update_balance(
        &mut self,
        user_id: String,
        base_asset: String,
        quote_asset: String,
        side: Side,
        fills: Vec<Fill>,
    ) -> Result<String, String> {
        match side {
            Side::Buy => {
                fills.iter().for_each(|fill| {
                    let balance_hmap = self.balances.get_mut(&fill.user_id);
                    if balance_hmap.is_none() {
                        return;
                    }
                    let balance_hmap = balance_hmap.unwrap();

                    balance_hmap.entry(base_asset.to_string()).and_modify(
                        |user_balance: &mut UserBalances| {
                            //update base asset
                            user_balance.available =
                                user_balance.available + (fill.price * fill.quantity as f64);
                            user_balance.locked =
                                user_balance.locked - (fill.price * fill.quantity as f64);
                        },
                    );
                    // update quote asset balance
                    balance_hmap.entry(quote_asset.to_string()).and_modify(
                        |user_balance: &mut UserBalances| {
                            user_balance.available =
                                user_balance.available + (fill.price * fill.quantity as f64);
                            user_balance.locked =
                                user_balance.locked - (fill.price * fill.quantity as f64);
                        },
                    );
                })
            }
            Side::Sell => {
                fills.iter().for_each(|fill| {
                    let balance_hmap = self.balances.get_mut(&fill.user_id);
                    if balance_hmap.is_none() {
                        return;
                    }
                    let balance_hmap = balance_hmap.unwrap();

                    balance_hmap.entry(base_asset.to_string()).and_modify(
                        |user_balance: &mut UserBalances| {
                            //update base asset
                            user_balance.available =
                                user_balance.available - (fill.price * fill.quantity as f64);
                            user_balance.locked =
                                user_balance.locked + (fill.price * fill.quantity as f64);
                        },
                    );
                    // update quote asset balance
                    balance_hmap.entry(quote_asset.to_string()).and_modify(
                        |user_balance: &mut UserBalances| {
                            user_balance.available =
                                user_balance.available - (fill.price * fill.quantity as f64);
                            user_balance.locked =
                                user_balance.locked + (fill.price * fill.quantity as f64);
                        },
                    );
                })
            }
        }
        return Ok("Updated Balances".to_string());
    }

    async fn update_db_order(
        order: Order,
        executed_qty: i64,
        fills: Vec<Fill>,
        market: String,
    ) -> Result<String, String> {
        let update_order = DbMessage::OrderUpdate(OrderUpdate {
            order_id: order.order_id,
            executed_qty: executed_qty,
            market: Some(market),
            price: Some(order.price.to_string()),
            quantity: Some(order.quantity.to_string()),
            side: Some(order.side),
        });

        let mut conn: tokio::sync::MutexGuard<'_, RedisManagerEngine> =
            RedisManagerEngine::get_instance().await.lock().await;
        let res = conn.push_message(update_order).await;
        if res.is_err() {
            return Err(res.unwrap_err().to_string());
        }

        for fill in fills {
            let create_trade = DbMessage::OrderUpdate(OrderUpdate {
                order_id: fill.order_id.to_owned(),
                executed_qty: fill.quantity,
                market: None,
                price: None,
                quantity: None,
                side: None,
            });

            if let Err(error) = conn.push_message(create_trade).await {
                println!("Error in creating trade: {}", error);
                return Err(error.to_string());
            }
        }

        return Ok("Db message published".to_string());
    }

    // return (bids, asks);
    async fn send_update_and_depth_at(&self,price: String, market: String)->Result<(),String> {
        let order_book=self.order_book.iter().find(|order|{
            order.ticker() == market
        });

        if order_book.is_none(){
            return Err("Order book not found".to_string());
        }
        let order_book = order_book.unwrap();
        let depth=order_book.get_depth().await;
        let updated_bids:Vec<(String,String)>=depth.0.iter().filter(|x|{x.0==price}).cloned().collect();
        let updated_asks:Vec<(String,String)>=depth.1.iter().filter(|x|{x.0==price}).cloned().collect();
        let ub=if !updated_bids.is_empty(){updated_bids} else {vec![(price.to_owned(),"0".to_string())]};
        let ua=if !updated_asks.is_empty(){updated_asks} else {vec![(price,"0".to_string())]};
        match RedisManagerEngine::get_instance().await.lock().await.publish_message(format!("depth@{}",market), json!({
            "stream":format!("trade@{}",market),
            "data":{
                "a":ua,
                "b":ub,
                "e":"depth"
            }
        }).to_string()).await{
            Ok(res)=>{
                println!("{} ",res);
                eprintln!("powerfull")
            }
            Err(error)=>{
                return Err(error.to_string());
            }
        };
        Ok(())
    }

    async fn create_db_trades(fills: Vec<Fill>, market: String, user_id: String) ->Result<(),RedisError>{
        for fill in fills {
            let res=RedisManagerEngine::get_instance().await.lock().await.push_message(
                DbMessage::TradeAdded(TradeAdd {
                    market: market.to_string(),
                    id: fill.order_id.to_string(),
                    is_buyer_maker: fill.user_id == user_id,
                    price: fill.price.to_string(),
                    quantity: fill.quantity.to_string(),
                    quote_quantity: (fill.quantity*fill.price as i64).to_string(),
                    timestamp:Utc::now().timestamp_millis() ,
                })
            ).await?;

            println!("{} ",res);
        }
        Ok(())
    }

    
}


//on-ramp - convert the quote asset to the base asset
//off-ramp - convert the base asset to the quote asset
