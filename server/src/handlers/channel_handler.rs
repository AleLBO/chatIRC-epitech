use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::errors::AppResult;
use crate::handlers::middleware::AuthUser;
use crate::models::{Channel, CreateChannelDto, UpdateChannelDto};
use crate::state::AppState;

/// Créer un canal (POST /servers/:server_id/channels)
pub async fn create_channel(
    State(app_state): State<AppState>,
    auth_user: AuthUser,
    Path(server_id): Path<i32>,
    Json(dto): Json<CreateChannelDto>,
) -> AppResult<(StatusCode, Json<Channel>)> {
    let channel = app_state.channel_service
        .create_channel(server_id, dto, auth_user.user_id)
        .await?;
    Ok((StatusCode::CREATED, Json(channel)))
}

/// Lister les canaux d'un serveur (GET /servers/:server_id/channels)
pub async fn list_channels(
    State(app_state): State<AppState>,
    auth_user: AuthUser,
    Path(server_id): Path<i32>,
) -> AppResult<Json<Vec<Channel>>> {
    let channels = app_state.channel_service
        .get_server_channels(server_id, auth_user.user_id)
        .await?;
    Ok(Json(channels))
}

/// Récupérer un canal (GET /channels/:id)
pub async fn get_channel(
    State(app_state): State<AppState>,
    auth_user: AuthUser,
    Path(channel_id): Path<i32>,
) -> AppResult<Json<Channel>> {
    let channel = app_state.channel_service
        .get_channel(channel_id, auth_user.user_id)
        .await?;
    Ok(Json(channel))
}

/// Mettre à jour un canal (PUT /channels/:id)
pub async fn update_channel(
    State(app_state): State<AppState>,
    auth_user: AuthUser,
    Path(channel_id): Path<i32>,
    Json(dto): Json<UpdateChannelDto>,
) -> AppResult<Json<Channel>> {
    let channel = app_state.channel_service
        .update_channel(channel_id, dto, auth_user.user_id)
        .await?;
    Ok(Json(channel))
}

/// Supprimer un canal (DELETE /channels/:id)
pub async fn delete_channel(
    State(app_state): State<AppState>,
    auth_user: AuthUser,
    Path(channel_id): Path<i32>,
) -> AppResult<StatusCode> {
    app_state.channel_service
        .delete_channel(channel_id, auth_user.user_id)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
