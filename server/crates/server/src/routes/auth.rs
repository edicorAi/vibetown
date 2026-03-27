use axum::{
    Extension, Json, Router,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use deployment::Deployment;
use serde::{Deserialize, Serialize};
use user_auth::{UserAuthService, models::User};

use crate::DeploymentImpl;

// ── Request/Response types ──

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LdapLoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct OidcCallbackQuery {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: User,
}

#[derive(Debug, Serialize)]
pub struct SetupStatusResponse {
    pub has_users: bool,
    pub auth_required: bool,
    pub providers: Vec<user_auth::providers::ProviderInfo>,
}

// ── Helpers ──

fn set_session_cookie(token: &str, max_age: i64) -> String {
    let secure = std::env::var("VT_AUTH_COOKIE_SECURE")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);

    let secure_flag = if secure { "; Secure" } else { "" };
    format!(
        "vt_session={token}; HttpOnly; SameSite=Lax; Path=/; Max-Age={max_age}{secure_flag}"
    )
}

fn clear_session_cookie() -> String {
    "vt_session=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0".to_string()
}

fn extract_client_info(headers: &HeaderMap) -> (Option<String>, Option<String>) {
    let ip = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or("").trim().to_string())
        .or_else(|| {
            headers
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
                .map(String::from)
        });

    let ua = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    (ip, ua)
}

fn get_auth_service(deployment: &DeploymentImpl) -> Result<&UserAuthService, Response> {
    deployment.user_auth().ok_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "Authentication not enabled"})),
        )
            .into_response()
    })
}

// ── Route handlers ──

/// GET /api/auth/status — auth configuration and setup status
async fn auth_status(
    State(deployment): State<DeploymentImpl>,
) -> Json<SetupStatusResponse> {
    match deployment.user_auth() {
        Some(auth) => {
            let has_users = auth.has_users().await.unwrap_or(false);
            let providers = auth.list_providers();
            Json(SetupStatusResponse {
                has_users,
                auth_required: true,
                providers,
            })
        }
        None => Json(SetupStatusResponse {
            has_users: false,
            auth_required: false,
            providers: vec![],
        }),
    }
}

/// POST /api/auth/login — local email+password login
async fn login(
    State(deployment): State<DeploymentImpl>,
    headers: HeaderMap,
    Json(body): Json<LoginRequest>,
) -> Result<Response, Response> {
    let auth = get_auth_service(&deployment)?;
    let (ip, ua) = extract_client_info(&headers);

    let (user, token) = auth
        .login_local(&body.email, &body.password, ip.as_deref(), ua.as_deref())
        .await
        .map_err(|e| e.into_response())?;

    let cookie = set_session_cookie(&token, auth.config().session_ttl);

    Ok((
        [(header::SET_COOKIE, cookie)],
        Json(AuthResponse { user }),
    )
        .into_response())
}

/// POST /api/auth/ldap/login — LDAP username+password login
async fn ldap_login(
    State(deployment): State<DeploymentImpl>,
    headers: HeaderMap,
    Json(body): Json<LdapLoginRequest>,
) -> Result<Response, Response> {
    let auth = get_auth_service(&deployment)?;
    let (ip, ua) = extract_client_info(&headers);

    let (user, token) = auth
        .login_ldap(&body.username, &body.password, ip.as_deref(), ua.as_deref())
        .await
        .map_err(|e| e.into_response())?;

    let cookie = set_session_cookie(&token, auth.config().session_ttl);

    Ok((
        [(header::SET_COOKIE, cookie)],
        Json(AuthResponse { user }),
    )
        .into_response())
}

/// GET /api/auth/oidc/:provider/login — initiate OIDC flow
async fn oidc_login(
    State(deployment): State<DeploymentImpl>,
    Path(provider): Path<String>,
) -> Result<Response, Response> {
    let auth = get_auth_service(&deployment)?;
    let (auth_url, _state) = auth
        .oidc_authorization_url(&provider)
        .await
        .map_err(|e| e.into_response())?;

    Ok(Redirect::temporary(&auth_url).into_response())
}

/// GET /api/auth/oidc/:provider/callback — OIDC callback
async fn oidc_callback(
    State(deployment): State<DeploymentImpl>,
    Path(provider): Path<String>,
    Query(query): Query<OidcCallbackQuery>,
    headers: HeaderMap,
) -> Result<Response, Response> {
    let auth = get_auth_service(&deployment)?;
    let (ip, ua) = extract_client_info(&headers);

    let (_user, token) = auth
        .oidc_callback(
            &provider,
            &query.code,
            &query.state,
            ip.as_deref(),
            ua.as_deref(),
        )
        .await
        .map_err(|e| e.into_response())?;

    let cookie = set_session_cookie(&token, auth.config().session_ttl);

    // After OIDC callback, redirect to the app root with the cookie set
    Ok((
        [(header::SET_COOKIE, cookie)],
        Redirect::temporary("/"),
    )
        .into_response())
}

/// POST /api/auth/logout — revoke session and clear cookie
async fn logout(
    State(deployment): State<DeploymentImpl>,
    headers: HeaderMap,
) -> Result<Response, Response> {
    let auth = get_auth_service(&deployment)?;

    // Extract token from cookie
    if let Some(cookie_header) = headers.get(header::COOKIE).and_then(|v| v.to_str().ok()) {
        for cookie in cookie_header.split(';') {
            let cookie = cookie.trim();
            if let Some(value) = cookie.strip_prefix("vt_session=") {
                let _ = auth.logout(value).await;
                break;
            }
        }
    }

    let cookie = clear_session_cookie();
    Ok(([(header::SET_COOKIE, cookie)], StatusCode::OK).into_response())
}

/// GET /api/auth/me — get current user
async fn me(
    Extension(user): Extension<User>,
) -> Json<AuthResponse> {
    Json(AuthResponse { user })
}

/// GET /api/auth/providers — list configured providers
async fn providers(
    State(deployment): State<DeploymentImpl>,
) -> Result<Json<Vec<user_auth::providers::ProviderInfo>>, Response> {
    let auth = get_auth_service(&deployment)?;
    Ok(Json(auth.list_providers()))
}

// ── Router ──

pub fn router() -> Router<DeploymentImpl> {
    Router::new()
        .route("/auth/status", get(auth_status))
        .route("/auth/login", post(login))
        .route("/auth/ldap/login", post(ldap_login))
        .route("/auth/oidc/{provider}/login", get(oidc_login))
        .route("/auth/oidc/{provider}/callback", get(oidc_callback))
        .route("/auth/logout", post(logout))
        .route("/auth/me", get(me))
        .route("/auth/providers", get(providers))
}
