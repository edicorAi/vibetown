use std::sync::Arc;

use axum::{
    Router,
    routing::{IntoMakeService, get},
};
use deployment::Deployment;
use tower_http::{compression::CompressionLayer, validate_request::ValidateRequestHeaderLayer};
use user_auth::UserAuthService;

use crate::{DeploymentImpl, middleware};

pub mod admin;
pub mod approvals;
pub mod auth;
pub mod config;
pub mod containers;
pub mod filesystem;
// pub mod github;
pub mod attachments;
pub mod events;
pub mod execution_processes;
pub mod feed;
pub mod frontend;
pub mod health;
pub mod mail;
pub mod migration;
pub mod oauth;
pub mod orchestration;
pub mod organizations;
pub mod releases;
pub mod remote;
pub mod repo;
pub mod scratch;
pub mod search;
pub mod sessions;
pub mod tags;
pub mod terminal;
pub mod work_items;
pub mod workspaces;

pub fn router(deployment: DeploymentImpl) -> IntoMakeService<Router> {
    // Build the auth service Arc for middleware state
    let auth_service: Arc<UserAuthService> = if let Some(auth) = deployment.user_auth() {
        // We need to create a new instance since we can't Arc a reference.
        // The deployment owns the service; we create a shared reference via cloning pool.
        let config = auth.config().clone();
        Arc::new(UserAuthService::new(config, auth.pool().clone()))
    } else {
        // Create a no-op auth service (auth not required)
        let config = user_auth::config::AuthConfig::default();
        Arc::new(UserAuthService::new(config, deployment.db().pool.clone()))
    };

    // Public routes — no auth required
    let public_routes = Router::new()
        .route("/health", get(health::health_check))
        .route("/ready", get(health::ready_check))
        .merge(auth::router());

    // Protected routes — auth middleware applied when VT_AUTH_MODE=required
    let protected_routes = Router::new()
        .merge(config::router())
        .merge(containers::router(&deployment))
        .merge(workspaces::router(&deployment))
        .merge(execution_processes::router(&deployment))
        .merge(tags::router(&deployment))
        .merge(oauth::router())
        .merge(organizations::router())
        .merge(filesystem::router())
        .merge(repo::router())
        .merge(events::router(&deployment))
        .merge(approvals::router())
        .merge(scratch::router(&deployment))
        .merge(search::router(&deployment))
        .merge(releases::router())
        .merge(migration::router())
        .merge(sessions::router(&deployment))
        .merge(terminal::router())
        .merge(orchestration::router(&deployment))
        .merge(feed::router(&deployment))
        .merge(mail::router(&deployment))
        .merge(work_items::router(&deployment))
        .merge(admin::router())
        .nest("/remote", remote::router())
        .nest("/attachments", attachments::routes())
        .layer(axum::middleware::from_fn_with_state(
            auth_service,
            user_auth::middleware::require_auth,
        ));

    let api_routes = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(ValidateRequestHeaderLayer::custom(
            middleware::validate_origin,
        ))
        .layer(axum::middleware::from_fn(middleware::log_server_errors))
        .with_state(deployment);

    Router::new()
        .route("/", get(frontend::serve_frontend_root))
        .route("/{*path}", get(frontend::serve_frontend))
        .nest("/api", api_routes)
        .layer(CompressionLayer::new())
        .into_make_service()
}
