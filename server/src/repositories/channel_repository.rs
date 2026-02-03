use async_trait::async_trait;
use sqlx::PgPool;
use crate::errors::AppResult;
use crate::models::Channel;

/// Trait définissant les opérations sur les canaux
#[async_trait]
pub trait ChannelRepositoryTrait: Send + Sync {
    async fn create(&self, name: &str, channel_type: &str, server_id: i32) -> AppResult<Channel>;
    async fn find_by_id(&self, id: i32) -> AppResult<Option<Channel>>;
    async fn find_by_server(&self, server_id: i32) -> AppResult<Vec<Channel>>;
    async fn update(&self, id: i32, name: &str) -> AppResult<Channel>;
    async fn delete(&self, id: i32) -> AppResult<()>;
}

/// Implémentation PostgreSQL
#[derive(Clone)]
pub struct ChannelRepository {
    pool: PgPool,
}

impl ChannelRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ChannelRepositoryTrait for ChannelRepository {
    async fn create(&self, name: &str, channel_type: &str, server_id: i32) -> AppResult<Channel> {
        let channel = sqlx::query_as!(
            Channel,
            "INSERT INTO channels (name, type, server_id) VALUES ($1, $2, $3) RETURNING id, name, type, server_id, created_at",
            name,
            channel_type,
            server_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(channel)
    }

    async fn find_by_id(&self, id: i32) -> AppResult<Option<Channel>> {
        let channel = sqlx::query_as!(
            Channel,
            "SELECT id, name, type, server_id, created_at FROM channels WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(channel)
    }

    async fn find_by_server(&self, server_id: i32) -> AppResult<Vec<Channel>> {
        let channels = sqlx::query_as!(
            Channel,
            "SELECT id, name, type, server_id, created_at FROM channels WHERE server_id = $1",
            server_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(channels)
    }

    async fn update(&self, id: i32, name: &str) -> AppResult<Channel> {
        let channel = sqlx::query_as!(
            Channel,
            "UPDATE channels SET name = $1 WHERE id = $2 RETURNING id, name, type, server_id, created_at",
            name,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(channel)
    }

    async fn delete(&self, id: i32) -> AppResult<()> {
        sqlx::query!("DELETE FROM channels WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
