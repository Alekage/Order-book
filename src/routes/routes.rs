use axum::Router;

use crate::models::orderbook::ExchangeModelController;

use super::{order_routes::order_routes, pair_routes::pair_routes};

pub fn routes(controller: ExchangeModelController) -> Router {
    Router::new().merge(pair_routes(controller.clone())).merge(order_routes(controller))
}