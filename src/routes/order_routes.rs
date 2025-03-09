use axum::Router;
use axum::routing::post;
use crate::handlers::orders::add_limit_order;
use crate::models::orderbook::ExchangeModelController;

pub fn order_routes(controller: ExchangeModelController) -> Router {
    Router::new().route("/api/add-order", post(add_limit_order)).with_state(controller)
}