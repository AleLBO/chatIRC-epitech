use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension,
    Json,
};
use serde::Deserialize;
use socketioxide::SocketIo;
use crate::errors::AppResult;
use crate::handlers::middleware::AuthUser;
use crate::models::{CreateMessageDto, Message, MessageWithAuthor};
use crate::state::AppState;
use crate::ws::SocketEvent;

/// Query parameters pour la pagination
#[derive(Deserialize)]
pub struct MessageQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Créer un message (POST /channels/:channel_id/messages)
pub async fn create_message(
    State(app_state): State<AppState>,
    Extension(io): Extension<SocketIo>,
    auth_user: AuthUser,
    Path(channel_id): Path<i32>,
    Json(dto): Json<CreateMessageDto>,
) -> AppResult<(StatusCode, Json<Message>)> {
    let message = app_state.message_service
        .create_message(channel_id, dto, auth_user.user_id)
        .await?;
    
    // Broadcaster le nouveau message via WebSocket
    let event = SocketEvent::NewMessage {
        channel_id: message.channel_id,
        message_id: message.id,
        content: message.content.clone(),
        author_id: auth_user.user_id,
        author_username: auth_user.username,
        created_at: message.created_at.to_rfc3339(),
    };
    
    // Envoyer à tous les membres du canal
    io.of("/")
        .expect("default namespace")
        .to(format!("channel:{}", channel_id))
        .emit(event.event_name(), serde_json::to_value(event).unwrap())
        .ok();
    
    Ok((StatusCode::CREATED, Json(message)))
}

/// Récupérer l'historique des messages (GET /channels/:channel_id/messages)
pub async fn get_messages(
    State(app_state): State<AppState>,
    auth_user: AuthUser,
    Path(channel_id): Path<i32>,
    Query(params): Query<MessageQuery>,
) -> AppResult<Json<Vec<MessageWithAuthor>>> {
    let messages = app_state.message_service
        .get_channel_messages(channel_id, auth_user.user_id, params.limit, params.offset)
        .await?;
    Ok(Json(messages))
}

/// Supprimer un message (DELETE /messages/:id)
pub async fn delete_message(
    State(app_state): State<AppState>,
    Extension(io): Extension<SocketIo>,
    auth_user: AuthUser,
    Path(message_id): Path<i32>,
) -> AppResult<StatusCode> {
    // Récupérer le message pour avoir le channel_id
    let message = app_state.message_service
        .get_message(message_id, auth_user.user_id)
        .await?;
    
    app_state.message_service
        .delete_message(message_id, auth_user.user_id)
        .await?;
    
    // Broadcaster la suppression via WebSocket
    let event = SocketEvent::MessageDeleted {
        channel_id: message.channel_id,
        message_id,
    };
    
    io.of("/")
        .expect("default namespace")
        .to(format!("channel:{}", message.channel_id))
        .emit(event.event_name(), serde_json::to_value(event).unwrap())
        .ok();
    
    Ok(StatusCode::NO_CONTENT)
}
