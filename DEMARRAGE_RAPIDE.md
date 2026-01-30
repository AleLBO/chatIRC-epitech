# 🚀 Chat RTC - Guide de Démarrage Rapide

**Backend Rust 100% Opérationnel** | Frontend Next.js à développer

---

## ⚡ DÉMARRAGE EN 30 SECONDES

```bash
# 1. Lancer avec le script automatique
./start-and-test.sh

# OU manuellement :
docker-compose up -d
docker logs -f irc_backend  # Suivre la compilation (3-5 min)
```

---

## 📡 SERVICES DISPONIBLES

| Service | URL | État |
|---------|-----|------|
| 🦀 Backend API | http://localhost:4000 | ✅ Opérationnel |
| ⚡ Frontend | http://localhost:3000 | ⏳ À développer |
| 🐘 PostgreSQL | localhost:5432 | ✅ Opérationnel |

---

## 🧪 TESTS RAPIDES

### Test 1 : Backend répond
```bash
curl http://localhost:4000/
# Résultat: "🚀 Chat RTC Backend opérationnel !"
```

### Test 2 : Créer un compte
```bash
curl -X POST http://localhost:4000/auth/signup \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","email":"alice@test.com","password":"Test123"}'
```

### Test 3 : Se connecter
```bash
curl -X POST http://localhost:4000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"alice@test.com","password":"Test123"}'
# Copier le token JWT retourné
```

### Test 4 : WebSocket temps réel
Ouvrir dans un navigateur : `server/test-websocket.html`

---

## 📚 DOCUMENTATION COMPLÈTE

### Guides principaux
- **[CORRECTIONS_FINALES.md](./CORRECTIONS_FINALES.md)** - Toutes les corrections appliquées
- **[GUIDE_TEST_BACKEND.md](./GUIDE_TEST_BACKEND.md)** - Tests détaillés du backend
- **[FRONTEND_INTEGRATION.md](./FRONTEND_INTEGRATION.md)** - Guide de développement frontend

### Documentation backend
- **[server/API_EXAMPLES.md](./server/API_EXAMPLES.md)** - Exemples curl pour chaque endpoint
- **[server/SOCKET_SPEC.md](./server/SOCKET_SPEC.md)** - Spécification WebSocket
- **[server/ARCHITECTURE.md](./server/ARCHITECTURE.md)** - Architecture détaillée

### Vérification
- **[STATUS_FINAL.md](./STATUS_FINAL.md)** - Résumé exécutif
- **[CERTIFICATION_BACKEND.md](./CERTIFICATION_BACKEND.md)** - Certification technique

---

## 🏗️ ARCHITECTURE

### Backend (✅ Complet)
```
20 Endpoints REST + 5 Événements WebSocket
↓
Clean Architecture (6 layers)
↓
PostgreSQL (5 tables)
```

**Stack** : Rust + Axum + Socket.IO + PostgreSQL + JWT + Bcrypt

### Frontend (⏳ À développer)
**Stack recommandé** : Next.js + TypeScript + Socket.IO Client + TailwindCSS

---

## 🔧 COMMANDES UTILES

```bash
# Logs en temps réel
docker-compose logs -f server

# Redémarrer le backend
docker-compose restart server

# État des services
docker-compose ps

# Arrêter tout
docker-compose down

# Tout nettoyer et redémarrer
docker-compose down -v && docker-compose up -d
```

---

## 🎯 ENDPOINTS API PRINCIPAUX

### Authentification
- `POST /auth/signup` - Créer un compte
- `POST /auth/login` - Se connecter
- `GET /auth/me` - Info utilisateur

### Serveurs
- `POST /servers` - Créer un serveur
- `GET /servers` - Lister mes serveurs
- `POST /servers/join` - Rejoindre avec code

### Canaux
- `POST /servers/:id/channels` - Créer un canal
- `GET /servers/:id/channels` - Lister les canaux

### Messages
- `POST /channels/:id/messages` - Envoyer un message
- `GET /channels/:id/messages` - Historique

**Total** : 20 endpoints | Voir `server/API_EXAMPLES.md` pour la liste complète

---

## 🔌 ÉVÉNEMENTS WEBSOCKET

```javascript
// Connexion
socket.emit('authenticate', { token: 'JWT_TOKEN' })

// Rejoindre un serveur
socket.emit('join_server', { server_id: 1 })

// Événements reçus
socket.on('new_message', (data) => { /* ... */ })
socket.on('user_connected', (data) => { /* ... */ })
socket.on('user_typing', (data) => { /* ... */ })
```

**Total** : 5 événements | Voir `server/SOCKET_SPEC.md` pour les détails

---

## 📊 STATISTIQUES

| Métrique | Valeur |
|----------|--------|
| **Lignes de code Rust** | 2,303 |
| **Fichiers source** | 31 |
| **Endpoints REST** | 20 |
| **Événements WebSocket** | 5 |
| **Tables PostgreSQL** | 5 |
| **Fichiers documentation** | 18 |
| **Erreurs de compilation** | 0 ✅ |

---

## ✅ CORRECTIONS APPLIQUÉES

1. **Cargo.toml** - Versions de dépendances compatibles
2. **AppState** - Architecture d'état Axum corrigée
3. **Handlers** - Tous mis à jour pour AppState
4. **Schema SQL** - Ajout de NOT NULL sur les colonnes
5. **WebSocket** - Hub centralisé pour gérer les sockets
6. **Models** - Exports complets

Détails complets : [CORRECTIONS_FINALES.md](./CORRECTIONS_FINALES.md)

---

## 🚀 PROCHAINES ÉTAPES

### Pour tester le backend (MAINTENANT)
1. Lancer : `./start-and-test.sh`
2. Tester l'API : Voir `GUIDE_TEST_BACKEND.md`
3. Tester WebSocket : Ouvrir `server/test-websocket.html`

### Pour développer le frontend (APRÈS)
1. Lire : `FRONTEND_INTEGRATION.md`
2. Installer les dépendances dans `client/`
3. Créer les pages d'auth et de chat
4. Intégrer Socket.IO client

---

## 💡 TIPS

### Première compilation lente ?
✅ **Normal !** Rust compile toutes les dépendances la première fois (3-5 min)
✅ Les compilations suivantes sont **instantanées** grâce au cache Docker

### Backend ne démarre pas ?
```bash
# Voir les logs détaillés
docker logs irc_backend 2>&1 | tail -50

# Redémarrer proprement
docker-compose down -v
docker-compose up --build
```

### Base de données
```bash
# Se connecter à PostgreSQL
docker exec -it irc_postgres psql -U chatadmin -d chatdb

# Lister les tables
\dt

# Quitter
\q
```

---

## 🎉 STATUT ACTUEL

```
╔═══════════════════════════════════════╗
║   ✅ BACKEND: PRÊT À L'EMPLOI         ║
║   ⏳ FRONTEND: À DÉVELOPPER           ║
╚═══════════════════════════════════════╝
```

**Le backend peut être utilisé immédiatement !**

Questions ? Voir la documentation complète dans les fichiers markdown.

---

*Dernière mise à jour : 30 janvier 2026*
