mod apidoc;
pub mod handlers;
pub mod state;

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use apidoc::ApiDoc;
use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    routing::{get, post},
    BoxError, Router,
};
use state::AppState;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::{limit::RequestBodyLimitLayer, timeout::TimeoutLayer};
use tracing::info;
use utoipa::OpenApi;

use crate::handlers::verify_handler_request::verify_signature_handler;

pub async fn run() -> anyhow::Result<()> {
    shared::init_tracing();

    info!("Verifier service starting...");

    let state = Arc::new(AppState::default());

    let app = Router::new()
        .route("/api-docs/openapi.json", get(|| async { axum::Json(ApiDoc::openapi()) }))
        .route("/api/verify-signature", post(verify_signature_handler))
        .layer(RequestBodyLimitLayer::new(1024 * 1024 * 10))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (StatusCode::INTERNAL_SERVER_ERROR, format!("Unhandled error: {}", err))
                }))
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(10, Duration::from_secs(60))),
        )
        .with_state(state.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener =
        tokio::net::TcpListener::bind(&addr).await.with_context(|| format!("Failed to bind to address: {}", addr))?;

    info!("Verifier running at http://{}", addr);
    info!("API Doc at http://localhost:3000/api-docs/openapi.json");
    info!("Rate limit: 10 requests per minute per IP");

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap_or_else(|e| panic!("Server error: {}", e));

    Ok(())
}
