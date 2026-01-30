use std::sync::Arc;
use crate::services::{AuthService, ServerService, ChannelService, MessageService};

/// Structure pour contenir tous les services dans un seul State
#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<AuthService>,
    pub server_service: Arc<ServerService>,
    pub channel_service: Arc<ChannelService>,
    pub message_service: Arc<MessageService>,
}

impl AppState {
    pub fn new(
        auth_service: Arc<AuthService>,
        server_service: Arc<ServerService>,
        channel_service: Arc<ChannelService>,
        message_service: Arc<MessageService>,
    ) -> Self {
        Self {
            auth_service,
            server_service,
            channel_service,
            message_service,
        }
    }
}
