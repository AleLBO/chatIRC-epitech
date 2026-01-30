# Architecture du Backend - Réponses aux Questions du Projet

Ce document répond spécifiquement aux questions d'architecture posées dans le sujet du projet.

---

## 🎯 Où vit la logique métier ?

### Réponse : **Dans la couche `services/`**

La logique métier vit dans des **services dédiés**, complètement isolés des détails techniques (HTTP, base de données).

### Exemple concret

```rust
// ❌ MAUVAIS : Logique métier dans le handler
async fn create_server(Json(dto): Json<CreateServerDto>) -> Result<Json<Server>> {
    // Hash du mot de passe dans le handler
    let hash = bcrypt::hash(&dto.password)?;
    // Requête SQL directe dans le handler
    let server = sqlx::query!("INSERT INTO servers...").fetch_one().await?;
    // Vérification des permissions dans le handler
    if server.owner_id != user_id { return Err(...) }
    Ok(Json(server))
}

// ✅ BON : Logique métier dans le service
// Handler (handlers/server_handler.rs)
async fn create_server(
    State(service): State<Arc<ServerService>>,
    auth_user: AuthUser,
    Json(dto): Json<CreateServerDto>
) -> AppResult<Json<Server>> {
    let server = service.create_server(dto, auth_user.user_id).await?;
    Ok(Json(server))
}

// Service (services/server_service.rs)
pub async fn create_server(&self, dto: CreateServerDto, owner_id: i32) -> AppResult<Server> {
    // LOGIQUE MÉTIER ICI :
    // 1. Génération du code d'invitation
    let invitation_code = generate_invitation_code();
    
    // 2. Création du serveur via le repository
    let server = self.server_repo.create(&dto.name, owner_id, &invitation_code).await?;
    
    // 3. Ajout automatique du créateur comme Owner
    self.server_repo.add_member(server.id, owner_id, UserRole::Owner).await?;
    
    Ok(server)
}
```

### Avantages

- ✅ Testable sans HTTP (pas besoin de simuler des requêtes)
- ✅ Réutilisable (peut être appelé depuis WebSocket, CLI, tests...)
- ✅ Un seul endroit pour les règles métier
- ✅ Facile à modifier sans casser les handlers

---

## 🗄️ Comment gérer l'accès à la base de données ?

### Réponse : **Via une couche `repositories/` séparée**

Les repositories sont la **seule couche** qui connaît SQL et PostgreSQL. Ils fournissent une abstraction claire.

### Architecture en couches

```
Service  →  Repository  →  Database
(Métier)    (SQL/BDD)      (PostgreSQL)
```

### Exemple concret

```rust
// Repository (repositories/user_repository.rs)
pub struct UserRepository {
    pool: PgPool,  // Seul endroit qui connaît la BDD
}

impl UserRepository {
    pub async fn create(&self, dto: CreateUserDto, password_hash: &str) -> AppResult<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, username, email, password_hash, created_at
            "#,
            dto.username,
            dto.email,
            password_hash
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn find_by_username(&self, username: &str) -> AppResult<Option<User>> {
        // SQL ici aussi
    }
}
```

### Avantages

- ✅ Changer de PostgreSQL vers MongoDB ? Remplacer juste les repositories
- ✅ Tester les services avec des repositories mockés (pas de vraie BDD)
- ✅ Requêtes SQL centralisées et réutilisables
- ✅ Vérification des requêtes à la compilation avec SQLx

---

## 🧪 Comment tester le code ?

### Réponse : **Tests unitaires des services avec mocks, tests d'intégration pour les repositories**

### 1. Tests unitaires des services (SANS base de données)

