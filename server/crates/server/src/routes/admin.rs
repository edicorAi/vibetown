use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
};
use deployment::Deployment;
use serde::Deserialize;
use user_auth::{
    UserAuthService,
    models::{
        CreateTeam, CreateUser, Team, TeamMember, User,
    },
};

use crate::DeploymentImpl;

fn get_auth_service(deployment: &DeploymentImpl) -> Result<&UserAuthService, Response> {
    deployment.user_auth().ok_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "Authentication not enabled"})),
        )
            .into_response()
    })
}

fn require_admin(user: &User) -> Result<(), Response> {
    if !user.is_admin {
        return Err((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"error": "Admin access required"})),
        )
            .into_response());
    }
    Ok(())
}

// ── User Management ──

#[derive(Debug, Deserialize)]
pub struct CreateLocalUserRequest {
    pub email: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub password: String,
    #[serde(default)]
    pub is_admin: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub is_active: Option<bool>,
    pub is_admin: Option<bool>,
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub new_password: String,
}

/// POST /api/admin/users — create a local user
async fn create_user(
    State(deployment): State<DeploymentImpl>,
    Extension(admin): Extension<User>,
    Json(body): Json<CreateLocalUserRequest>,
) -> Result<Response, Response> {
    require_admin(&admin)?;
    let auth = get_auth_service(&deployment)?;

    let data = CreateUser {
        email: body.email,
        username: body.username,
        display_name: body.display_name,
        password: Some(body.password),
        auth_provider: "local".to_string(),
        external_id: None,
        avatar_url: None,
        is_admin: body.is_admin,
    };

    let user = auth
        .create_user(data)
        .await
        .map_err(|e| e.into_response())?;

    Ok((StatusCode::CREATED, Json(user)).into_response())
}

/// GET /api/admin/users — list all users
async fn list_users(
    State(deployment): State<DeploymentImpl>,
    Extension(admin): Extension<User>,
) -> Result<Json<Vec<User>>, Response> {
    require_admin(&admin)?;
    let auth = get_auth_service(&deployment)?;
    let users = auth.list_users().await.map_err(|e| e.into_response())?;
    Ok(Json(users))
}

/// PUT /api/admin/users/:id — update user
async fn update_user(
    State(deployment): State<DeploymentImpl>,
    Extension(admin): Extension<User>,
    Path(user_id): Path<String>,
    Json(body): Json<UpdateUserRequest>,
) -> Result<Response, Response> {
    require_admin(&admin)?;
    let auth = get_auth_service(&deployment)?;

    if let Some(is_active) = body.is_active {
        if is_active {
            auth.activate_user(&user_id)
                .await
                .map_err(|e| e.into_response())?;
        } else {
            auth.deactivate_user(&user_id)
                .await
                .map_err(|e| e.into_response())?;
        }
    }

    if let Some(is_admin) = body.is_admin {
        auth.set_user_admin(&user_id, is_admin)
            .await
            .map_err(|e| e.into_response())?;
    }

    let user = auth
        .get_user(&user_id)
        .await
        .map_err(|e| e.into_response())?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "User not found"})),
            )
                .into_response()
        })?;

    Ok(Json(user).into_response())
}

