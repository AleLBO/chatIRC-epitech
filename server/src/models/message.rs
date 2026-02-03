use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Représente un message dans un canal
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: i32,
    pub content: String,
    pub channel_id: i32,
    pub author_id: i32,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// DTO pour la création d'un message
#[derive(Debug, Deserialize)]
pub struct CreateMessageDto {
    pub content: String,
}

/// Message avec les détails de l'auteur
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MessageWithAuthor {
    pub id: i32,
    pub content: String,
    pub channel_id: i32,
    pub author_id: i32,
    pub author_username: String,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
