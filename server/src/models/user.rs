use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Représente un utilisateur dans le système
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)] // Ne jamais exposer le hash dans les API
    pub password_hash: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

/// Rôles disponibles pour un utilisateur dans un serveur
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRole {
    Owner,
    Admin,
    Member,
}

/// DTO pour la création d'un utilisateur (inscription)
#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// DTO pour la connexion
#[derive(Debug, Deserialize)]
pub struct LoginDto {
    #[serde(alias = "email")]
    pub username: String,
    pub password: String,
}

/// Response après authentification réussie
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: User,
    pub token: String,
}

/// DTO pour la mise à jour d'un utilisateur
#[derive(Debug, Deserialize)]
pub struct UpdateUserDto {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
