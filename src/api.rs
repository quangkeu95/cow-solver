use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use cow_offchain_shared::http_solver::model::SettledBatchAuctionModel;
use serde::Serialize;
use std::net::SocketAddr;
use tokio::task::JoinHandle;

pub async fn health_check() -> (StatusCode, Json<String>) {
    (StatusCode::OK, Json("OK".to_string()))
}

pub async fn solve() -> (StatusCode, Json<SettledBatchAuctionModel>) {
    (StatusCode::OK, Json(SettledBatchAuctionModel::default()))
}

pub async fn serve(bind_address: &SocketAddr) -> eyre::Result<()> {
    tracing::info!(%bind_address, "Serving HTTP server");

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/solve", post(solve));

    let server = axum::Server::bind(bind_address).serve(app.into_make_service());
    server.await.map_err(Into::into)
}
