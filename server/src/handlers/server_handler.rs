use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::errors::AppResult;
use crate::handlers::middleware::AuthUser;
use crate::models::{CreateServerDto, JoinServerDto, Server, ServerMemberDetails, UserRole};
use crate::state::AppState;
use serde::Deserialize;

/// Créer un serveur (POST /servers)
pub async fn create_server(
    State(app_state): State<AppState>,
    auth_user: AuthUser,
    Json(dto): Json<CreateServerDto>,
) -> AppResult<(StatusCode, Json<Server>)> {
    let server = app_state.server_service.create_server(dto, auth_user.user_id).await?;
    Ok((StatusCode::CREATED, Json(server)))
}

/// Lister les serveurs de l'utilisateur (GET /servers)
pub async fn list_servers(
    State(app_state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<Json<Vec<Server>>> {
    let servers = app_state.server_service.get_user_servers(auth_user.user_id).await?;
    Ok(Json(servers))
}

/// Récupérer un serveur (GET /servers/:id)
pub async fn get_server(
    State(app_state): State<AppState>,
    auth_user: AuthUser,
    Path(server_id): Path<i32>,
) -> AppResult<Json<Server>> {
    let server = app_state.server_service.get_server(server_id, auth_user.user_id).await?;
    Ok(Json(server))
}

/// Mettre à jour un serveur (PUT /servers/:id)
pub async fn update_server(
    State(app_state): State<AppState>,
    auth_user: AuthUser,
    Path(server_id): Path<i32>,
    Json(dto): Json<CreateServerDto>,
) -> AppResult<Json<Server>> {
    let server = app_state.server_service
        .update_server(server_id, dto.name, auth_user.user_id)
        .await?;
    Ok(Json(server))
}

/// Supprimer un serveur (DELETE /servers/:id)
pub async fn delete_server(
    State(app_state): State<AppState>,
    auth_user: AuthUser,
    Path(server_id): Path<i32>,
) -> AppResult<StatusCode> {
    app_state.server_service.delete_server(server_id, auth_user.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Rejoindre un serveur (POST /servers/join)
pub async fn join_server(
    State(app_state): State<AppState>,
    auth_user: AuthUser,
    Json(dto): Json<JoinServerDto>,
) -> AppResult<Json<Server>> {
    let server = app_state.server_service
        .join_server(dto.invitation_code, auth_user.user_id)
        .await?;
    Ok(Json(server))
}

/// Quitter un serveur (DELETE /servers/:id/leave)
pub async fn leave_server(
    State(app_state): State<AppState>,
    auth_user: AuthUser,
    Path(server_id): Path<i32>,
) -> AppResult<StatusCode> {
    app_state.server_service.leave_server(server_id, auth_user.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Lister les membres d'un serveur (GET /servers/:id/members)
pub async fn list_members(
    State(app_state): State<AppState>,
    auth_user: AuthUser,
    Path(server_id): Path<i32>,
) -> AppResult<Json<Vec<ServerMemberDetails>>> {
    let members = app_state.server_service.get_members(server_id, auth_user.user_id).await?;
    Ok(Json(members))
}

/// DTO pour mettre à jour le rôle d'un membre
#[derive(Deserialize)]
pub struct UpdateMemberRoleDto {
    pub role: UserRole,
}

/// Mettre à jour le rôle d'un membre (PUT /servers/:server_id/members/:user_id)
pub async fn update_member_role(
    State(app_state): State<AppState>,
    auth_user: AuthUser,
    Path((server_id, target_user_id)): Path<(i32, i32)>,
    Json(dto): Json<UpdateMemberRoleDto>,
) -> AppResult<StatusCode> {
    app_state.server_service
        .update_member_role(server_id, target_user_id, dto.role, auth_user.user_id)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
