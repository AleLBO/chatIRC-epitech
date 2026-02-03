# 🧪 Guide de Test WebSocket

Ce guide vous aide à tester la fonctionnalité WebSocket de votre backend.

## 📋 Prérequis

1. Backend lancé sur `http://localhost:3000`
2. Base de données PostgreSQL configurée
3. Au moins un utilisateur créé via `/auth/signup`

---

## 🚀 Option 1 : Test avec la Page HTML

### Étape 1 : Ouvrir la page de test

```bash
# Depuis le dossier server
open test-websocket.html
# Ou double-cliquez sur le fichier
```

### Étape 2 : Se connecter

1. **Créer un compte ou se connecter** :
```bash
curl -X POST http://localhost:3000/auth/signup \
  -H "Content-Type: application/json" \
  -d '{
    "username": "test_user",
    "email": "test@example.com",
    "password": "password123"
  }'
```

2. **Copier le token JWT** de la réponse

3. **Coller le token** dans le champ de la page HTML

4. **Cliquer sur "Se connecter"**

### Étape 3 : Rejoindre un serveur

1. **Créer un serveur** (si pas déjà fait) :
```bash
TOKEN="votre_token_ici"

curl -X POST http://localhost:3000/servers \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name": "Test Server"}'
```

2. **Noter le server_id** (ex: 1)

3. **Entrer le server_id** dans la page HTML

4. **Cliquer sur "Rejoindre"**

### Étape 4 : Tester les messages

1. **Créer un canal** (si pas déjà fait) :
```bash
curl -X POST http://localhost:3000/servers/1/channels \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name": "general"}'
```

2. **Modifier le `currentChannelId`** dans le code HTML (ligne 175)

3. **Taper un message** et appuyer sur Entrée

4. **Ouvrir plusieurs fenêtres** pour voir les messages en temps réel

---

## 🧪 Option 2 : Test avec du JavaScript pur

### Script de test complet

```javascript
// test-socket.js
const io = require('socket.io-client');

// Configuration
const SERVER_URL = 'http://localhost:3000';
const TOKEN = 'VOTRE_TOKEN_JWT_ICI';
const SERVER_ID = 1;
const CHANNEL_ID = 1;

// Connexion
const socket = io(SERVER_URL, {
    transports: ['websocket', 'polling']
});

socket.on('connect', () => {
    console.log('✓ Connecté au serveur WebSocket');
    
    // S'authentifier
    socket.emit('authenticate', { token: TOKEN });
});

socket.on('authenticated', (data) => {
    if (data.success) {
        console.log(`✓ Authentifié: ${data.username}`);
        
        // Rejoindre un serveur
        socket.emit('join_server', { server_id: SERVER_ID });
    } else {
        console.error(`✗ Erreur auth: ${data.error}`);
    }
});

socket.on('server:joined', (data) => {
    console.log(`✓ Serveur rejoint: ${data.server_id}`);
    console.log(`Utilisateurs connectés: ${data.connected_users}`);
});

socket.on('message:new', (data) => {
    console.log(`📨 Nouveau message de ${data.author_username}: ${data.content}`);
});

socket.on('user:connected', (data) => {
    console.log(`👤 ${data.username} s'est connecté`);
});

socket.on('user:disconnected', (data) => {
    console.log(`👤 ${data.username} s'est déconnecté`);
});

socket.on('user:typing', (data) => {
    console.log(`⌨️  ${data.username} est en train d'écrire...`);
});

socket.on('error', (data) => {
    console.error(`✗ Erreur: ${data.error}`);
});

socket.on('disconnect', () => {
    console.log('✗ Déconnecté');
});

// Simuler "typing" après 2 secondes
setTimeout(() => {
    console.log('Envoi événement typing...');
    socket.emit('typing_start', { channel_id: CHANNEL_ID });
}, 2000);
```

### Installation et exécution

```bash
# Installer socket.io-client
npm install socket.io-client

# Modifier TOKEN, SERVER_ID, CHANNEL_ID dans le fichier

