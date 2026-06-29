use std::net::SocketAddr;

use axum::extract::{ConnectInfo, Request, State};
use axum::http::{StatusCode, header::AUTHORIZATION};
use axum::middleware::Next;
use axum::response::Response;
use tracing::{info, warn};

use crate::state::AppState;

#[derive(Clone)]
pub struct AuthUser(pub String);

pub async fn validate_token(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let presented = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(strip_bearer);

    let Some(token) = presented else {
        warn!(ip = %addr, reason = "missing_token", "unauthorized request");
        return Err(StatusCode::UNAUTHORIZED);
    };

    let username = state.config.auth_users.get(token);

    match username {
        Some(user) => {
            info!(ip = %addr, user = %user, "authenticated request");
            req.extensions_mut().insert(AuthUser(user.clone()));
            Ok(next.run(req).await)
        }
        None => {
            warn!(ip = %addr, token = %token, reason = "invalid_token", "unauthorized request");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

fn strip_bearer(value: &str) -> Option<&str> {
    let value = value.trim();

    if let Some(stripped) = value.strip_prefix("Bearer ") {
        Some(stripped.trim())
    } else {
        None
    }
}