```rust
// tests/auth_service_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock du repository
    struct MockUserRepository {
        users: Vec<User>,
    }
    
    #[async_trait]
    impl UserRepositoryTrait for MockUserRepository {
        async fn find_by_username(&self, username: &str) -> AppResult<Option<User>> {
            Ok(self.users.iter().find(|u| u.username == username).cloned())
        }
        // ... autres méthodes mockées
    }
    
    #[tokio::test]
    async fn test_register_user_success() {
        // Arrange
        let mock_repo = Arc::new(MockUserRepository { users: vec![] });
        let service = AuthService::new(mock_repo);
        
        let dto = CreateUserDto {
            username: "john".to_string(),
            email: "john@example.com".to_string(),
            password: "password123".to_string(),
        };
        
        // Act
        let result = service.register(dto).await;
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_register_duplicate_username_fails() {
        // Arrange
        let existing_user = User { /* ... */ };
        let mock_repo = Arc::new(MockUserRepository { users: vec![existing_user] });
        let service = AuthService::new(mock_repo);
        
        // Act
        let result = service.register(/* dto with same username */).await;
        
        // Assert
        assert!(matches!(result, Err(AppError::UsernameTaken)));
    }
}
```

### 2. Tests d'intégration (AVEC base de données de test)

```rust
// tests/integration_test.rs
#[tokio::test]
async fn test_full_user_flow() {
    // Setup base de données de test
    let pool = PgPool::connect("postgres://test_db").await.unwrap();
    
    let user_repo = Arc::new(UserRepository::new(pool));
    let auth_service = AuthService::new(user_repo);
    
    // Test du flow complet
    let signup = auth_service.register(/* ... */).await;
    assert!(signup.is_ok());
    
    let login = auth_service.login(/* ... */).await;
    assert!(login.is_ok());
}
```

### Commandes de test

```bash
# Tous les tests
cargo test

# Tests unitaires seulement (rapides)
cargo test --lib

# Tests d'intégration (plus lents)
cargo test --test '*'

# Un module spécifique
cargo test services::auth_service

# Avec output détaillé
cargo test -- --nocapture
```

---

## 👥 Que se passe-t-il quand l'équipe grandit ?

### Réponse : **Architecture modulaire = pas de conflits**

### Scénario concret

```
Alice travaille sur les features utilisateurs
├── models/user.rs
├── repositories/user_repository.rs
├── services/auth_service.rs
└── handlers/auth_handler.rs

Bob travaille sur les features serveurs
├── models/server.rs
├── repositories/server_repository.rs
├── services/server_service.rs
└── handlers/server_handler.rs

Charlie travaille sur les messages
├── models/message.rs
├── repositories/message_repository.rs
├── services/message_service.rs
└── handlers/message_handler.rs
```

### Avantages

- ✅ **Chacun travaille dans ses fichiers** → pas de conflits Git
- ✅ **Responsabilités claires** → on sait où ajouter du code
- ✅ **Modules indépendants** → on peut développer en parallèle
- ✅ **Code reviews faciles** → changements isolés

### Exemple : Ajouter une feature "réactions aux messages"

Charlie sait exactement où aller :
1. Créer `models/reaction.rs`
2. Créer `repositories/reaction_repository.rs`
3. Créer `services/reaction_service.rs`
4. Créer `handlers/reaction_handler.rs`
5. Ajouter les routes dans `main.rs`

**Aucun impact sur le code d'Alice ou Bob !**

---

## 🔄 Que se passe-t-il si les requirements changent ?

### Réponse : **Couplage faible = changements faciles**

### Scénario 1 : Passer de PostgreSQL à MongoDB

```rust
// Avant (PostgreSQL)
pub struct UserRepository {
    pool: PgPool,
}

// Après (MongoDB)
pub struct UserRepository {
    collection: Collection<User>,
}

// Le trait reste le même !
#[async_trait]
pub trait UserRepositoryTrait {
    async fn find_by_username(&self, username: &str) -> AppResult<Option<User>>;
}

// Les services ne changent PAS !
// Les handlers ne changent PAS !
// Seule l'implémentation du repository change
```

### Scénario 2 : Ajouter GraphQL à côté de REST

