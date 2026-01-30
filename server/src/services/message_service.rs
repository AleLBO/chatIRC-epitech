use std::sync::Arc;
use crate::errors::{AppError, AppResult};
use crate::models::{Message, MessageWithAuthor, CreateMessageDto, UserRole};
use crate::repositories::message_repository::{MessageRepository, MessageRepositoryTrait};
use crate::repositories::channel_repository::{ChannelRepository, ChannelRepositoryTrait};
use crate::services::ServerService;

/// Service gérant les messages
#[derive(Clone)]
pub struct MessageService {
    message_repo: Arc<MessageRepository>,
    channel_repo: Arc<ChannelRepository>,
    server_service: Arc<ServerService>,
}

impl MessageService {
    pub fn new(
        message_repo: Arc<MessageRepository>,
        channel_repo: Arc<ChannelRepository>,
        server_service: Arc<ServerService>,
    ) -> Self {
        Self {
            message_repo,
            channel_repo,
            server_service,
        }
    }

    /// Créer un message
    pub async fn create_message(
        &self,
        channel_id: i32,
        dto: CreateMessageDto,
        author_id: i32,
    ) -> AppResult<Message> {
        // Vérifier que le canal existe
        let channel = self
            .channel_repo
            .find_by_id(channel_id)
            .await?
            .ok_or(AppError::ChannelNotFound)?;

        // Vérifier que l'utilisateur est membre du serveur
        self.server_service
            .get_user_role(channel.server_id, author_id)
            .await?;

        // Valider le contenu
        if dto.content.trim().is_empty() {
            return Err(AppError::ValidationError(
                "Le message ne peut pas être vide".to_string(),
            ));
        }

        self.message_repo
            .create(&dto.content, channel_id, author_id)
            .await
    }

    /// Récupérer l'historique des messages d'un canal
    pub async fn get_channel_messages(
        &self,
        channel_id: i32,
        user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> AppResult<Vec<MessageWithAuthor>> {
        // Vérifier que le canal existe
        let channel = self
            .channel_repo
            .find_by_id(channel_id)
            .await?
            .ok_or(AppError::ChannelNotFound)?;

        // Vérifier que l'utilisateur est membre du serveur
        self.server_service
            .get_user_role(channel.server_id, user_id)
            .await?;

        let limit = limit.unwrap_or(50).min(100); // Max 100 messages par requête
        let offset = offset.unwrap_or(0);

        self.message_repo
            .find_by_channel(channel_id, limit, offset)
            .await
    }

    /// Récupérer un message par ID (pour les opérations)
    pub async fn get_message(&self, message_id: i32, user_id: i32) -> AppResult<Message> {
        let message = self
            .message_repo
            .find_by_id(message_id)
            .await?
            .ok_or(AppError::MessageNotFound)?;

        // Vérifier que le canal existe
        let channel = self
            .channel_repo
            .find_by_id(message.channel_id)
            .await?
            .ok_or(AppError::ChannelNotFound)?;

        // Vérifier que l'utilisateur est membre du serveur
        self.server_service
            .get_user_role(channel.server_id, user_id)
            .await?;

        Ok(message)
    }

    /// Supprimer un message
    pub async fn delete_message(&self, message_id: i32, user_id: i32) -> AppResult<()> {
        let message = self
            .message_repo
            .find_by_id(message_id)
            .await?
            .ok_or(AppError::MessageNotFound)?;

        // Vérifier que le canal existe
        let channel = self
            .channel_repo
            .find_by_id(message.channel_id)
            .await?
            .ok_or(AppError::ChannelNotFound)?;

        // Permissions : l'auteur peut supprimer son message, ou un Admin/Owner
        let role = self
            .server_service
            .get_user_role(channel.server_id, user_id)
            .await?;

        let can_delete = message.author_id == user_id
            || matches!(role, UserRole::Admin | UserRole::Owner);

        if !can_delete {
            return Err(AppError::Forbidden);
        }

        // Soft delete
        self.message_repo.soft_delete(message_id).await
    }
}
