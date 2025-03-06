use axum::extract::State;
use axum::Json;
use crate::models::orderbook::{Pair, ExchangeModelController, OrderBook};
use std::collections::BTreeMap;

#[derive(serde::Deserialize)]
pub struct PairPayload {
    pub pair: Pair,
}

pub async fn add_pair(
    State(controller): State<ExchangeModelController>,
    Json(payload): Json<PairPayload>,
) {
    let mut exchange = controller.exchange.lock().unwrap();

    exchange.insert(
        payload.pair,
        OrderBook {
            bid: BTreeMap::new(),
            ask: BTreeMap::new(),
        },
    );
}