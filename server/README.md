# Chat RTC - Backend Rust

Backend du projet Chat RTC développé en Rust avec Axum, SQLx et Socket.IO.

## 🏗️ Architecture

Ce projet suit une architecture **Clean Architecture / Architecture Hexagonale** pour garantir la maintenabilité, la testabilité et l'évolutivité.

### Structure des dossiers

```
src/
├── main.rs              # Point d'entrée, configuration du serveur
├── models/              # Couche Domaine : structures de données
│   ├── user.rs
│   ├── server.rs
│   ├── channel.rs
│   └── message.rs
├── repositories/        # Couche Accès aux Données : abstraction BDD
│   ├── user_repository.rs
│   ├── server_repository.rs
│   ├── channel_repository.rs
│   └── message_repository.rs
├── services/            # Couche Logique Métier : cas d'utilisation
│   ├── auth_service.rs
│   ├── server_service.rs
│   ├── channel_service.rs
│   └── message_service.rs
├── handlers/            # Couche Web : gestionnaires HTTP/WS
│   ├── auth_handler.rs
│   ├── server_handler.rs
│   ├── channel_handler.rs
│   ├── message_handler.rs
│   └── middleware.rs
├── utils/               # Utilitaires (JWT, hashing, etc.)
│   ├── jwt.rs
│   ├── password.rs
│   └── invitation_code.rs
├── ws/                  # WebSocket / Socket.IO
│   ├── hub.rs
│   └── events.rs
└── errors.rs            # Gestion centralisée des erreurs
```

### Flux de données

```
HTTP Request → Handler → Service → Repository → Database
                  ↓
             Middleware (Auth)
                  ↓
             Response/Error
```

### Principes clés

✅ **Séparation des préoccupations** : Chaque couche a une responsabilité unique  
✅ **Testabilité** : La logique métier peut être testée sans base de données  
✅ **Maintenabilité** : Facile d'ajouter de nouvelles fonctionnalités  
✅ **Évolutivité** : Possibilité de changer de BDD ou de framework facilement  

## 🚀 Technologies

- **Axum** : Framework web moderne et performant
- **SQLx** : Client PostgreSQL avec vérification à la compilation
- **Socket.IO** : Communication temps réel
- **JWT** : Authentification stateless
- **Bcrypt** : Hachage sécurisé des mots de passe
- **PostgreSQL** : Base de données relationnelle

## 📦 Installation

### Prérequis

- Rust 1.75+ ([installer](https://rustup.rs/))
- PostgreSQL 15+
- Docker & Docker Compose (optionnel)

### Configuration

1. Copier le fichier d'environnement :
```bash
cp .env.example .env
```

2. Modifier les variables dans `.env` selon votre configuration

3. Installer SQLx CLI (pour les migrations) :
```bash
cargo install sqlx-cli --no-default-features --features postgres
```

4. Créer la base de données et exécuter les migrations :
```bash
sqlx database create
sqlx migrate run
```

### Lancement

```bash
# Développement
cargo run

# Production (optimisé)
cargo build --release
./target/release/server
```

### Avec Docker

```bash
docker-compose up
```

## 🔌 API REST

### Authentification

| Méthode | Route | Description |
|---------|-------|-------------|
| POST | `/auth/signup` | Inscription |
| POST | `/auth/login` | Connexion |
| GET | `/auth/me` | Utilisateur actuel (🔒) |

### Serveurs

| Méthode | Route | Description |
|---------|-------|-------------|
| POST | `/servers` | Créer un serveur (🔒) |
| GET | `/servers` | Lister mes serveurs (🔒) |
| POST | `/servers/join` | Rejoindre un serveur (🔒) |
| GET | `/servers/:id` | Détails d'un serveur (🔒) |
| PUT | `/servers/:id` | Modifier un serveur (🔒 Admin) |
| DELETE | `/servers/:id` | Supprimer un serveur (🔒 Owner) |
| DELETE | `/servers/:id/leave` | Quitter un serveur (🔒) |
| GET | `/servers/:id/members` | Liste des membres (🔒) |
| PUT | `/servers/:id/members/:user_id` | Modifier le rôle (🔒 Owner) |

### Canaux

| Méthode | Route | Description |
|---------|-------|-------------|
| POST | `/servers/:id/channels` | Créer un canal (🔒 Admin) |
| GET | `/servers/:id/channels` | Liste des canaux (🔒) |
| GET | `/channels/:id` | Détails d'un canal (🔒) |
| PUT | `/channels/:id` | Modifier un canal (🔒 Admin) |
| DELETE | `/channels/:id` | Supprimer un canal (🔒 Admin) |

### Messages

| Méthode | Route | Description |
|---------|-------|-------------|
| POST | `/channels/:id/messages` | Envoyer un message (🔒) |
| GET | `/channels/:id/messages` | Historique (🔒) |
| DELETE | `/messages/:id` | Supprimer un message (🔒) |

🔒 = Nécessite un token JWT dans le header `Authorization: Bearer <token>`

## 🔄 Événements WebSocket

### Événements émis par le serveur

- `message:new` - Nouveau message
- `message:deleted` - Message supprimé
- `user:typing` - Utilisateur en train de taper
- `user:connected` - Utilisateur connecté
- `user:disconnected` - Utilisateur déconnecté
- `member:joined` - Nouveau membre
- `member:left` - Membre parti
- `channel:created` - Canal créé
- `channel:deleted` - Canal supprimé

## 🧪 Tests

```bash
# Tous les tests
cargo test

# Tests avec output détaillé
cargo test -- --nocapture

# Tests d'un module spécifique
cargo test services::auth_service
```

## 🛠️ Développement

### Ajouter une nouvelle feature

1. **Créer le modèle** dans `models/`
2. **Créer le repository** dans `repositories/`
3. **Créer le service** dans `services/`
4. **Créer le handler** dans `handlers/`
5. **Ajouter les routes** dans `main.rs`
6. **Tester** !

### Conventions de code

- Utiliser `cargo fmt` pour formater le code
- Utiliser `cargo clippy` pour détecter les problèmes
- Documenter les fonctions publiques avec `///`
- Suivre les conventions Rust standard

## 📝 Réponses aux questions d'architecture

### Où vit la logique métier ?
**Dans la couche `services/`**. Les services contiennent toutes les règles métier, validations et orchestration.

### Comment gérer l'accès à la base de données ?
**Via la couche `repositories/`**. Les repositories sont la seule couche qui connaît SQL et PostgreSQL.

### Comment tester le code ?
Les services peuvent être testés avec des mocks de repositories. Pas besoin de base de données réelle !

### Évolution de l'équipe ?
- Alice ajoute les features utilisateurs dans `services/auth_service.rs`
- Bob ajoute les features serveurs dans `services/server_service.rs`
- Pas de conflits !

### Changements de requirements ?
- Changer de BDD ? Remplacer `repositories/`
- Ajouter GraphQL ? Créer une nouvelle couche de handlers
- Facile car les couches sont découplées !

## 📚 Ressources

- [Axum Documentation](https://docs.rs/axum)
- [SQLx Documentation](https://docs.rs/sqlx)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)

## 👥 Contributeurs

Développé dans le cadre du projet Chat RTC - EPITECH

## 📄 Licence

Ce projet est développé à des fins éducatives.
