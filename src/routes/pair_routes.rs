use crate::handlers::pairs::{add_pair, get_pairs};
use crate::models::orderbook::ExchangeModelController;
use axum::routing::{post, get};
use axum::Router;

pub fn pair_routes(controller: ExchangeModelController) -> Router {
    Router::new()
        .route("/api/get-pairs", get(get_pairs))
        .route("/api/add-pair", post(add_pair))
        .with_state(controller)
}
