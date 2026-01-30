use async_trait::async_trait;
use sqlx::PgPool;
use crate::errors::AppResult;
use crate::models::{Message, MessageWithAuthor};

/// Trait définissant les opérations sur les messages
#[async_trait]
pub trait MessageRepositoryTrait: Send + Sync {
    async fn create(&self, content: &str, channel_id: i32, author_id: i32) -> AppResult<Message>;
    async fn find_by_id(&self, id: i32) -> AppResult<Option<Message>>;
    async fn find_by_channel(&self, channel_id: i32, limit: i64, offset: i64) -> AppResult<Vec<MessageWithAuthor>>;
    async fn delete(&self, id: i32) -> AppResult<()>;
    async fn soft_delete(&self, id: i32) -> AppResult<()>;
}

/// Implémentation PostgreSQL
#[derive(Clone)]
pub struct MessageRepository {
    pool: PgPool,
}

impl MessageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MessageRepositoryTrait for MessageRepository {
    async fn create(&self, content: &str, channel_id: i32, author_id: i32) -> AppResult<Message> {
        let message = sqlx::query_as!(
            Message,
            r#"
            INSERT INTO messages (content, channel_id, author_id)
            VALUES ($1, $2, $3)
            RETURNING id, content, channel_id, author_id, is_deleted, created_at, updated_at
            "#,
            content,
            channel_id,
            author_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(message)
    }

    async fn find_by_id(&self, id: i32) -> AppResult<Option<Message>> {
        let message = sqlx::query_as!(
            Message,
            "SELECT id, content, channel_id, author_id, is_deleted, created_at, updated_at FROM messages WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(message)
    }

    async fn find_by_channel(&self, channel_id: i32, limit: i64, offset: i64) -> AppResult<Vec<MessageWithAuthor>> {
        let messages = sqlx::query_as!(
            MessageWithAuthor,
            r#"
            SELECT 
                m.id, 
                m.content, 
                m.channel_id, 
                m.author_id, 
                u.username as author_username,
                m.is_deleted, 
                m.created_at, 
                m.updated_at
            FROM messages m
            INNER JOIN users u ON m.author_id = u.id
            WHERE m.channel_id = $1
            ORDER BY m.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            channel_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(messages)
    }

    async fn delete(&self, id: i32) -> AppResult<()> {
        sqlx::query!("DELETE FROM messages WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn soft_delete(&self, id: i32) -> AppResult<()> {
        sqlx::query!("UPDATE messages SET is_deleted = true WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
