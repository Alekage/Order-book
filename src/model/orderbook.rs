#![allow(unused)]

use std::collections::VecDeque;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use std::sync::Mutex;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Pair {
    pair: String,
}

pub struct LimitOrder {
    id: u64,
    quantity: u64,
}

type Price = u64;

pub struct OrderBook {
    bid: BTreeMap<Price, VecDeque<LimitOrder>>,
    ask: BTreeMap<Price, VecDeque<LimitOrder>>,
}

#[derive(Clone)]
pub struct ExchangeModelController {
    exchange: Arc<Mutex<HashMap<Pair, OrderBook>>>,
}

impl ExchangeModelController {
    pub fn new() -> Self {
        ExchangeModelController {
            exchange: Arc::new(Mutex::new(HashMap::<Pair, OrderBook>::new())),
        }
    }
}
