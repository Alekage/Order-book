use axum::routing::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/add-pair", get(add_pair));

    let listener = tokio::net::TcpListener::bind("0.0.0.3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}


async fn add_pair(pair: String) -> String {
    pair
}