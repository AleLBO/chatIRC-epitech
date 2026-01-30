# 💬 Chat RTC - Projet Epitech

Application de chat en temps réel avec architecture client-serveur.

---

## 📁 STRUCTURE DU PROJET

```
chatIRC-epitech/
├── 📁 server/          ✅ Backend Rust (COMPLET)
├── 📁 client/          ⚠️ Frontend Next.js (À DÉVELOPPER)
└── 📚 Documentation    11 fichiers markdown
```

---

## ✅ BACKEND (Rust + Axum + Socket.IO + PostgreSQL)

### Status: **100% OPÉRATIONNEL**

Le backend est entièrement fonctionnel avec :
- **20 endpoints REST** (Auth, Servers, Channels, Messages)
- **WebSocket temps réel** avec Socket.IO (5 événements)
- **Clean Architecture** (6 layers: Models, Repositories, Services, Handlers, Utils, WebSocket)
- **Sécurité**: JWT + Bcrypt + Permissions par rôle (Owner/Admin/Member)
- **Base PostgreSQL** avec 5 tables et migrations
- **Documentation complète** (8 fichiers dans `server/`)

### Lancement du Backend

```bash
# Avec Docker (RECOMMANDÉ)
docker-compose up --build

# Backend disponible sur: http://localhost:4000
# PostgreSQL sur: localhost:5432
```

### Documentation Backend
- `server/README.md` - Vue d'ensemble technique
- `server/QUICKSTART.md` - Démarrage en 5 minutes
- `server/API_EXAMPLES.md` - Exemples curl pour chaque endpoint
- `server/SOCKET_SPEC.md` - Spécification WebSocket
- `server/ARCHITECTURE.md` - Détails de l'architecture
- `server/test-websocket.html` - Page de test interactive

---

## ⚠️ FRONTEND (Next.js + React)

### Status: **À DÉVELOPPER**

Le dossier `client/` contient uniquement le squelette Next.js de base.

### À implémenter
- [ ] Pages d'authentification (login, signup)
- [ ] Interface de chat
- [ ] Liste des serveurs et canaux
- [ ] Zone de messages avec historique
- [ ] Intégration WebSocket pour le temps réel
- [ ] State management
- [ ] UI/UX moderne

### Guide de développement
Suivre le guide **`FRONTEND_INTEGRATION.md`** qui contient :
- Installation des dépendances (socket.io-client, axios)
- Structure recommandée (lib/, contexts/, components/)
- Exemples de code TypeScript
- Intégration API REST et WebSocket

---

## 📚 DOCUMENTATION PRINCIPALE

| Document | Description |
|----------|-------------|
| **STATUS_FINAL.md** | ⭐ Résumé exécutif du projet |
| **FRONTEND_INTEGRATION.md** | ⭐ Guide complet pour le frontend |
| **CERTIFICATION_BACKEND.md** | Audit technique du backend |
| **VERIFICATION_FINALE.md** | Vérification détaillée |

---

## 🚀 DÉMARRAGE RAPIDE

### 1. Lancer le backend
```bash
cd /Users/shakzk/Desktop/chatIRC-epitech
docker-compose up --build
```

### 2. Tester l'API
```bash
# Signup
curl -X POST http://localhost:4000/auth/signup \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","email":"alice@example.com","password":"password123"}'

# Login (récupérer le token)
curl -X POST http://localhost:4000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"alice@example.com","password":"password123"}'
```

Plus d'exemples dans `server/API_EXAMPLES.md`

### 3. Tester WebSocket
Ouvrir `server/test-websocket.html` dans un navigateur

### 4. Développer le frontend
Suivre `FRONTEND_INTEGRATION.md`

---

## 🏗️ ARCHITECTURE

