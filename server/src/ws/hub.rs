use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Informations utilisateur pour le WebSocket
#[derive(Clone, Debug)]
pub struct UserInfo {
    pub user_id: i32,
    pub username: String,
}

/// Hub central pour gérer les connexions WebSocket
/// Cette structure garde trace de qui est connecté à quel serveur/canal
#[derive(Clone)]
pub struct Hub {
    /// Map des utilisateurs connectés par serveur
    /// server_id -> Vec<user_id>
    connected_users: Arc<RwLock<HashMap<i32, Vec<i32>>>>,
    
    /// Map socket_id -> UserInfo pour l'authentification
    socket_to_user: Arc<RwLock<HashMap<String, UserInfo>>>,
}

impl Hub {
    pub fn new() -> Self {
        Self {
            connected_users: Arc::new(RwLock::new(HashMap::new())),
            socket_to_user: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Authentifier un socket avec les infos utilisateur
    pub async fn authenticate_socket(&self, socket_id: String, user_id: i32, username: String) {
        let mut map = self.socket_to_user.write().await;
        map.insert(socket_id, UserInfo { user_id, username });
    }

    /// Récupérer les infos utilisateur depuis un socket_id
    pub async fn get_user_info(&self, socket_id: &str) -> Option<UserInfo> {
        let map = self.socket_to_user.read().await;
        map.get(socket_id).cloned()
    }

    /// Déconnecter un socket
    pub async fn disconnect_socket(&self, socket_id: &str) {
        let mut map = self.socket_to_user.write().await;
        map.remove(socket_id);
    }

    /// Enregistrer qu'un utilisateur s'est connecté à un serveur
    pub async fn connect_user(&self, server_id: i32, user_id: i32) {
        let mut users = self.connected_users.write().await;
        users
            .entry(server_id)
            .or_insert_with(Vec::new)
            .push(user_id);
    }

    /// Enregistrer qu'un utilisateur s'est déconnecté d'un serveur
    pub async fn disconnect_user(&self, server_id: i32, user_id: i32) {
        let mut users = self.connected_users.write().await;
        if let Some(server_users) = users.get_mut(&server_id) {
            server_users.retain(|&id| id != user_id);
        }
    }

    /// Récupérer la liste des utilisateurs connectés à un serveur
    pub async fn get_connected_users(&self, server_id: i32) -> Vec<i32> {
        let users = self.connected_users.read().await;
        users.get(&server_id).cloned().unwrap_or_default()
    }

    /// Vérifier si un utilisateur est connecté à un serveur
    pub async fn is_user_connected(&self, server_id: i32, user_id: i32) -> bool {
        let users = self.connected_users.read().await;
        users
            .get(&server_id)
            .map(|server_users| server_users.contains(&user_id))
            .unwrap_or(false)
    }
}

impl Default for Hub {
    fn default() -> Self {
        Self::new()
    }
}
