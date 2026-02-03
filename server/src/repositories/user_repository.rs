use async_trait::async_trait;
use sqlx::PgPool;
use crate::errors::AppResult;
use crate::models::{User, CreateUserDto};

/// Trait définissant les opérations sur les utilisateurs
#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn create(&self, dto: CreateUserDto, password_hash: &str) -> AppResult<User>;
    async fn find_by_id(&self, id: i32) -> AppResult<Option<User>>;
    async fn find_by_username(&self, username: &str) -> AppResult<Option<User>>;
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;

    // Ajout pour update/delete
    async fn update(
        &self,
        id: i32,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> AppResult<User>;

    async fn delete(&self, id: i32) -> AppResult<()>;
}

/// Implémentation PostgreSQL du repository utilisateur
#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn create(&self, dto: CreateUserDto, password_hash: &str) -> AppResult<User> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id, username, email, password_hash, created_at",
            dto.username,
            dto.email,
            password_hash
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_id(&self, id: i32) -> AppResult<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, username, email, password_hash, created_at FROM users WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> AppResult<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, username, email, password_hash, created_at FROM users WHERE username = $1",
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, username, email, password_hash, created_at FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    // ---- Update ----
    async fn update(
        &self,
        id: i32,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> AppResult<User> {
        let user = sqlx::query_as!(
            User,
            "UPDATE users SET username = $1, email = $2, password_hash = $3 WHERE id = $4 RETURNING id, username, email, password_hash, created_at",
            username,
            email,
            password_hash,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    // ---- Delete ----
    async fn delete(&self, id: i32) -> AppResult<()> {
        sqlx::query!(
            "DELETE FROM users WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
