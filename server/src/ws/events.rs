use serde::{Deserialize, Serialize};

/// Événements Socket.IO que le serveur peut envoyer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum SocketEvent {
    /// Un nouveau message a été créé
    NewMessage {
        channel_id: i32,
        message_id: i32,
        content: String,
        author_id: i32,
        author_username: String,
        created_at: String,
    },
    
    /// Un message a été supprimé
    MessageDeleted {
        channel_id: i32,
        message_id: i32,
    },
    
    /// Un utilisateur est en train de taper
    UserTyping {
        channel_id: i32,
        user_id: i32,
        username: String,
    },
    
    /// Un utilisateur s'est connecté
    UserConnected {
        server_id: i32,
        user_id: i32,
        username: String,
    },
    
    /// Un utilisateur s'est déconnecté
    UserDisconnected {
        server_id: i32,
        user_id: i32,
        username: String,
    },
    
    /// Un nouveau membre a rejoint le serveur
    MemberJoined {
        server_id: i32,
        user_id: i32,
        username: String,
    },
    
    /// Un membre a quitté le serveur
    MemberLeft {
        server_id: i32,
        user_id: i32,
        username: String,
    },
    
    /// Un canal a été créé
    ChannelCreated {
        server_id: i32,
        channel_id: i32,
        name: String,
    },
    
    /// Un canal a été supprimé
    ChannelDeleted {
        server_id: i32,
        channel_id: i32,
    },
}

impl SocketEvent {
    /// Retourne le nom de l'événement pour Socket.IO
    pub fn event_name(&self) -> &'static str {
        match self {
            SocketEvent::NewMessage { .. } => "message:new",
            SocketEvent::MessageDeleted { .. } => "message:deleted",
            SocketEvent::UserTyping { .. } => "user:typing",
            SocketEvent::UserConnected { .. } => "user:connected",
            SocketEvent::UserDisconnected { .. } => "user:disconnected",
            SocketEvent::MemberJoined { .. } => "member:joined",
            SocketEvent::MemberLeft { .. } => "member:left",
            SocketEvent::ChannelCreated { .. } => "channel:created",
            SocketEvent::ChannelDeleted { .. } => "channel:deleted",
        }
    }
}
