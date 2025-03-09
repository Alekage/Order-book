use models::orderbook::ExchangeModelController;
use routes::routes::routes;

mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let controller = ExchangeModelController::new();

    let app = routes(controller);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
