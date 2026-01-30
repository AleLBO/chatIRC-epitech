// Exports des modules de modèles
pub mod user;
pub mod server;
pub mod channel;
pub mod message;

// Re-exports pour faciliter l'usage
pub use user::{User, UserRole, CreateUserDto, LoginDto, AuthResponse};
pub use server::{Server, CreateServerDto, JoinServerDto, ServerMember, ServerMemberDetails};
pub use channel::{Channel, CreateChannelDto, UpdateChannelDto};
pub use message::{Message, CreateMessageDto, MessageWithAuthor};