# Lancer
node test-socket.js
```

---

## 🔍 Vérification des Événements

### 1. Authentification

**Client envoie :**
```json
{
  "token": "eyJhbGc..."
}
```

**Serveur répond :**
```json
{
  "success": true,
  "user_id": 1,
  "username": "test_user"
}
```

### 2. Rejoindre un serveur

**Client envoie :**
```json
{
  "server_id": 1
}
```

**Serveur répond :**
```json
{
  "server_id": 1,
  "connected_users": [1, 2, 3]
}
```

**Broadcast aux autres :**
```json
{
  "type": "user:connected",
  "server_id": 1,
  "user_id": 1,
  "username": "test_user"
}
```

### 3. Nouveau message

**Via API REST :**
```bash
curl -X POST http://localhost:3000/channels/1/messages \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"content": "Hello World!"}'
```

**Broadcast WebSocket :**
```json
{
  "type": "message:new",
  "channel_id": 1,
  "message_id": 42,
  "content": "Hello World!",
  "author_id": 1,
  "author_username": "test_user",
  "created_at": "2026-01-29T10:30:00Z"
}
```

### 4. Typing

**Client envoie :**
```json
{
  "channel_id": 1
}
```

**Broadcast aux autres :**
```json
{
  "type": "user:typing",
  "channel_id": 1,
  "user_id": 1,
  "username": "test_user"
}
```

---

## 📊 Scénario de Test Complet

### Préparation (API REST)

```bash
TOKEN1="token_user1"
TOKEN2="token_user2"

# User 1 crée un serveur
SERVER_RESPONSE=$(curl -s -X POST http://localhost:3000/servers \
  -H "Authorization: Bearer $TOKEN1" \
  -H "Content-Type: application/json" \
  -d '{"name": "Test WebSocket"}')

SERVER_ID=$(echo $SERVER_RESPONSE | jq -r '.id')
INVITE_CODE=$(echo $SERVER_RESPONSE | jq -r '.invitation_code')

# User 1 crée un canal
CHANNEL_RESPONSE=$(curl -s -X POST http://localhost:3000/servers/$SERVER_ID/channels \
  -H "Authorization: Bearer $TOKEN1" \
  -H "Content-Type: application/json" \
  -d '{"name": "general"}')

CHANNEL_ID=$(echo $CHANNEL_RESPONSE | jq -r '.id')

# User 2 rejoint le serveur
curl -X POST http://localhost:3000/servers/join \
  -H "Authorization: Bearer $TOKEN2" \
  -H "Content-Type: application/json" \
  -d "{\"invitation_code\": \"$INVITE_CODE\"}"

echo "SERVER_ID: $SERVER_ID"
echo "CHANNEL_ID: $CHANNEL_ID"
```

### Test WebSocket

1. **Ouvrir 2 fenêtres de navigateur** avec `test-websocket.html`

2. **Fenêtre 1 :**
   - Entrer TOKEN1
   - Se connecter
   - Rejoindre server avec SERVER_ID

3. **Fenêtre 2 :**
   - Entrer TOKEN2
   - Se connecter
   - Rejoindre le même server

4. **Vérifier :**
   - Fenêtre 1 voit "User X s'est connecté"
   - Les deux fenêtres voient la liste des utilisateurs

5. **Taper un message dans Fenêtre 1**
   - Vérifier qu'il apparaît dans Fenêtre 2 en temps réel

6. **Taper dans Fenêtre 2**
   - Vérifier que Fenêtre 1 voit "User Y est en train d'écrire..."

---

## ⚠️ Résolution de Problèmes

### Erreur : "Connection refused"

→ Vérifiez que le serveur est lancé :
```bash
curl http://localhost:3000
```

### Erreur : "Invalid token"

→ Vérifiez que le token est valide :
```bash
curl http://localhost:3000/auth/me \
  -H "Authorization: Bearer $TOKEN"
```

### Erreur : "Not a member of this server"

→ Vérifiez que vous êtes bien membre :
```bash
curl http://localhost:3000/servers/$SERVER_ID \
  -H "Authorization: Bearer $TOKEN"
```

### Les messages n'apparaissent pas

→ Vérifiez :
1. Que le CHANNEL_ID est correct
2. Que vous êtes dans la même room
3. Les logs du serveur pour voir les broadcasts

### CORS errors

→ Le backend a déjà CORS configuré, mais vérifiez dans main.rs :
```rust
let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);
```

---

## 📝 Checklist de Test

- [ ] Connexion WebSocket fonctionne
- [ ] Authentification avec JWT
- [ ] Rejoindre un serveur
- [ ] Voir les utilisateurs connectés
- [ ] Recevoir les nouveaux messages en temps réel
- [ ] Événement "typing" fonctionne
- [ ] Notifications de connexion/déconnexion
- [ ] Plusieurs utilisateurs simultanés
- [ ] Messages persistent en base de données
- [ ] Historique des messages via REST

---

## 🎯 Prochaines Étapes

Une fois les tests WebSocket validés :

1. ✅ Backend complet et fonctionnel
2. 🔄 Intégrer au frontend Next.js
3. 🎨 Créer une belle UI
4. 🚀 Déployer en production

**Félicitations ! Votre backend temps réel fonctionne ! 🎉**
