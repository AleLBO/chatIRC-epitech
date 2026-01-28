use axum::{routing::get, Router};
use socketioxide::{extract::SocketRef, SocketIo};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialiser les logs (pour voir ce qui se passe dans la console Docker)
    tracing_subscriber::fmt().init();

    // 2. Initialiser Socket.IO
    let (layer, io) = SocketIo::new_layer();

    // Petit test : quand quelqu'un se connecte via WebSocket
    io.ns("/", |socket: SocketRef| {
        info!("Nouveau client connecté via Socket.IO : {}", socket.id);
    });

    // 3. Initialiser le routeur Axum (API REST)
    let app = Router::new()
        .route("/", get(|| async { "Serveur Rust opérationnel !" })) // Route de test
        .layer(layer); // Ajout de la couche Socket.IO

    // 4. Lancer le serveur
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    info!("Serveur lancé sur http://{}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}