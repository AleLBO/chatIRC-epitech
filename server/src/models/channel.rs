use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Représente un canal dans un serveur
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Channel {
    pub id: i32,
    pub name: String,
    #[sqlx(rename = "type")]
    pub channel_type: String,
    pub server_id: i32,
    pub created_at: DateTime<Utc>,
}

/// DTO pour la création d'un canal
#[derive(Debug, Deserialize)]
pub struct CreateChannelDto {
    pub name: String,
    #[serde(default = "default_channel_type")]
    pub channel_type: String,
}

fn default_channel_type() -> String {
    "text".to_string()
}

/// DTO pour la mise à jour d'un canal
#[derive(Debug, Deserialize)]
pub struct UpdateChannelDto {
    pub name: String,
}