/// DELETE /api/admin/users/:id — deactivate user
async fn deactivate_user(
    State(deployment): State<DeploymentImpl>,
    Extension(admin): Extension<User>,
    Path(user_id): Path<String>,
) -> Result<StatusCode, Response> {
    require_admin(&admin)?;
    let auth = get_auth_service(&deployment)?;
    auth.deactivate_user(&user_id)
        .await
        .map_err(|e| e.into_response())?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/admin/users/:id/reset-password
async fn reset_password(
    State(deployment): State<DeploymentImpl>,
    Extension(admin): Extension<User>,
    Path(user_id): Path<String>,
    Json(body): Json<ResetPasswordRequest>,
) -> Result<StatusCode, Response> {
    require_admin(&admin)?;
    let auth = get_auth_service(&deployment)?;
    auth.reset_user_password(&user_id, &body.new_password)
        .await
        .map_err(|e| e.into_response())?;
    Ok(StatusCode::NO_CONTENT)
}

// ── Team Management ──

/// POST /api/teams — create team
async fn create_team(
    State(deployment): State<DeploymentImpl>,
    Extension(user): Extension<User>,
    Json(body): Json<CreateTeam>,
) -> Result<Response, Response> {
    let auth = get_auth_service(&deployment)?;
    let team = user_auth::models::create_team(auth.pool(), &body, &user.id)
        .await
        .map_err(|e| e.into_response())?;
    Ok((StatusCode::CREATED, Json(team)).into_response())
}

/// GET /api/teams — list user's teams
async fn list_teams(
    State(deployment): State<DeploymentImpl>,
    Extension(user): Extension<User>,
) -> Result<Json<Vec<Team>>, Response> {
    let auth = get_auth_service(&deployment)?;
    let teams = user_auth::models::list_user_teams(auth.pool(), &user.id)
        .await
        .map_err(|e| e.into_response())?;
    Ok(Json(teams))
}

/// GET /api/teams/:id — get team details
async fn get_team(
    State(deployment): State<DeploymentImpl>,
    Path(team_id): Path<String>,
) -> Result<Json<Team>, Response> {
    let auth = get_auth_service(&deployment)?;
    let team = user_auth::models::get_team_by_id(auth.pool(), &team_id)
        .await
        .map_err(|e| e.into_response())?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Team not found"})),
            )
                .into_response()
        })?;
    Ok(Json(team))
}

/// DELETE /api/teams/:id — delete team (admin only)
async fn delete_team_handler(
    State(deployment): State<DeploymentImpl>,
    Extension(admin): Extension<User>,
    Path(team_id): Path<String>,
) -> Result<StatusCode, Response> {
    require_admin(&admin)?;
    let auth = get_auth_service(&deployment)?;
    user_auth::models::delete_team(auth.pool(), &team_id)
        .await
        .map_err(|e| e.into_response())?;
    Ok(StatusCode::NO_CONTENT)
}

/// GET /api/teams/:id/members — list team members
async fn list_team_members(
    State(deployment): State<DeploymentImpl>,
    Path(team_id): Path<String>,
) -> Result<Json<Vec<TeamMember>>, Response> {
    let auth = get_auth_service(&deployment)?;
    let members = user_auth::models::list_team_members(auth.pool(), &team_id)
        .await
        .map_err(|e| e.into_response())?;
    Ok(Json(members))
}

#[derive(Debug, Deserialize)]
pub struct AddTeamMemberRequest {
    pub user_id: String,
    pub role: String,
}

/// POST /api/teams/:id/members — add member
async fn add_team_member(
    State(deployment): State<DeploymentImpl>,
    Path(team_id): Path<String>,
    Json(body): Json<AddTeamMemberRequest>,
) -> Result<StatusCode, Response> {
    let auth = get_auth_service(&deployment)?;
    let role_id = format!("role-{}", body.role);
    user_auth::models::add_team_member(auth.pool(), &team_id, &body.user_id, &role_id)
        .await
        .map_err(|e| e.into_response())?;
    Ok(StatusCode::CREATED)
}

/// DELETE /api/teams/:id/members/:user_id — remove member
async fn remove_team_member(
    State(deployment): State<DeploymentImpl>,
    Path((team_id, user_id)): Path<(String, String)>,
) -> Result<StatusCode, Response> {
    let auth = get_auth_service(&deployment)?;
    user_auth::models::remove_team_member(auth.pool(), &team_id, &user_id)
        .await
        .map_err(|e| e.into_response())?;
    Ok(StatusCode::NO_CONTENT)
}

// ── Router ──

pub fn router() -> Router<DeploymentImpl> {
    let admin_routes = Router::new()
        .route("/admin/users", post(create_user).get(list_users))
        .route("/admin/users/{id}", put(update_user).delete(deactivate_user))
        .route("/admin/users/{id}/reset-password", post(reset_password));

    let team_routes = Router::new()
        .route("/teams", post(create_team).get(list_teams))
        .route("/teams/{id}", get(get_team).delete(delete_team_handler))
        .route(
            "/teams/{id}/members",
            get(list_team_members).post(add_team_member),
        )
        .route("/teams/{id}/members/{user_id}", delete(remove_team_member));

    admin_routes.merge(team_routes)
}
