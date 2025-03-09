use axum::Router;

use crate::models::orderbook::ExchangeModelController;

pub fn order_routes(controller: ExchangeModelController) -> Router {
    Router::new().route("/api/add-order", method_router).with_state(controller)
}