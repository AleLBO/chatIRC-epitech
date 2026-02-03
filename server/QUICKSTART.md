# 🚀 Guide de Démarrage Rapide

## Installation de Rust (si pas déjà fait)

```bash
# Installer Rust et Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Recharger votre shell
source $HOME/.cargo/env

# Vérifier l'installation
cargo --version
```

## Option 1 : Avec Docker (Recommandé)

C'est la méthode la plus simple, tout est configuré automatiquement.

```bash
# Depuis la racine du projet
cd /Users/shakzk/Desktop/chatIRC-epitech

# Lancer tout (backend + base de données)
docker-compose up

# Le serveur sera accessible sur http://localhost:3000
```

## Option 2 : En local (Développement)

### Prérequis

- Rust 1.75+
- PostgreSQL 15+ installé et lancé

### Étapes

```bash
# 1. Aller dans le dossier server
cd server

# 2. Installer SQLx CLI (pour les migrations)
cargo install sqlx-cli --no-default-features --features postgres

# 3. Copier et configurer les variables d'environnement
cp .env.example .env
# Éditez .env avec vos paramètres PostgreSQL

# 4. Créer la base de données
sqlx database create

# 5. Exécuter les migrations SQL
sqlx migrate run

# 6. Lancer le serveur
cargo run

# Le serveur sera accessible sur http://localhost:3000
```

## Tester que tout fonctionne

### 1. Test de base

```bash
curl http://localhost:3000
# Devrait retourner : "🚀 Chat RTC Backend opérationnel !"
```

### 2. Inscription d'un utilisateur

```bash
curl -X POST http://localhost:3000/auth/signup \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "email": "john@example.com",
    "password": "password123"
  }'
```

Vous devriez recevoir un objet JSON avec l'utilisateur et un token JWT.

### 3. Tester l'authentification

```bash
# Utilisez le token reçu précédemment
curl http://localhost:3000/auth/me \
  -H "Authorization: Bearer VOTRE_TOKEN_ICI"
```

## Structure du Projet

```
server/
├── src/
│   ├── main.rs              # Point d'entrée
│   ├── models/              # Structures de données
│   ├── repositories/        # Accès base de données
│   ├── services/            # Logique métier
│   ├── handlers/            # Routes HTTP
│   ├── utils/               # Utilitaires (JWT, etc.)
│   ├── ws/                  # WebSocket
│   └── errors.rs            # Gestion des erreurs
├── database/
│   └── init.sql             # Schéma de la base de données
├── Cargo.toml               # Dépendances Rust
├── .env                     # Configuration (ne pas commit)
├── README.md                # Documentation générale
├── ARCHITECTURE.md          # Explications architecture
└── SOCKET_SPEC.md           # Spécification WebSocket
```

## Commandes Utiles

### Développement

```bash
# Compiler sans exécuter
cargo build

# Exécuter avec logs détaillés
RUST_LOG=debug cargo run

# Exécuter les tests
cargo test

# Vérifier le code (rapide, sans compiler)
cargo check

# Formatter le code
cargo fmt

# Linter (détection de problèmes)
cargo clippy
```

### Base de données

```bash
# Créer la base de données
sqlx database create

# Supprimer la base de données
sqlx database drop

# Créer une nouvelle migration
sqlx migrate add nom_de_la_migration

# Exécuter les migrations
sqlx migrate run

# Revenir en arrière (une migration)
sqlx migrate revert
```

## Prochaines Étapes

1. **Lire la documentation** : `README.md`, `ARCHITECTURE.md`, `SOCKET_SPEC.md`
2. **Tester les endpoints** : Utilisez Postman, Insomnia, ou curl
3. **Implémenter le WebSocket** : Suivre `SOCKET_SPEC.md`
4. **Connecter le frontend** : Next.js dans `/client`
5. **Ajouter les tests** : Dans `tests/`

## Résolution de Problèmes

### Erreur : "DATABASE_URL must be set"

→ Créez un fichier `.env` avec `DATABASE_URL=postgres://...`

### Erreur : "connection refused" (PostgreSQL)

→ Assurez-vous que PostgreSQL est lancé :
```bash
# macOS
brew services start postgresql

# Linux
sudo systemctl start postgresql

# Docker
docker-compose up db
```

### Erreur : "table does not exist"

→ Exécutez les migrations :
```bash
sqlx migrate run
```

### Le serveur ne démarre pas sur le port 3000

→ Changez le port dans `.env` :
```
PORT=8080
```

## Aide et Support

- Documentation Rust : https://doc.rust-lang.org/book/
- Documentation Axum : https://docs.rs/axum
- Documentation SQLx : https://docs.rs/sqlx

## 📝 Checklist de démarrage

- [ ] Rust installé (`cargo --version`)
- [ ] PostgreSQL installé et lancé
- [ ] Fichier `.env` configuré
- [ ] Base de données créée (`sqlx database create`)
- [ ] Migrations exécutées (`sqlx migrate run`)
- [ ] Serveur lance sans erreur (`cargo run`)
- [ ] Test endpoint root (`curl http://localhost:3000`)
- [ ] Test inscription utilisateur
- [ ] Documentation lue

Bon développement ! 🎉
