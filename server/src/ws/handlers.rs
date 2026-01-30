use serde::{Deserialize, Serialize};
use socketioxide::extract::{Data, SocketRef};
use std::sync::Arc;
use tracing::{error, info};
use crate::services::{MessageService, ServerService};
use crate::utils::verify_token;
use crate::ws::{Hub, SocketEvent};

/// Payload pour l'authentification Socket.IO
#[derive(Debug, Deserialize)]
pub struct AuthenticatePayload {
    pub token: String,
}

/// Réponse d'authentification
#[derive(Debug, Serialize)]
pub struct AuthenticateResponse {
    pub success: bool,
    pub user_id: Option<i32>,
    pub username: Option<String>,
    pub error: Option<String>,
}

/// Payload pour rejoindre un serveur
#[derive(Debug, Deserialize)]
pub struct JoinServerPayload {
    pub server_id: i32,
}

/// Payload pour quitter un serveur
#[derive(Debug, Deserialize)]
pub struct LeaveServerPayload {
    pub server_id: i32,
}

/// Payload pour l'événement "typing"
#[derive(Debug, Deserialize)]
pub struct TypingPayload {
    pub channel_id: i32,
}

/// Handler pour l'authentification Socket.IO
pub async fn on_authenticate(
    socket: SocketRef,
    Data(data): Data<AuthenticatePayload>,
    hub: Arc<Hub>,
) {
    info!("Tentative d'authentification Socket.IO : {}", socket.id);
    
    match verify_token(&data.token) {
        Ok(claims) => {
            // Stocker les informations utilisateur dans le hub
            hub.authenticate_socket(
                socket.id.to_string(),
                claims.sub,
                claims.username.clone()
            ).await;
            
            info!("✓ Utilisateur {} authentifié via Socket.IO", claims.username);
            
            let response = AuthenticateResponse {
                success: true,
                user_id: Some(claims.sub),
                username: Some(claims.username),
                error: None,
            };
            
            let _ = socket.emit("authenticated", response);
        }
        Err(_) => {
            error!("❌ Échec d'authentification Socket.IO");
            
            let response = AuthenticateResponse {
                success: false,
                user_id: None,
                username: None,
                error: Some("Invalid or expired token".to_string()),
            };
            
            let _ = socket.emit("authenticated", response);
            let _ = socket.disconnect();
        }
    }
}

/// Handler pour rejoindre un serveur
pub async fn on_join_server(
    socket: SocketRef,
    Data(data): Data<JoinServerPayload>,
    hub: Arc<Hub>,
    server_service: Arc<ServerService>,
) {
    // Récupérer l'ID utilisateur depuis le hub
    let user_info = match hub.get_user_info(&socket.id.to_string()).await {
        Some(info) => info,
        None => {
            error!("Utilisateur non authentifié tente de rejoindre un serveur");
            let _ = socket.emit("error", serde_json::json!({
                "error": "Not authenticated",
                "code": "UNAUTHORIZED"
            }));
            return;
        }
    };
    
    // Vérifier que l'utilisateur est membre du serveur
    match server_service.get_user_role(data.server_id, user_info.user_id).await {
        Ok(_role) => {
            // Rejoindre la room du serveur
            let room_name = format!("server:{}", data.server_id);
            let _ = socket.join(room_name.clone());
            
            // Ajouter au hub
            hub.connect_user(data.server_id, user_info.user_id).await;
            
            info!("✓ Utilisateur {} a rejoint le serveur {}", user_info.username, data.server_id);
            
            // Notifier les autres membres
            let event = SocketEvent::UserConnected {
                server_id: data.server_id,
                user_id: user_info.user_id,
                username: user_info.username.clone(),
            };
            
            socket.to(room_name.clone()).emit(event.event_name(), serde_json::to_value(event).unwrap()).ok();
            
            // Envoyer la liste des utilisateurs connectés au nouveau membre
            let connected_users = hub.get_connected_users(data.server_id).await;
            let _ = socket.emit("server:joined", serde_json::json!({
                "server_id": data.server_id,
                "connected_users": connected_users
            }));
        }
        Err(_) => {
            error!("Utilisateur {} n'est pas membre du serveur {}", user_info.user_id, data.server_id);
            let _ = socket.emit("error", serde_json::json!({
                "error": "Not a member of this server",
                "code": "FORBIDDEN"
            }));
        }
    }
}

/// Handler pour quitter un serveur
pub async fn on_leave_server(
    socket: SocketRef,
    Data(data): Data<LeaveServerPayload>,
    hub: Arc<Hub>,
) {
    let user_info = match hub.get_user_info(&socket.id.to_string()).await {
        Some(info) => info,
        None => return,
    };
    
    // Quitter la room
    let room_name = format!("server:{}", data.server_id);
    let _ = socket.leave(room_name.clone());
    
    // Retirer du hub
    hub.disconnect_user(data.server_id, user_info.user_id).await;
    
    // Notifier les autres membres
    let event = SocketEvent::UserDisconnected {
        server_id: data.server_id,
        user_id: user_info.user_id,
        username: user_info.username.clone(),
    };
    
    socket.to(room_name).emit(event.event_name(), serde_json::to_value(event).unwrap()).ok();
    
    info!("✓ Utilisateur {} a quitté le serveur {}", user_info.user_id, data.server_id);
}

/// Handler pour l'événement "typing_start"
pub async fn on_typing_start(
    socket: SocketRef,
    Data(data): Data<TypingPayload>,
    hub: Arc<Hub>,
) {
    let user_info = match hub.get_user_info(&socket.id.to_string()).await {
        Some(info) => info,
        None => return,
    };
    
    let event = SocketEvent::UserTyping {
        channel_id: data.channel_id,
        user_id: user_info.user_id,
        username: user_info.username,
    };
    
    // Envoyer à tous sauf l'émetteur
    socket.to(format!("channel:{}", data.channel_id))
        .emit(event.event_name(), serde_json::to_value(event).unwrap())
        .ok();
}

/// Handler pour la déconnexion
pub async fn on_disconnect(socket: SocketRef, hub: Arc<Hub>) {
    let user_info = match hub.get_user_info(&socket.id.to_string()).await {
        Some(info) => info,
        None => return,
    };
    
    info!("Utilisateur {} déconnecté", user_info.username);
    
    // Nettoyer les données du socket
    hub.disconnect_socket(&socket.id.to_string()).await;
    
    // Note: Dans une vraie application, il faudrait tracker tous les serveurs
    // où l'utilisateur était connecté pour notifier correctement
}
