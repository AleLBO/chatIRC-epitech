use async_trait::async_trait;
use sqlx::PgPool;
use crate::errors::AppResult;
use crate::models::{Server, ServerMember, ServerMemberDetails, UserRole};

/// Trait définissant les opérations sur les serveurs
#[async_trait]
pub trait ServerRepositoryTrait: Send + Sync {
    async fn create(&self, name: &str, owner_id: i32, invitation_code: &str) -> AppResult<Server>;
    async fn find_by_id(&self, id: i32) -> AppResult<Option<Server>>;
    async fn find_by_invitation_code(&self, code: &str) -> AppResult<Option<Server>>;
    async fn find_by_user(&self, user_id: i32) -> AppResult<Vec<Server>>;
    async fn update(&self, id: i32, name: &str) -> AppResult<Server>;
    async fn delete(&self, id: i32) -> AppResult<()>;
    
    // Gestion des membres
    async fn add_member(&self, server_id: i32, user_id: i32, role: UserRole) -> AppResult<ServerMember>;
    async fn remove_member(&self, server_id: i32, user_id: i32) -> AppResult<()>;
    async fn get_member_role(&self, server_id: i32, user_id: i32) -> AppResult<Option<UserRole>>;
    async fn update_member_role(&self, server_id: i32, user_id: i32, role: UserRole) -> AppResult<()>;
    async fn get_members(&self, server_id: i32) -> AppResult<Vec<ServerMemberDetails>>;
    async fn is_member(&self, server_id: i32, user_id: i32) -> AppResult<bool>;
}

/// Implémentation PostgreSQL
#[derive(Clone)]
pub struct ServerRepository {
    pool: PgPool,
}

impl ServerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ServerRepositoryTrait for ServerRepository {
    async fn create(&self, name: &str, owner_id: i32, invitation_code: &str) -> AppResult<Server> {
        let server = sqlx::query_as!(
            Server,
            r#"
            INSERT INTO servers (name, owner_id, invitation_code)
            VALUES ($1, $2, $3)
            RETURNING id, name, invitation_code, owner_id, created_at
            "#,
            name,
            owner_id,
            invitation_code
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(server)
    }

    async fn find_by_id(&self, id: i32) -> AppResult<Option<Server>> {
        let server = sqlx::query_as!(
            Server,
            "SELECT id, name, invitation_code, owner_id, created_at FROM servers WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(server)
    }

    async fn find_by_invitation_code(&self, code: &str) -> AppResult<Option<Server>> {
        let server = sqlx::query_as!(
            Server,
            "SELECT id, name, invitation_code, owner_id, created_at FROM servers WHERE invitation_code = $1",
            code
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(server)
    }

    async fn find_by_user(&self, user_id: i32) -> AppResult<Vec<Server>> {
        let servers = sqlx::query_as!(
            Server,
            r#"
            SELECT s.id, s.name, s.invitation_code, s.owner_id, s.created_at
            FROM servers s
            INNER JOIN server_members sm ON s.id = sm.server_id
            WHERE sm.user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(servers)
    }

    async fn update(&self, id: i32, name: &str) -> AppResult<Server> {
        let server = sqlx::query_as!(
            Server,
            "UPDATE servers SET name = $1 WHERE id = $2 RETURNING id, name, invitation_code, owner_id, created_at",
            name,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(server)
    }

    async fn delete(&self, id: i32) -> AppResult<()> {
        sqlx::query!("DELETE FROM servers WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn add_member(&self, server_id: i32, user_id: i32, role: UserRole) -> AppResult<ServerMember> {
        let member = sqlx::query_as!(
            ServerMember,
            r#"
            INSERT INTO server_members (server_id, user_id, role)
            VALUES ($1, $2, $3)
            RETURNING server_id, user_id, role as "role: UserRole", joined_at
            "#,
            server_id,
            user_id,
            role as UserRole
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(member)
    }

    async fn remove_member(&self, server_id: i32, user_id: i32) -> AppResult<()> {
        sqlx::query!(
            "DELETE FROM server_members WHERE server_id = $1 AND user_id = $2",
            server_id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_member_role(&self, server_id: i32, user_id: i32) -> AppResult<Option<UserRole>> {
        let result = sqlx::query!(
            r#"
            SELECT role as "role: UserRole"
            FROM server_members
            WHERE server_id = $1 AND user_id = $2
            "#,
            server_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|r| r.role))
    }

    async fn update_member_role(&self, server_id: i32, user_id: i32, role: UserRole) -> AppResult<()> {
        sqlx::query!(
            "UPDATE server_members SET role = $1 WHERE server_id = $2 AND user_id = $3",
            role as UserRole,
            server_id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_members(&self, server_id: i32) -> AppResult<Vec<ServerMemberDetails>> {
        let members = sqlx::query_as!(
            ServerMemberDetails,
            r#"
            SELECT u.id as user_id, u.username, sm.role as "role: UserRole", sm.joined_at
            FROM server_members sm
            INNER JOIN users u ON sm.user_id = u.id
            WHERE sm.server_id = $1
            ORDER BY sm.joined_at
            "#,
            server_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(members)
    }

    async fn is_member(&self, server_id: i32, user_id: i32) -> AppResult<bool> {
        let result = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM server_members WHERE server_id = $1 AND user_id = $2) as \"exists!\"",
            server_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.exists)
    }
}
