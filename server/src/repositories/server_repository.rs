use async_trait::async_trait;
use sqlx::{PgPool, Row};
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
            "INSERT INTO servers (name, owner_id, invitation_code) VALUES ($1, $2, $3) RETURNING id, name, owner_id, invitation_code, created_at",
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
            "SELECT id, name, owner_id, invitation_code, created_at FROM servers WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(server)
    }

    async fn find_by_invitation_code(&self, code: &str) -> AppResult<Option<Server>> {
        let server = sqlx::query_as!(
            Server,
            "SELECT id, name, owner_id, invitation_code, created_at FROM servers WHERE invitation_code = $1",
            code
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(server)
    }

    async fn find_by_user(&self, user_id: i32) -> AppResult<Vec<Server>> {
        let servers = sqlx::query_as!(
            Server,
            "SELECT s.id, s.name, s.owner_id, s.invitation_code, s.created_at FROM servers s INNER JOIN server_members sm ON s.id = sm.server_id WHERE sm.user_id = $1",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(servers)
    }

    async fn update(&self, id: i32, name: &str) -> AppResult<Server> {
        let server = sqlx::query_as!(
            Server,
            "UPDATE servers SET name = $1 WHERE id = $2 RETURNING id, name, owner_id, invitation_code, created_at",
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
        let role_str = match role {
            UserRole::Owner => "OWNER",
            UserRole::Admin => "ADMIN",
            UserRole::Member => "MEMBER",
        };

        let row = sqlx::query(
            "INSERT INTO server_members (server_id, user_id, role) VALUES ($1, $2, $3) RETURNING server_id, user_id, role, joined_at"
        )
        .bind(server_id)
        .bind(user_id)
        .bind(role_str)
        .fetch_one(&self.pool)
        .await?;

        let role_db: String = row.get("role");
        let role_enum = match role_db.as_str() {
            "OWNER" => UserRole::Owner,
            "ADMIN" => UserRole::Admin,
            "MEMBER" => UserRole::Member,
            _ => UserRole::Member,
        };

        Ok(ServerMember {
            server_id: row.get("server_id"),
            user_id: row.get("user_id"),
            role: role_enum,
            joined_at: row.get("joined_at"),
        })
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
        let row = sqlx::query(
            "SELECT role FROM server_members WHERE server_id = $1 AND user_id = $2"
        )
        .bind(server_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.and_then(|r| {
            let role_str: String = r.get("role");
            match role_str.as_str() {
                "OWNER" => Some(UserRole::Owner),
                "ADMIN" => Some(UserRole::Admin),
                "MEMBER" => Some(UserRole::Member),
                _ => None,
            }
        }))
    }

    async fn update_member_role(&self, server_id: i32, user_id: i32, role: UserRole) -> AppResult<()> {
        let role_str = match role {
            UserRole::Owner => "OWNER",
            UserRole::Admin => "ADMIN",
            UserRole::Member => "MEMBER",
        };

        sqlx::query(
            "UPDATE server_members SET role = $1 WHERE server_id = $2 AND user_id = $3"
        )
        .bind(role_str)
        .bind(server_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_members(&self, server_id: i32) -> AppResult<Vec<ServerMemberDetails>> {
        let rows = sqlx::query(
            "SELECT u.id, u.username, sm.role, sm.joined_at FROM users u INNER JOIN server_members sm ON u.id = sm.user_id WHERE sm.server_id = $1"
        )
        .bind(server_id)
        .fetch_all(&self.pool)
        .await?;

        let members = rows.into_iter().filter_map(|row| {
            let user_id: i32 = row.get("id");
            let username: String = row.get("username");
            let role_str: String = row.get("role");
            let joined_at: chrono::DateTime<chrono::Utc> = row.get("joined_at");
            
            let role = match role_str.as_str() {
                "OWNER" => UserRole::Owner,
                "ADMIN" => UserRole::Admin,
                "MEMBER" => UserRole::Member,
                _ => return None,
            };
            
            Some(ServerMemberDetails {
                user_id,
                username,
                role,
                joined_at,
            })
        }).collect();

        Ok(members)
    }

    async fn is_member(&self, server_id: i32, user_id: i32) -> AppResult<bool> {
        let result = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM server_members WHERE server_id = $1 AND user_id = $2) as exists",
            server_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.exists.unwrap_or(false))
    }
}