```
src/
├── handlers/          # Handlers REST existants (ne bougent pas)
│   └── auth_handler.rs
├── graphql/           # Nouveaux handlers GraphQL
│   └── auth_resolver.rs
└── services/          # Services partagés entre REST et GraphQL !
    └── auth_service.rs
```

Les deux peuvent utiliser les mêmes services !

### Scénario 3 : Ajouter une authentification OAuth

```rust
// Ajouter une méthode au service
impl AuthService {
    // Méthode existante (ne change pas)
    pub async fn login(&self, dto: LoginDto) -> AppResult<AuthResponse> { /* ... */ }
    
    // NOUVELLE méthode
    pub async fn login_with_google(&self, google_token: String) -> AppResult<AuthResponse> {
        // Valider le token Google
        // Créer ou récupérer l'utilisateur
        // Générer notre token JWT
    }
}

// Nouveau handler
async fn google_login(/* ... */) -> AppResult<Json<AuthResponse>> {
    let response = auth_service.login_with_google(token).await?;
    Ok(Json(response))
}
```

**Changement minimal, code existant préservé !**

---

## ⚠️ Signes d'une mauvaise architecture (que nous évitons)

### ❌ "Je ne peux pas tester sans base de données"
**Solution :** Services testables avec des mocks de repositories

### ❌ "Changer une chose casse trois autres choses"
**Solution :** Couplage faible entre les couches

### ❌ "Je ne sais pas où mettre cette nouvelle feature"
**Solution :** Structure claire : models → repositories → services → handlers

### ❌ "Mes handlers font 200+ lignes"
**Solution :** Logique métier dans les services, handlers simples (10-30 lignes)

### ❌ "Tout est dans un fichier géant"
**Solution :** Modules séparés par domaine (user, server, channel, message)

### ❌ "Je copie le même code partout"
**Solution :** Réutilisation via services et utilities

---

## ✅ Signes d'une bonne architecture (que nous avons)

### ✅ Tester la logique métier sans HTTP/database
```bash
cargo test services::auth_service  # Rapide, pas de BDD
```

### ✅ Chaque fichier a UNE responsabilité claire
- `auth_service.rs` → Logique d'authentification
- `user_repository.rs` → Accès BDD pour les users
- `auth_handler.rs` → Entrée/sortie HTTP

### ✅ Changer de BDD facilement
Remplacer `repositories/` sans toucher `services/` ou `handlers/`

### ✅ Nouveaux membres comprennent rapidement
Structure intuitive et documentation claire

### ✅ Ajouter des features sans tout réécrire
Modules indépendants et extensibles

---

## 📊 Récapitulatif de l'architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Client (Next.js)                      │
└─────────────────────────────────────────────────────────┘
                          ↓ HTTP/WS
┌─────────────────────────────────────────────────────────┐
│                  HANDLERS (Couche Web)                   │
│  • Extraction des données HTTP                           │
│  • Validation basique                                    │
│  • Appel du service approprié                            │
│  • Formatage de la réponse                               │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│               SERVICES (Logique Métier)                  │
│  • Règles métier                                         │
│  • Validations complexes                                 │
│  • Orchestration                                         │
│  • Permissions                                           │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│            REPOSITORIES (Accès aux Données)              │
│  • Requêtes SQL                                          │
│  • Abstraction de la BDD                                 │
│  • CRUD operations                                       │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│                   PostgreSQL Database                    │
└─────────────────────────────────────────────────────────┘
```

**Flux de données :** HTTP → Handler → Service → Repository → Database

**Principe clé :** Chaque couche ne connaît que celle juste en dessous !

---

## 🎓 Conclusion

Cette architecture vous permet de :

1. ✅ **Développer rapidement** avec une structure claire
2. ✅ **Travailler en équipe** sans conflits
3. ✅ **Tester facilement** avec ou sans base de données
4. ✅ **Évoluer sereinement** quand les requirements changent
5. ✅ **Maintenir le code** sur le long terme

**"Weeks of coding can save you hours of planning."**

Nous avons planifié, et maintenant le développement est simple et rapide ! 🚀
