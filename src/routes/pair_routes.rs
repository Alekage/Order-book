use axum::Router;
use axum::routing::post;
use crate::models::orderbook::ExchangeModelController;
use crate::handlers::pairs::add_pair;


pub fn pair_routes(controller: ExchangeModelController) -> Router {
    Router::new()
        .route("/api/add-pair", post(add_pair))
        .with_state(controller)
}
