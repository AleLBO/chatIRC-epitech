use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// Erreurs personnalisées de l'application
#[derive(Debug, Error)]
pub enum AppError {
    // Erreurs d'authentification
    #[error("Nom d'utilisateur déjà pris")]
    UsernameTaken,
    
    #[error("Email déjà utilisé")]
    EmailTaken,
    
    #[error("Identifiants invalides")]
    InvalidCredentials,
    
    #[error("Token invalide ou expiré")]
    InvalidToken,
    
    #[error("Non autorisé")]
    Unauthorized,
    
    // Erreurs de permissions
    #[error("Permission refusée")]
    Forbidden,
    
    #[error("Vous devez être propriétaire pour effectuer cette action")]
    OwnerOnly,
    
    #[error("Vous devez être admin ou propriétaire pour effectuer cette action")]
    AdminOnly,
    
    // Erreurs de ressources
    #[error("Utilisateur non trouvé")]
    UserNotFound,
    
    #[error("Serveur non trouvé")]
    ServerNotFound,
    
    #[error("Canal non trouvé")]
    ChannelNotFound,
    
    #[error("Message non trouvé")]
    MessageNotFound,
    
    #[error("Code d'invitation invalide")]
    InvalidInvitationCode,
    
    #[error("Vous êtes déjà membre de ce serveur")]
    AlreadyMember,
    
    #[error("Le propriétaire ne peut pas quitter son serveur")]
    OwnerCannotLeave,
    
    // Erreurs de validation
    #[error("Données invalides: {0}")]
    ValidationError(String),
    
    // Erreurs base de données
    #[error("Erreur de base de données: {0}")]
    DatabaseError(String),
    
    // Erreurs internes
    #[error("Erreur interne du serveur")]
    InternalServerError,
}

/// Conversion des erreurs SQLx en AppError
impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => AppError::UserNotFound,
            _ => AppError::DatabaseError(error.to_string()),
        }
    }
}

/// Conversion de AppError en réponse HTTP
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::UsernameTaken | AppError::EmailTaken | AppError::AlreadyMember => {
                (StatusCode::CONFLICT, self.to_string())
            }
            AppError::InvalidCredentials | AppError::InvalidToken => {
                (StatusCode::UNAUTHORIZED, self.to_string())
            }
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Forbidden | AppError::OwnerOnly | AppError::AdminOnly => {
                (StatusCode::FORBIDDEN, self.to_string())
            }
            AppError::UserNotFound
            | AppError::ServerNotFound
            | AppError::ChannelNotFound
            | AppError::MessageNotFound
            | AppError::InvalidInvitationCode => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::ValidationError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::DatabaseError(_) | AppError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Erreur interne du serveur".to_string())
            }
            AppError::OwnerCannotLeave => (StatusCode::BAD_REQUEST, self.to_string()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

/// Type Result personnalisé pour l'application
pub type AppResult<T> = Result<T, AppError>;
