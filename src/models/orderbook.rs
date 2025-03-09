#![allow(unused)]
use std::collections::VecDeque;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use std::sync::Mutex;

use crate::handlers::orders::LimitOrderPayload;

pub type Pair = String;
type Price = u64;

pub struct LimitOrder {
    id: u64,
    quantity: u64,
}

pub struct OrderBook {
    pub bid: BTreeMap<Price, VecDeque<LimitOrder>>,
    pub ask: BTreeMap<Price, VecDeque<LimitOrder>>,
}

#[derive(Clone)]
pub struct ExchangeModelController {
    pub exchange: Arc<Mutex<HashMap<Pair, OrderBook>>>,
}

impl ExchangeModelController {
    pub fn new() -> Self {
        ExchangeModelController {
            exchange: Arc::new(Mutex::new(HashMap::<Pair, OrderBook>::new())),
        }
    }

    pub fn add_order(&self, pair: Pair, lop: LimitOrderPayload ) {
        let mut exchange = self.exchange.lock().unwrap();

        let order_book = exchange.get_mut(&pair).unwrap();

        if lop.side == "bid" {
            order_book.bid.entry(lop.price).or_insert_with(|| {
                VecDeque::from(vec![LimitOrder {
                    id: lop.id,
                    quantity: lop.quantity,
                }])
            }).push_back(LimitOrder {
                id: lop.id,
                quantity: lop.quantity,
            });
        } else if lop.side == "ask" {
            order_book.ask.entry(lop.price).or_insert_with(|| {
                VecDeque::from(vec![LimitOrder {
                    id: lop.id,
                    quantity: lop.quantity,
                }])
            }).push_back(LimitOrder {
                id: lop.id,
                quantity: lop.quantity,
            });
        }
    }

    pub fn get_pairs(&self) -> Vec<Pair> {
        let exchange = self.exchange.lock().unwrap();

        let mut pairs = vec![];

        for pair in exchange.keys() {
            pairs.push(pair.clone());
        }

        pairs
    }
}
