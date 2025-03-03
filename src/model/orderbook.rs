#![allow(unused)]

use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::{HashMap, BTreeMap};


#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Pair {
    pair: String
}

pub struct LimitOrder {
    id: u64,
    quantity: u64
}

type Price = u64;

pub struct OrderBook {
    bid: BTreeMap<Price, VecDeque<LimitOrder>>,
    ask: BTreeMap<Price, VecDeque<LimitOrder>>
}

pub struct ExchangeModelController {
    exchange: Arc<Mutex<HashMap<Pair, OrderBook>>>
}