# 🧪 Exemples de Requêtes API

Ce fichier contient des exemples concrets pour tester tous les endpoints de l'API.

## Variables à utiliser

```bash
BASE_URL="http://localhost:3000"
TOKEN="votre_token_jwt_ici"
```

---

## 📝 Authentication

### 1. Inscription

```bash
curl -X POST $BASE_URL/auth/signup \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "email": "john@example.com",
    "password": "password123"
  }'
```

**Réponse attendue (201 Created):**
```json
{
  "user": {
    "id": 1,
    "username": "john_doe",
    "email": "john@example.com",
    "created_at": "2026-01-28T10:30:00Z"
  },
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

### 2. Connexion

```bash
curl -X POST $BASE_URL/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "password": "password123"
  }'
```

### 3. Utilisateur actuel (authentifié)

```bash
curl $BASE_URL/auth/me \
  -H "Authorization: Bearer $TOKEN"
```

---

## 🏰 Servers

### 1. Créer un serveur

```bash
curl -X POST $BASE_URL/servers \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Mon Super Serveur"
  }'
```

**Réponse (201):**
```json
{
  "id": 1,
  "name": "Mon Super Serveur",
  "invitation_code": "ABC12345",
  "owner_id": 1,
  "created_at": "2026-01-28T10:30:00Z"
}
```

### 2. Lister mes serveurs

```bash
curl $BASE_URL/servers \
  -H "Authorization: Bearer $TOKEN"
```

### 3. Détails d'un serveur

```bash
curl $BASE_URL/servers/1 \
  -H "Authorization: Bearer $TOKEN"
```

### 4. Modifier un serveur (Admin/Owner)

```bash
curl -X PUT $BASE_URL/servers/1 \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Nouveau nom du serveur"
  }'
```

### 5. Rejoindre un serveur avec code d'invitation

```bash
curl -X POST $BASE_URL/servers/join \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "invitation_code": "ABC12345"
  }'
```

### 6. Lister les membres d'un serveur

```bash
curl $BASE_URL/servers/1/members \
  -H "Authorization: Bearer $TOKEN"
```

**Réponse:**
```json
[
  {
    "user_id": 1,
    "username": "john_doe",
    "role": "OWNER",
    "joined_at": "2026-01-28T10:30:00Z"
  },
  {
    "user_id": 2,
    "username": "alice",
    "role": "MEMBER",
    "joined_at": "2026-01-28T11:00:00Z"
  }
]
```

### 7. Modifier le rôle d'un membre (Owner only)

```bash
curl -X PUT $BASE_URL/servers/1/members/2 \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "role": "ADMIN"
  }'
```

### 8. Quitter un serveur

```bash
curl -X DELETE $BASE_URL/servers/1/leave \
  -H "Authorization: Bearer $TOKEN"
```

### 9. Supprimer un serveur (Owner only)

```bash
curl -X DELETE $BASE_URL/servers/1 \
  -H "Authorization: Bearer $TOKEN"
```

---

## 📺 Channels

### 1. Créer un canal (Admin/Owner)

```bash
curl -X POST $BASE_URL/servers/1/channels \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "general",
    "channel_type": "text"
  }'
```

**Réponse (201):**
```json
{
  "id": 1,
  "name": "general",
  "channel_type": "text",
  "server_id": 1,
  "created_at": "2026-01-28T10:30:00Z"
}
```

### 2. Lister les canaux d'un serveur

```bash
curl $BASE_URL/servers/1/channels \
  -H "Authorization: Bearer $TOKEN"
```

### 3. Détails d'un canal

```bash
curl $BASE_URL/channels/1 \
  -H "Authorization: Bearer $TOKEN"
```

### 4. Modifier un canal (Admin/Owner)

```bash
curl -X PUT $BASE_URL/channels/1 \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "general-chat"
  }'
```

### 5. Supprimer un canal (Admin/Owner)

```bash
curl -X DELETE $BASE_URL/channels/1 \
  -H "Authorization: Bearer $TOKEN"
```

---

## 💬 Messages

### 1. Envoyer un message

```bash
curl -X POST $BASE_URL/channels/1/messages \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Hello world! 👋"
  }'
```

**Réponse (201):**
```json
{
  "id": 1,
  "content": "Hello world! 👋",
  "channel_id": 1,
  "author_id": 1,
  "is_deleted": false,
  "created_at": "2026-01-28T10:30:00Z",
  "updated_at": null
}
```

### 2. Récupérer l'historique des messages

```bash
# Sans pagination (50 derniers messages par défaut)
curl $BASE_URL/channels/1/messages \
  -H "Authorization: Bearer $TOKEN"

# Avec pagination
curl "$BASE_URL/channels/1/messages?limit=20&offset=0" \
  -H "Authorization: Bearer $TOKEN"
```

**Réponse:**
```json
[
  {
    "id": 1,
    "content": "Hello world! 👋",
    "channel_id": 1,
    "author_id": 1,
    "author_username": "john_doe",
    "is_deleted": false,
    "created_at": "2026-01-28T10:30:00Z",
    "updated_at": null
  }
]
```

### 3. Supprimer un message

```bash
curl -X DELETE $BASE_URL/messages/1 \
  -H "Authorization: Bearer $TOKEN"
