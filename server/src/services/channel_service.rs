use std::sync::Arc;
use crate::errors::{AppError, AppResult};
use crate::models::{Channel, CreateChannelDto, UpdateChannelDto, UserRole};
use crate::repositories::channel_repository::{ChannelRepository, ChannelRepositoryTrait};
use crate::services::ServerService;

/// Service gérant les canaux
#[derive(Clone)]
pub struct ChannelService {
    channel_repo: Arc<ChannelRepository>,
    server_service: Arc<ServerService>,
}

impl ChannelService {
    pub fn new(channel_repo: Arc<ChannelRepository>, server_service: Arc<ServerService>) -> Self {
        Self {
            channel_repo,
            server_service,
        }
    }

    /// Créer un canal (Admin ou Owner uniquement)
    pub async fn create_channel(
        &self,
        server_id: i32,
        dto: CreateChannelDto,
        user_id: i32,
    ) -> AppResult<Channel> {
        // Vérifier les permissions
        let role = self.server_service.get_user_role(server_id, user_id).await?;
        match role {
            UserRole::Owner | UserRole::Admin => {}
            UserRole::Member => return Err(AppError::AdminOnly),
        }

        self.channel_repo
            .create(&dto.name, &dto.channel_type, server_id)
            .await
    }

    /// Récupérer les canaux d'un serveur
    pub async fn get_server_channels(&self, server_id: i32, user_id: i32) -> AppResult<Vec<Channel>> {
        // Vérifier que l'utilisateur est membre du serveur
        self.server_service.get_user_role(server_id, user_id).await?;

        self.channel_repo.find_by_server(server_id).await
    }

    /// Récupérer un canal par ID
    pub async fn get_channel(&self, channel_id: i32, user_id: i32) -> AppResult<Channel> {
        let channel = self
            .channel_repo
            .find_by_id(channel_id)
            .await?
            .ok_or(AppError::ChannelNotFound)?;

        // Vérifier que l'utilisateur est membre du serveur
        self.server_service
            .get_user_role(channel.server_id, user_id)
            .await?;

        Ok(channel)
    }

    /// Mettre à jour un canal (Admin ou Owner uniquement)
    pub async fn update_channel(
        &self,
        channel_id: i32,
        dto: UpdateChannelDto,
        user_id: i32,
    ) -> AppResult<Channel> {
        let channel = self
            .channel_repo
            .find_by_id(channel_id)
            .await?
            .ok_or(AppError::ChannelNotFound)?;

        // Vérifier les permissions
        let role = self
            .server_service
            .get_user_role(channel.server_id, user_id)
            .await?;
        match role {
            UserRole::Owner | UserRole::Admin => {}
            UserRole::Member => return Err(AppError::AdminOnly),
        }

        self.channel_repo.update(channel_id, &dto.name).await
    }

    /// Supprimer un canal (Admin ou Owner uniquement)
    pub async fn delete_channel(&self, channel_id: i32, user_id: i32) -> AppResult<()> {
        let channel = self
            .channel_repo
            .find_by_id(channel_id)
            .await?
            .ok_or(AppError::ChannelNotFound)?;

        // Vérifier les permissions
        let role = self
            .server_service
            .get_user_role(channel.server_id, user_id)
            .await?;
        match role {
            UserRole::Owner | UserRole::Admin => {}
            UserRole::Member => return Err(AppError::AdminOnly),
        }

        self.channel_repo.delete(channel_id).await
    }
}
