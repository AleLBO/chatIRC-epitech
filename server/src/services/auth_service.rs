use std::sync::Arc;
use crate::errors::{AppError, AppResult};
use crate::models::{User, CreateUserDto, LoginDto, AuthResponse, UpdateUserDto};
use crate::repositories::user_repository::{UserRepository, UserRepositoryTrait};
use crate::utils::{hash_password, verify_password, create_token};

#[derive(Clone)]
pub struct AuthService {
    user_repo: Arc<UserRepository>,
}

impl AuthService {
    pub fn new(user_repo: Arc<UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn register(&self, dto: CreateUserDto) -> AppResult<AuthResponse> {
        if self.user_repo.find_by_username(&dto.username).await?.is_some() {
            return Err(AppError::UsernameTaken);
        }

        if self.user_repo.find_by_email(&dto.email).await?.is_some() {
            return Err(AppError::EmailTaken);
        }

        if dto.password.len() < 6 {
            return Err(AppError::ValidationError(
                "Le mot de passe doit contenir au moins 6 caractères".to_string(),
            ));
        }

        let password_hash = hash_password(&dto.password)?;
        let user = self.user_repo.create(dto, &password_hash).await?;
        let token = create_token(user.id, &user.username)?;
        tracing::info!("✅ Inscription réussie - Nouvel utilisateur: {} (ID: {})", user.username, user.id);

        Ok(AuthResponse { user, token })
    }

    pub async fn login(&self, dto: LoginDto) -> AppResult<AuthResponse> {
        // Essayer de trouver l'utilisateur par username d'abord, puis par email
        let user = match self.user_repo.find_by_username(&dto.username).await? {
            Some(user) => user,
            None => self.user_repo
                .find_by_email(&dto.username)
                .await?
                .ok_or(AppError::InvalidCredentials)?,
        };

        let is_valid = verify_password(&dto.password, &user.password_hash)?;
        if !is_valid {
            return Err(AppError::InvalidCredentials);
        }

        let token = create_token(user.id, &user.username)?;
        tracing::info!("✅ Connexion réussie - Utilisateur: {} (ID: {})", user.username, user.id);
        Ok(AuthResponse { user, token })
    }

    pub async fn get_current_user(&self, user_id: i32) -> AppResult<User> {
        self.user_repo
            .find_by_id(user_id)
            .await?
            .ok_or(AppError::UserNotFound)
    }

    // --------- Update ---------
    pub async fn update_user(
        &self,
        user_id: i32,
        dto: UpdateUserDto,
    ) -> AppResult<User> {
        let user = self.user_repo.find_by_id(user_id).await?
            .ok_or(AppError::UserNotFound)?;

        let username = dto.username.unwrap_or(user.username);
        let email = dto.email.unwrap_or(user.email);

        if let Some(_) = self.user_repo.find_by_username(&username).await? {
            return Err(AppError::UsernameTaken);
        }
        if let Some(_) = self.user_repo.find_by_email(&email).await? {
            return Err(AppError::EmailTaken);
        }

        let password_hash = if let Some(pwd) = &dto.password {
            hash_password(pwd)?
        } else {
            user.password_hash.clone()
        };

        self.user_repo.update(user_id, &username, &email, &password_hash).await
    }

    // --------- Delete ---------
    pub async fn delete_user(&self, user_id: i32) -> AppResult<()> {
        self.user_repo.delete(user_id).await?;
        Ok(())
    }
}
