use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Représente un serveur (communauté/guild)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Server {
    pub id: i32,
    pub name: String,
    pub invitation_code: String,
    pub owner_id: i32,
    pub created_at: DateTime<Utc>,
}

/// DTO pour la création d'un serveur
#[derive(Debug, Deserialize)]
pub struct CreateServerDto {
    pub name: String,
}

/// DTO pour rejoindre un serveur
#[derive(Debug, Deserialize)]
pub struct JoinServerDto {
    pub invitation_code: String,
}

/// Représente un membre d'un serveur avec son rôle
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ServerMember {
    pub server_id: i32,
    pub user_id: i32,
    pub role: crate::models::UserRole,
    pub joined_at: DateTime<Utc>,
}

/// Détails complets d'un membre (utilisateur + rôle)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ServerMemberDetails {
    pub user_id: i32,
    pub username: String,
    pub role: crate::models::UserRole,
    pub joined_at: DateTime<Utc>,
}
