use axum::routing::{get, post, Router};
use std::net::SocketAddr;

mod db;
mod options;
mod routes;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(routes::index))
        .route("/image", get(routes::deliver_file));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
