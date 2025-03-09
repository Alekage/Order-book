use axum::extract::State;
use serde::Deserialize;
use axum::extract::Json;
use axum::extract::Path;

use crate::models::orderbook::{ExchangeModelController, Pair};

#[derive(Deserialize)]
pub struct LimitOrderPayload {
    pub id: u64,
    pub side: String,
    pub price: u64,
    pub quantity: u64,
}


pub async fn add_limit_order(Path(pair): Path<Pair>, State(controller): State<ExchangeModelController>, Json(payload): Json<LimitOrderPayload>) {
    controller.add_order(pair, payload);
}