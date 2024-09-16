use std::{cmp::min, collections::HashMap};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::db::types::Side;

use super::Engine::BASE_CURRENCY;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Fill {
    pub price: f64,
    pub quantity: i64,
    pub order_id: String,
    pub filled: i32,
    pub side: Side,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub price: f64,
    pub quantity: f64,
    pub order_id: String,
    pub filled: i32,
    pub side: Side,
    pub user_id: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderBook {
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
    pub base_asset: String,
    pub quote_asset: String,
    pub last_trade_id: i64,
    pub current_price: i32,
}

impl OrderBook {
    pub fn new(
        bids: Vec<Order>,
        asks: Vec<Order>,
        base_asset: String,
        last_trade_id: i64,
        current_price: i32,
    ) -> OrderBook {
        return OrderBook {
            bids,
            asks,
            base_asset,
            quote_asset: BASE_CURRENCY.to_string(),
            last_trade_id,
            current_price,
        };
    }

    pub fn ticker(&self) -> String {
        return format!("{}_{}", self.base_asset, self.quote_asset);
    }

    pub fn get_snapshot(&self) -> Value {
        let snap = json!(
            {
                "base_asset":self.base_asset,
                "bids":self.bids,
                "asks":self.asks,
                "last_trade_id":self.last_trade_id,
                "current_price":self.current_price
            }
        );
        return snap;
    }

    pub fn add_order(&mut self,order: Order)->(Vec<Fill>,i64){
        
        match order.side {
            Side::Buy => {
                let (fills,executed_qty)=self.match_bid(order.clone());
                if executed_qty==order.quantity as i64{
                    return (fills,executed_qty);
                }
                self.bids.push(order);
                return (fills,executed_qty);
            }
            Side::Sell => {
                let (fills,executed_qty)=self.match_ask(order.clone());
                if executed_qty==order.quantity as i64{
                    return (fills,executed_qty);
                }
                self.bids.push(order);
                return (fills,executed_qty);
            }
        }
    }
    pub fn match_bid(&mut self, order: Order)->(Vec<Fill>, i64) {
        let mut fills: Vec<Fill> = vec![];
        let mut executed_qty = 0;

        for i in 0..self.asks.len() {
            if self.asks[i].price <= order.price && executed_qty < order.quantity as i64 && self.asks[i].order_id != order.order_id {
                //ttaking min here because if a person wants to buy 2k solana and at a certauin price if there is 20 solana set for sale then , take its min
                let filled_qty = min(
                    order.quantity as i64 - executed_qty,
                    self.asks[i].quantity as i64,
                );
                executed_qty += filled_qty;
                self.asks[i].filled += filled_qty as i32;
                fills.push(Fill {
                    price: self.asks[i].price,
                    quantity: filled_qty,
                    order_id: self.asks[i].order_id.to_string(),
                    filled: self.asks[i].filled,
                    side: Side::Sell,
                    user_id: self.asks[i].user_id.to_string(),
                });
            }
        }
        for i in 0..self.asks.len() {
            if self.asks[i].filled == self.asks[i].quantity as i32 {
                self.asks.remove(i);
            }
        }
        return(fills,executed_qty);
        
    }

    pub fn match_ask(&mut self, order: Order) -> (Vec<Fill>, i64) {

        let mut fills: Vec<Fill> = vec![];
        let mut executed_qty = 0;

        for i in 0..self.bids.len(){
            if self.bids[i].price>=order.price && executed_qty<order.quantity as i64 && self.bids[i].order_id != order.order_id{
                let amt_remaining = min(
                    order.quantity as i64 - executed_qty,
                    self.bids[i].quantity as i64
                );
                executed_qty += amt_remaining;
                self.bids[i].filled += amt_remaining as i32;
                fills.push(Fill {
                    price: self.bids[i].price,
                    quantity: amt_remaining,
                    order_id: self.bids[i].order_id.to_string(),
                    filled: self.bids[i].filled,
                    side: Side::Buy,
                    user_id: self.bids[i].user_id.to_string(),
                });
            }
        }

        for i in 0..self.bids.len() {
            if self.bids[i].filled == self.bids[i].quantity as i32 {
                self.bids.remove(i);
            }
        }
        return (fills, executed_qty);

    }


    pub async fn get_depth(&self)->(Vec<(String, String)>,Vec<(String, String)>){
        let mut bids: Vec<(String, String)> = vec![];
        let mut asks: Vec<(String, String)> = vec![];

        let mut bids_map=HashMap::new();
        let mut asks_map=HashMap::new();
        //adding bids and ask pruice and the total num of bids ans asks at that price
        for i in 0..self.bids.len(){
            if bids_map.contains_key(&self.bids[i].price.to_string()){
                bids_map.insert(self.bids[i].price.to_string(), self.bids[i].quantity as i64 + bids_map.get(&self.bids[i].price.to_string()).unwrap());
            }
            else{
                bids_map.insert(self.bids[i].price.to_string(), self.bids[i].quantity as i64);
            }
        }

        for i in 0..self.asks.len(){
            if asks_map.contains_key(&self.asks[i].price.to_string()){
                asks_map.insert(self.asks[i].price.to_string(), self.asks[i].quantity as i64 + asks_map.get(&self.asks[i].price.to_string()).unwrap());
            }
            else {
                asks_map.insert(self.asks[i].price.to_string(), self.asks[i].quantity as i64);
            }
        }

        for (price, quantity) in asks_map.iter() {
            // println!("{} {}", price, quantity);
            asks.push((price.to_string(), quantity.to_string()));
        }
        for (price, quantity) in bids_map.iter() {
            // println!("{} {}", price, quantity);
            bids.push((price.to_string(), quantity.to_string()));
        }
        return (bids, asks);

    }

    pub async fn get_open_orders(&mut self,user_id:String)->Vec<Order>{
        let mut asks:Vec<Order>=self.asks.iter().filter(|x| x.user_id==user_id).cloned().collect();
        let bids:Vec<Order>=self.bids.iter().filter(|x| x.user_id==user_id).cloned().collect();
        asks.extend(bids.into_iter());
        return asks
    }
    pub async fn cancel_bid(&mut self,order_id:String)->Result<Order, String>{
        let index=self.bids.iter().position(|x| x.order_id==order_id);
        if index.is_none(){
            return Err("Order not found".to_string());
        }       
        let index=index.unwrap();
        let removed_order=self.bids.remove(index);
        return Ok(removed_order)
    }

    pub async fn cancel_ask(&mut self,order_id:String)->Result<Order, String>{
        let index=self.asks.iter().position(|x| x.order_id==order_id);
        if index.is_none(){
            return Err("Order not found".to_string());
        }       
        let index=index.unwrap();
        let removed_order=self.bids.remove(index);
        return Ok(removed_order)
    }


}
