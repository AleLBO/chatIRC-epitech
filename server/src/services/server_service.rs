use std::sync::Arc;
use crate::errors::{AppError, AppResult};
use crate::models::{Server, ServerMemberDetails, UserRole, CreateServerDto};
use crate::repositories::server_repository::{ServerRepository, ServerRepositoryTrait};
use crate::utils::generate_invitation_code;

/// Service gérant les serveurs
#[derive(Clone)]
pub struct ServerService {
    server_repo: Arc<ServerRepository>,
}

impl ServerService {
    pub fn new(server_repo: Arc<ServerRepository>) -> Self {
        Self { server_repo }
    }

    /// Créer un nouveau serveur
    pub async fn create_server(&self, dto: CreateServerDto, owner_id: i32) -> AppResult<Server> {
        // Générer un code d'invitation unique
        let invitation_code = generate_invitation_code();

        // Créer le serveur
        let server = self
            .server_repo
            .create(&dto.name, owner_id, &invitation_code)
            .await?;

        // Ajouter le créateur comme Owner
        self.server_repo
            .add_member(server.id, owner_id, UserRole::Owner)
            .await?;

        Ok(server)
    }

    /// Récupérer les serveurs d'un utilisateur
    pub async fn get_user_servers(&self, user_id: i32) -> AppResult<Vec<Server>> {
        self.server_repo.find_by_user(user_id).await
    }

    /// Récupérer un serveur par ID
    pub async fn get_server(&self, server_id: i32, user_id: i32) -> AppResult<Server> {
        let server = self
            .server_repo
            .find_by_id(server_id)
            .await?
            .ok_or(AppError::ServerNotFound)?;

        // Vérifier que l'utilisateur est membre
        self.check_is_member(server_id, user_id).await?;

        Ok(server)
    }

    /// Mettre à jour un serveur
    pub async fn update_server(&self, server_id: i32, name: String, user_id: i32) -> AppResult<Server> {
        // Vérifier les permissions (Admin ou Owner)
        self.check_is_admin_or_owner(server_id, user_id).await?;

        self.server_repo.update(server_id, &name).await
    }

    /// Supprimer un serveur
    pub async fn delete_server(&self, server_id: i32, user_id: i32) -> AppResult<()> {
        // Seul le propriétaire peut supprimer
        self.check_is_owner(server_id, user_id).await?;

        self.server_repo.delete(server_id).await
    }

    /// Rejoindre un serveur via code d'invitation
    pub async fn join_server(&self, invitation_code: String, user_id: i32) -> AppResult<Server> {
        let server = self
            .server_repo
            .find_by_invitation_code(&invitation_code)
            .await?
            .ok_or(AppError::InvalidInvitationCode)?;

        // Vérifier si l'utilisateur est déjà membre
        if self.server_repo.is_member(server.id, user_id).await? {
            return Err(AppError::AlreadyMember);
        }

        // Ajouter comme membre
        self.server_repo
            .add_member(server.id, user_id, UserRole::Member)
            .await?;

        Ok(server)
    }

    /// Quitter un serveur
    pub async fn leave_server(&self, server_id: i32, user_id: i32) -> AppResult<()> {
        // Vérifier que l'utilisateur est membre
        self.check_is_member(server_id, user_id).await?;

        // Vérifier que ce n'est pas le propriétaire
        let role = self
            .server_repo
            .get_member_role(server_id, user_id)
            .await?
            .ok_or(AppError::ServerNotFound)?;

        if role == UserRole::Owner {
            return Err(AppError::OwnerCannotLeave);
        }

        self.server_repo.remove_member(server_id, user_id).await
    }

    /// Récupérer les membres d'un serveur
    pub async fn get_members(&self, server_id: i32, user_id: i32) -> AppResult<Vec<ServerMemberDetails>> {
        // Vérifier que l'utilisateur est membre
        self.check_is_member(server_id, user_id).await?;

        self.server_repo.get_members(server_id).await
    }

    /// Mettre à jour le rôle d'un membre
    pub async fn update_member_role(
        &self,
        server_id: i32,
        target_user_id: i32,
        new_role: UserRole,
        requester_id: i32,
    ) -> AppResult<()> {
        // Seul le propriétaire peut changer les rôles
        self.check_is_owner(server_id, requester_id).await?;

        // Ne pas permettre de changer le rôle du propriétaire
        let target_role = self
            .server_repo
            .get_member_role(server_id, target_user_id)
            .await?
            .ok_or(AppError::UserNotFound)?;

        if target_role == UserRole::Owner {
            return Err(AppError::Forbidden);
        }

        self.server_repo
            .update_member_role(server_id, target_user_id, new_role)
            .await
    }

    /// Vérifier si un utilisateur est membre d'un serveur
    async fn check_is_member(&self, server_id: i32, user_id: i32) -> AppResult<()> {
        if !self.server_repo.is_member(server_id, user_id).await? {
            return Err(AppError::Forbidden);
        }
        Ok(())
    }

    /// Vérifier si un utilisateur est admin ou owner
    async fn check_is_admin_or_owner(&self, server_id: i32, user_id: i32) -> AppResult<()> {
        let role = self
            .server_repo
            .get_member_role(server_id, user_id)
            .await?
            .ok_or(AppError::Forbidden)?;

        match role {
            UserRole::Owner | UserRole::Admin => Ok(()),
            UserRole::Member => Err(AppError::AdminOnly),
        }
    }

    /// Vérifier si un utilisateur est owner
    async fn check_is_owner(&self, server_id: i32, user_id: i32) -> AppResult<()> {
        let role = self
            .server_repo
            .get_member_role(server_id, user_id)
            .await?
            .ok_or(AppError::Forbidden)?;

        match role {
            UserRole::Owner => Ok(()),
            _ => Err(AppError::OwnerOnly),
        }
    }

    /// Récupérer le rôle d'un utilisateur dans un serveur (utilisé par d'autres services)
    pub async fn get_user_role(&self, server_id: i32, user_id: i32) -> AppResult<UserRole> {
        self.server_repo
            .get_member_role(server_id, user_id)
            .await?
            .ok_or(AppError::Forbidden)
    }
}
