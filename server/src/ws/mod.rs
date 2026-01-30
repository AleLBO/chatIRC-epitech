// Module WebSocket pour la communication en temps réel
// Ce module gérera les connexions Socket.IO et la diffusion des événements

pub mod hub;
pub mod events;
pub mod handlers;

pub use hub::{Hub, UserInfo};
pub use events::SocketEvent;
pub use handlers::*;
