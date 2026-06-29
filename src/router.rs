use axum::Router;
use axum::middleware::from_fn_with_state;
use tower_http::cors::CorsLayer;

use crate::auth::validate_token;
use crate::mcp::http_service::build_mcp_http_service;
use crate::state::AppState;


pub fn build_router(state: AppState) -> Router {
    let mcp_service = build_mcp_http_service(&state);

    Router::new()
        .route_service("/mcp", mcp_service)
        .layer(from_fn_with_state(state.clone(), validate_token))
        .layer(CorsLayer::permissive())
}