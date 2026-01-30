use std::sync::Arc;
use crate::errors::{AppError, AppResult};
use crate::models::{User, CreateUserDto, LoginDto, AuthResponse};
use crate::repositories::user_repository::{UserRepository, UserRepositoryTrait};
use crate::utils::{hash_password, verify_password, create_token};

/// Service gérant l'authentification
#[derive(Clone)]
pub struct AuthService {
    user_repo: Arc<UserRepository>,
}

impl AuthService {
    pub fn new(user_repo: Arc<UserRepository>) -> Self {
        Self { user_repo }
    }

    /// Inscription d'un nouvel utilisateur
    pub async fn register(&self, dto: CreateUserDto) -> AppResult<AuthResponse> {
        // Vérifier si l'username existe déjà
        if self.user_repo.find_by_username(&dto.username).await?.is_some() {
            return Err(AppError::UsernameTaken);
        }

        // Vérifier si l'email existe déjà
        if self.user_repo.find_by_email(&dto.email).await?.is_some() {
            return Err(AppError::EmailTaken);
        }

        // Valider le mot de passe (au moins 6 caractères)
        if dto.password.len() < 6 {
            return Err(AppError::ValidationError(
                "Le mot de passe doit contenir au moins 6 caractères".to_string(),
            ));
        }

        // Hasher le mot de passe
        let password_hash = hash_password(&dto.password)?;

        // Créer l'utilisateur
        let user = self.user_repo.create(dto, &password_hash).await?;

        // Générer un token JWT
        let token = create_token(user.id, &user.username)?;

        Ok(AuthResponse { user, token })
    }

    /// Connexion d'un utilisateur
    pub async fn login(&self, dto: LoginDto) -> AppResult<AuthResponse> {
        // Trouver l'utilisateur par username
        let user = self
            .user_repo
            .find_by_username(&dto.username)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        // Vérifier le mot de passe
        let is_valid = verify_password(&dto.password, &user.password_hash)?;
        if !is_valid {
            return Err(AppError::InvalidCredentials);
        }

        // Générer un token JWT
        let token = create_token(user.id, &user.username)?;

        Ok(AuthResponse { user, token })
    }

    /// Récupérer l'utilisateur actuel par son ID
    pub async fn get_current_user(&self, user_id: i32) -> AppResult<User> {
        self.user_repo
            .find_by_id(user_id)
            .await?
            .ok_or(AppError::UserNotFound)
    }
}
