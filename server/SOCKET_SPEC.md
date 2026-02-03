# Spécification Socket.IO - Chat RTC

Ce document décrit tous les événements Socket.IO utilisés dans l'application Chat RTC.

## 🔌 Connexion

### Client → Serveur

#### `authenticate`
Authentifie un utilisateur via Socket.IO.

**Payload:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Réponse (Success):**
```json
{
  "success": true,
  "user_id": 1,
  "username": "john_doe"
}
```

**Réponse (Error):**
```json
{
  "success": false,
  "error": "Invalid token"
}
```

---

#### `join_server`
Rejoindre un serveur pour recevoir ses événements.

**Payload:**
```json
{
  "server_id": 1
}
```

**Réponse:**
```json
{
  "success": true,
  "connected_users": [1, 2, 5, 8]
}
```

---

#### `leave_server`
Quitter un serveur.

**Payload:**
```json
{
  "server_id": 1
}
```

---

## 💬 Messages

### Client → Serveur

#### `typing_start`
Indique que l'utilisateur commence à taper.

**Payload:**
```json
{
  "channel_id": 3
}
```

---

#### `typing_stop`
Indique que l'utilisateur arrête de taper.

**Payload:**
```json
{
  "channel_id": 3
}
```

---

### Serveur → Client

#### `message:new`
Un nouveau message a été créé dans un canal.

**Payload:**
```json
{
  "channel_id": 3,
  "message_id": 42,
  "content": "Hello world!",
  "author_id": 1,
  "author_username": "john_doe",
  "created_at": "2026-01-28T10:30:00Z"
}
```

**Émis vers:** Tous les membres du serveur contenant ce canal.

---

#### `message:deleted`
Un message a été supprimé.

**Payload:**
```json
{
  "channel_id": 3,
  "message_id": 42
}
```

**Émis vers:** Tous les membres du serveur contenant ce canal.

---

#### `user:typing`
Un utilisateur est en train de taper.

**Payload:**
```json
{
  "channel_id": 3,
  "user_id": 2,
  "username": "alice"
}
```

**Émis vers:** Tous les membres connectés au canal (sauf l'émetteur).

**Note:** Cet événement devrait être suivi d'un `typing_stop` ou expirer après 3 secondes.

---

## 👥 Présence

### Serveur → Client

#### `user:connected`
Un utilisateur s'est connecté au serveur.

**Payload:**
```json
{
  "server_id": 1,
  "user_id": 2,
  "username": "alice"
}
```

**Émis vers:** Tous les membres du serveur.

---

#### `user:disconnected`
Un utilisateur s'est déconnecté du serveur.

**Payload:**
```json
{
  "server_id": 1,
  "user_id": 2,
  "username": "alice"
}
```

**Émis vers:** Tous les membres du serveur.

---

## 🏰 Serveurs

### Serveur → Client

#### `member:joined`
Un nouveau membre a rejoint le serveur.

**Payload:**
```json
{
  "server_id": 1,
  "user_id": 5,
  "username": "bob"
}
```

**Émis vers:** Tous les membres du serveur.

---

#### `member:left`
Un membre a quitté le serveur.

**Payload:**
```json
{
  "server_id": 1,
  "user_id": 5,
  "username": "bob"
}
```

**Émis vers:** Tous les membres du serveur.

---

## 📺 Canaux

### Serveur → Client

#### `channel:created`
Un nouveau canal a été créé.

**Payload:**
```json
{
  "server_id": 1,
  "channel_id": 8,
  "name": "general"
}
```

**Émis vers:** Tous les membres du serveur.

---

#### `channel:deleted`
Un canal a été supprimé.

**Payload:**
```json
{
  "server_id": 1,
  "channel_id": 8
}
```

**Émis vers:** Tous les membres du serveur.

---

## 🔐 Gestion des erreurs

Tous les événements peuvent retourner une erreur :

```json
{
  "error": "Permission denied",
  "code": "FORBIDDEN"
}
```

### Codes d'erreur courants

- `UNAUTHORIZED` - Token invalide ou manquant
- `FORBIDDEN` - Permissions insuffisantes
- `NOT_FOUND` - Ressource introuvable
- `BAD_REQUEST` - Payload invalide
- `INTERNAL_ERROR` - Erreur serveur

---

## 🎯 Rooms Socket.IO

Pour optimiser les broadcasts, nous utilisons des rooms :

- `server:{server_id}` - Tous les membres d'un serveur
- `channel:{channel_id}` - Tous les membres actifs dans un canal
- `user:{user_id}` - Socket personnel d'un utilisateur

### Exemple de broadcast

```rust
// Envoyer à tous les membres d'un serveur
io.to(format!("server:{}", server_id))
  .emit("message:new", payload);

// Envoyer à tous sauf l'émetteur
socket.to(format!("channel:{}", channel_id))
  .emit("user:typing", payload);
```

---

## 📊 Diagramme de séquence - Envoi de message

```
Client A                Backend                 Database                Client B
   |                       |                        |                       |
   |  POST /messages       |                        |                       |
   |---------------------->|                        |                       |
   |                       |  INSERT message        |                       |
   |                       |----------------------->|                       |
   |                       |<-----------------------|                       |
   |  201 Created          |                        |                       |
   |<----------------------|                        |                       |
   |                       |  emit "message:new"    |                       |
   |                       |------------------------------------------------>|
   |                       |                        |                       |
```

---

## 🧪 Test des événements Socket.IO

### Avec un client JavaScript

```javascript
import { io } from 'socket.io-client';

const socket = io('http://localhost:3000');

// Authentification
socket.emit('authenticate', { token: 'YOUR_JWT_TOKEN' });

// Écouter les nouveaux messages
socket.on('message:new', (data) => {
  console.log('New message:', data);
});

// Rejoindre un serveur
socket.emit('join_server', { server_id: 1 });

// Indiquer que l'on tape
socket.emit('typing_start', { channel_id: 3 });
```

---

## 📝 Notes d'implémentation

1. **Heartbeat** : Le client devrait envoyer un ping toutes les 30 secondes pour maintenir la connexion
2. **Reconnexion** : Le client devrait rejoindre automatiquement les rooms après reconnexion
3. **Rate limiting** : Limiter les événements `typing` à 1 par seconde maximum
4. **Persistence** : Les messages sont persistés en BDD, pas les événements de présence
5. **Scalabilité** : Pour une architecture multi-serveurs, utiliser Redis comme adaptateur Socket.IO

---

## 🔄 Évolution future

### Fonctionnalités bonus possibles

- `message:edit` - Modification de message
- `user:status` - Changement de statut (away, busy, etc.)
- `voice:join` / `voice:leave` - Canaux vocaux
- `reaction:add` - Réactions aux messages
- `mention` - Notifications de mention

---

**Version:** 1.0  
**Date:** Janvier 2026  
**Auteur:** Équipe Chat RTC
