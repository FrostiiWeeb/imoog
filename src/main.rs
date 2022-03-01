use axum::routing::{get, post, Router};
use std::net::SocketAddr;

mod routes;
mod db;
mod options;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(routes::index));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
