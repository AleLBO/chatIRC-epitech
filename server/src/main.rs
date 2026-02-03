mod errors;
mod handlers;
mod models;
mod repositories;
mod services;
mod state;
mod utils;
mod ws;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use socketioxide::{extract::{Data, SocketRef}, SocketIo};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Charger les variables d'environnement
    dotenvy::dotenv().ok();
    
    // 2. Initialiser les logs
    tracing_subscriber::fmt().init();

    // 3. Connexion à la base de données PostgreSQL
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL doit être définie dans les variables d'environnement");
    
    info!("Connexion à la base de données...");
    let pool = PgPool::connect(&database_url).await?;
    info!("✓ Connexion à PostgreSQL établie");

    // 4. Initialiser les repositories
    let user_repo = Arc::new(repositories::UserRepository::new(pool.clone()));
    let server_repo = Arc::new(repositories::ServerRepository::new(pool.clone()));
    let channel_repo = Arc::new(repositories::ChannelRepository::new(pool.clone()));
    let message_repo = Arc::new(repositories::MessageRepository::new(pool.clone()));    // 5. Initialiser les services
    let auth_service = Arc::new(services::AuthService::new(user_repo.clone()));
    let server_service = Arc::new(services::ServerService::new(server_repo.clone()));
    let channel_service = Arc::new(services::ChannelService::new(
        channel_repo.clone(),
        server_service.clone(),
    ));
    let message_service = Arc::new(services::MessageService::new(
        message_repo.clone(),
        channel_repo.clone(),
        server_service.clone(),
    ));

    // Créer l'AppState avec tous les services
    let app_state = AppState::new(
        auth_service.clone(),
        server_service.clone(),
        channel_service.clone(),
        message_service.clone(),
    );// 6. Initialiser Socket.IO pour le temps réel
    let (socket_layer, io) = SocketIo::new_layer();
    
    // Créer le Hub pour gérer les connexions
    let hub = Arc::new(ws::Hub::new());    // Configuration des événements WebSocket
    io.ns("/", {
        let hub = hub.clone();
        let server_service = server_service.clone();
        
        move |socket: SocketRef| {            info!("🔌 Nouveau client Socket.IO connecté : {}", socket.id);
              // Handler pour l'authentification
            socket.on("authenticate", {
                let hub = hub.clone();
                move |socket: SocketRef, data: Data<ws::AuthenticatePayload>| async move {
                    ws::on_authenticate(socket, data, hub.clone()).await;
                }
            });
              // Handler pour rejoindre un serveur
            socket.on("join_server", {
                let hub = hub.clone();
                let server_service = server_service.clone();
                move |socket: SocketRef, data: Data<ws::JoinServerPayload>| async move {
                    ws::on_join_server(socket, data, hub.clone(), server_service.clone()).await;
                }
            });
              // Handler pour quitter un serveur
            socket.on("leave_server", {
                let hub = hub.clone();
                move |socket: SocketRef, data: Data<ws::LeaveServerPayload>| async move {
                    ws::on_leave_server(socket, data, hub.clone()).await;
                }
            });
              // Handler pour "typing"
            socket.on("typing_start", {
                let hub = hub.clone();
                move |socket: SocketRef, data: Data<ws::TypingPayload>| async move {
                    ws::on_typing_start(socket, data, hub.clone()).await;
                }
            });
              // Handler pour la déconnexion
            socket.on_disconnect({
                let hub = hub.clone();
                move |socket: SocketRef| async move {
                    ws::on_disconnect(socket, hub.clone()).await;
                }
            });
        }
    });

    // 7. Configuration CORS pour le frontend
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);    // 8. Créer le routeur avec toutes les routes REST
    let app = Router::new()
        // Route de test
        .route("/", get(|| async { "🚀 Chat RTC Backend opérationnel !" }))
        
        // Routes d'authentification
        .route("/auth/signup", post(handlers::signup))
        .route("/auth/login", post(handlers::login))
        .route("/auth/me", get(handlers::get_me))

       
        
        
        // Routes des serveurs
        .route("/servers", post(handlers::create_server))
        .route("/servers", get(handlers::list_servers))
        .route("/servers/join", post(handlers::join_server))
        .route("/servers/:id", get(handlers::get_server))
        .route("/servers/:id", put(handlers::update_server))
        .route("/servers/:id", delete(handlers::delete_server))
        .route("/servers/:id/leave", delete(handlers::leave_server))
        .route("/servers/:id/members", get(handlers::list_members))
        .route("/servers/:server_id/members/:user_id", put(handlers::update_member_role))
        
        // Routes des canaux
        .route("/servers/:server_id/channels", post(handlers::create_channel))
        .route("/servers/:server_id/channels", get(handlers::list_channels))
        .route("/channels/:id", get(handlers::get_channel))
        .route("/channels/:id", put(handlers::update_channel))
        .route("/channels/:id", delete(handlers::delete_channel))
        
        // Routes des messages
        .route("/channels/:channel_id/messages", post(handlers::create_message))
        .route("/channels/:channel_id/messages", get(handlers::get_messages))
        .route("/messages/:id", delete(handlers::delete_message))
        
        // Injection de l'état global avec tous les services
        .with_state(app_state)
        
        // Ajouter SocketIo comme Extension pour l'utiliser dans les handlers
        .layer(axum::Extension(io.clone()))
        
        // Middlewares
        .layer(cors)
        .layer(socket_layer);

    // 9. Lancer le serveur
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    info!("🚀 Serveur lancé sur http://{}", addr);
    info!("📚 Documentation API disponible sur http://{}/", addr);

    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}