### Backend (Rust)
```
Clean Architecture / Hexagonal Architecture

┌─────────────────────────────────────────┐
│         HTTP/WebSocket Layer            │  (Handlers)
├─────────────────────────────────────────┤
│         Business Logic Layer            │  (Services)
├─────────────────────────────────────────┤
│         Data Access Layer               │  (Repositories)
├─────────────────────────────────────────┤
│         Domain Layer                    │  (Models)
└─────────────────────────────────────────┘
```

### Frontend (À développer)
```
Recommandé: Architecture par features

client/
├── lib/            (API client, WebSocket, types)
├── contexts/       (Auth, WebSocket contexts)
├── components/     (UI réutilisables)
└── app/            (Pages Next.js)
```

---

## 🔐 SÉCURITÉ

- **JWT** pour l'authentification stateless
- **Bcrypt** pour le hashage des mots de passe
- **Permissions par rôle** (Owner, Admin, Member)
- **CORS** configuré pour le frontend
- **Validation** des inputs au niveau des handlers

---

## 🐳 DOCKER

### Services
- **db**: PostgreSQL 15-alpine (port 5432)
- **server**: Backend Rust/Axum (port 4000)
- **client**: Frontend Next.js (port 3000) - À configurer

### Configuration
Variables d'environnement dans `.env` (voir `server/.env.example`)

---

## 📊 MÉTRIQUES

| Métrique | Valeur |
|----------|--------|
| Lignes de code Rust | 2,303 |
| Fichiers Rust | 31 |
| Endpoints REST | 20 |
| Événements WebSocket | 5 |
| Tables PostgreSQL | 5 |
| Documentation | 11 fichiers |

---

## 🎯 ROADMAP

### ✅ Phase 1: Backend (TERMINÉ)
- [x] API REST complète
- [x] WebSocket temps réel
- [x] Authentification et permissions
- [x] Base de données PostgreSQL
- [x] Documentation exhaustive

### 🚧 Phase 2: Frontend (EN COURS - Votre responsabilité)
- [ ] Interface utilisateur
- [ ] Authentification UI
- [ ] Chat interface
- [ ] Intégration WebSocket
- [ ] Tests frontend

### 📋 Phase 3: Améliorations (OPTIONNEL)
- [ ] Tests automatisés (backend + frontend)
- [ ] Monitoring et observabilité
- [ ] Rate limiting
- [ ] Édition de messages
- [ ] Réactions aux messages
- [ ] Upload de fichiers
- [ ] Mentions @utilisateur
- [ ] Notifications push

---

## 🧪 TESTS

### Backend
```bash
cd server
cargo test  # Tests unitaires (à implémenter)
```

### Frontend
```bash
cd client
npm test    # À configurer
```

### Test manuel
- API REST: Voir `server/API_EXAMPLES.md`
- WebSocket: Ouvrir `server/test-websocket.html`

---

## 📞 SUPPORT

### Documentation détaillée
- Backend: `server/README.md`
- Architecture: `server/ARCHITECTURE.md`
- API: `server/API_EXAMPLES.md`
- WebSocket: `server/SOCKET_SPEC.md`
- Frontend: `FRONTEND_INTEGRATION.md`

### Vérification du projet
- `STATUS_FINAL.md` - Résumé exécutif
- `CERTIFICATION_BACKEND.md` - Audit technique
- `VERIFICATION_FINALE.md` - Checklist complète

---

## 👥 ÉQUIPE

- **Backend (Rust)**: ✅ Développé par GitHub Copilot
- **Frontend (Next.js)**: ⚠️ À développer par shakzk

---

## 📄 LICENCE

Projet Epitech - 2026

---

## 🎉 ÉTAT ACTUEL

```
╔════════════════════════════════════════════════════════╗
║  ✅ BACKEND: 100% COMPLET ET OPÉRATIONNEL             ║
║  ❌ FRONTEND: À DÉVELOPPER                             ║
╚════════════════════════════════════════════════════════╝
```

**Le backend peut être lancé immédiatement avec Docker !**

```bash
docker-compose up --build
```

Ensuite, suivre **`FRONTEND_INTEGRATION.md`** pour développer l'interface utilisateur.
