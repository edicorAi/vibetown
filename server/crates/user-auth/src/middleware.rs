use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::UserAuthService;
use crate::models::User;

/// Extract the session token from the `vt_session` cookie.
pub fn extract_session_cookie(req: &Request) -> Option<String> {
    let cookie_header = req.headers().get(header::COOKIE)?.to_str().ok()?;
    for cookie in cookie_header.split(';') {
        let cookie = cookie.trim();
        if let Some(value) = cookie.strip_prefix("vt_session=") {
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }
    None
}

/// Extract the session token from request headers (cookie or upgrade).
pub fn extract_session_from_headers(headers: &axum::http::HeaderMap) -> Option<String> {
    let cookie_header = headers.get(header::COOKIE)?.to_str().ok()?;
    for cookie in cookie_header.split(';') {
        let cookie = cookie.trim();
        if let Some(value) = cookie.strip_prefix("vt_session=") {
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }
    None
}

/// Middleware that requires authentication. Returns 401 if no valid session.
pub async fn require_auth(
    State(auth_service): State<Arc<UserAuthService>>,
    mut request: Request,
    next: Next,
) -> Response {
    // Skip auth if auth is not required
    if !auth_service.auth_required() {
        return next.run(request).await;
    }

    let Some(token) = extract_session_cookie(&request) else {
        return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
    };

    match auth_service.validate_session(&token).await {
        Ok(user) => {
            request.extensions_mut().insert(user);
            next.run(request).await
        }
        Err(_) => (StatusCode::UNAUTHORIZED, "Invalid or expired session").into_response(),
    }
}

/// Middleware that optionally loads user if a valid session exists.
/// Does not reject unauthenticated requests.
pub async fn optional_auth(
    State(auth_service): State<Arc<UserAuthService>>,
    mut request: Request,
    next: Next,
) -> Response {
    if let Some(token) = extract_session_cookie(&request) {
        if let Ok(user) = auth_service.validate_session(&token).await {
            request.extensions_mut().insert(user);
        }
    }
    next.run(request).await
}

/// Middleware that requires the authenticated user to be an admin.
pub async fn require_admin(
    State(auth_service): State<Arc<UserAuthService>>,
    request: Request,
    next: Next,
) -> Response {
    if !auth_service.auth_required() {
        return next.run(request).await;
    }

    let user = request.extensions().get::<User>();
    match user {
        Some(user) if user.is_admin => next.run(request).await,
        Some(_) => (StatusCode::FORBIDDEN, "Admin access required").into_response(),
        None => (StatusCode::UNAUTHORIZED, "Authentication required").into_response(),
    }
}