```

---

## 🧪 Scénario Complet de Test

### Étape 1 : Créer deux utilisateurs

```bash
# Utilisateur 1 (John)
curl -X POST $BASE_URL/auth/signup \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "email": "john@example.com",
    "password": "password123"
  }'

# Sauvegarder le token dans TOKEN1

# Utilisateur 2 (Alice)
curl -X POST $BASE_URL/auth/signup \
  -H "Content-Type: application/json" \
  -d '{
    "username": "alice",
    "email": "alice@example.com",
    "password": "password123"
  }'

# Sauvegarder le token dans TOKEN2
```

### Étape 2 : John crée un serveur

```bash
curl -X POST $BASE_URL/servers \
  -H "Authorization: Bearer $TOKEN1" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Serveur de Test"
  }'

# Sauvegarder le SERVER_ID et INVITATION_CODE
```

### Étape 3 : John crée un canal

```bash
curl -X POST $BASE_URL/servers/$SERVER_ID/channels \
  -H "Authorization: Bearer $TOKEN1" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "general"
  }'

# Sauvegarder le CHANNEL_ID
```

### Étape 4 : Alice rejoint le serveur

```bash
curl -X POST $BASE_URL/servers/join \
  -H "Authorization: Bearer $TOKEN2" \
  -H "Content-Type: application/json" \
  -d '{
    "invitation_code": "'$INVITATION_CODE'"
  }'
```

### Étape 5 : Les deux envoient des messages

```bash
# John envoie un message
curl -X POST $BASE_URL/channels/$CHANNEL_ID/messages \
  -H "Authorization: Bearer $TOKEN1" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Bienvenue sur le serveur !"
  }'

# Alice répond
curl -X POST $BASE_URL/channels/$CHANNEL_ID/messages \
  -H "Authorization: Bearer $TOKEN2" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Merci John ! 😊"
  }'
```

### Étape 6 : John promeut Alice en Admin

```bash
curl -X PUT $BASE_URL/servers/$SERVER_ID/members/2 \
  -H "Authorization: Bearer $TOKEN1" \
  -H "Content-Type: application/json" \
  -d '{
    "role": "ADMIN"
  }'
```

### Étape 7 : Alice crée un nouveau canal (maintenant qu'elle est admin)

```bash
curl -X POST $BASE_URL/servers/$SERVER_ID/channels \
  -H "Authorization: Bearer $TOKEN2" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "random"
  }'
```

---

## ⚠️ Tests d'Erreurs

### Inscription avec username déjà pris (409 Conflict)

```bash
curl -X POST $BASE_URL/auth/signup \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "email": "autre@example.com",
    "password": "password123"
  }'
```

### Connexion avec mauvais mot de passe (401 Unauthorized)

```bash
curl -X POST $BASE_URL/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "password": "wrongpassword"
  }'
```

### Accès sans token (401 Unauthorized)

```bash
curl $BASE_URL/auth/me
```

### Accès à un serveur dont on n'est pas membre (403 Forbidden)

```bash
curl $BASE_URL/servers/999 \
  -H "Authorization: Bearer $TOKEN"
```

### Créer un canal sans être admin (403 Forbidden)

```bash
# TOKEN d'un membre simple
curl -X POST $BASE_URL/servers/1/channels \
  -H "Authorization: Bearer $TOKEN_MEMBER" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "nouveau-canal"
  }'
```

### Owner essaie de quitter son serveur (400 Bad Request)

```bash
curl -X DELETE $BASE_URL/servers/1/leave \
  -H "Authorization: Bearer $TOKEN_OWNER"
```

---

## 🔧 Utiliser avec Postman

1. Importez ces exemples dans Postman
2. Créez une collection "Chat RTC"
3. Ajoutez une variable d'environnement `BASE_URL` = `http://localhost:3000`
4. Ajoutez une variable `TOKEN` que vous mettrez à jour après login
5. Utilisez `{{BASE_URL}}` et `{{TOKEN}}` dans vos requêtes

---

## 📊 Tests de Performance

### Créer 100 messages rapidement

```bash
for i in {1..100}; do
  curl -X POST $BASE_URL/channels/1/messages \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d "{\"content\": \"Message numéro $i\"}" &
done
wait
```

### Récupérer l'historique complet

```bash
curl "$BASE_URL/channels/1/messages?limit=100&offset=0" \
  -H "Authorization: Bearer $TOKEN"
```

---

## 💡 Conseils

1. **Sauvegardez les tokens** après signup/login pour les réutiliser
2. **Testez les cas d'erreur** autant que les cas de succès
3. **Vérifiez les permissions** (member vs admin vs owner)
4. **Utilisez jq** pour formater le JSON : `curl ... | jq`
5. **Regardez les logs du serveur** pour déboguer

```bash
# Exemple avec jq
curl $BASE_URL/auth/signup \
  -H "Content-Type: application/json" \
  -d '{"username": "test", "email": "test@test.com", "password": "test123"}' \
  | jq '.token' -r
```

---

**Bon test ! 🧪**
