use axum::Router;

use crate::model::orderbook::ExchangeModelController;

pub fn pair_routes(controller: ExchangeModelController) -> Router {
    Router::new()
        .route("api/add-pair", add_pair)
        .with_state(controller)
}
