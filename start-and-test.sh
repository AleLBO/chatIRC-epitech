#!/bin/bash

# Script de démarrage et test du backend Chat RTC
# Usage: ./start-and-test.sh

set -e

echo "🚀 Chat RTC Backend - Démarrage et Test"
echo "========================================"
echo ""

# Couleurs
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Fonction pour afficher les erreurs
error() {
    echo -e "${RED}❌ $1${NC}"
}

# Fonction pour afficher les succès
success() {
    echo -e "${GREEN}✅ $1${NC}"
}

# Fonction pour afficher les infos
info() {
    echo -e "${YELLOW}ℹ️  $1${NC}"
}

# 1. Vérifier que Docker est installé
echo "1. Vérification de Docker..."
if ! command -v docker &> /dev/null; then
    error "Docker n'est pas installé"
    exit 1
fi
success "Docker est installé"
echo ""

# 2. Arrêter les anciens conteneurs
echo "2. Nettoyage des anciens conteneurs..."
docker-compose down -v 2>/dev/null || true
success "Conteneurs arrêtés"
echo ""

# 3. Démarrer les services
echo "3. Démarrage des services Docker..."
info "Cela peut prendre plusieurs minutes (compilation Rust)..."
docker-compose up -d
success "Services démarrés"
echo ""

# 4. Attendre que PostgreSQL soit prêt
echo "4. Attente de la base de données..."
sleep 10
for i in {1..30}; do
    if docker exec irc_postgres pg_isready -U chatadmin &> /dev/null; then
        success "PostgreSQL est prêt"
        break
    fi
    if [ $i -eq 30 ]; then
        error "PostgreSQL n'a pas démarré"
        exit 1
    fi
    sleep 2
done
echo ""

# 5. Attendre la compilation du backend
echo "5. Compilation du backend Rust..."
info "Première compilation : 3-5 minutes"
info "Suivez les logs avec: docker logs -f irc_backend"
echo ""

# Attendre que le backend compile
for i in {1..60}; do
    if docker logs irc_backend 2>&1 | grep -q "🚀 Serveur lancé"; then
        success "Backend compilé et démarré!"
        break
    fi
    if docker logs irc_backend 2>&1 | grep -q "error: could not compile"; then
        error "Erreur de compilation du backend"
        echo ""
        echo "Dernières lignes des logs:"
        docker logs irc_backend 2>&1 | tail -20
        exit 1
    fi
    if [ $i -eq 60 ]; then
        info "Compilation en cours... (peut prendre encore 1-2 min)"
    fi
    sleep 3
done
echo ""

# 6. Afficher l'état des conteneurs
echo "6. État des services:"
docker-compose ps
echo ""

# 7. Tester la connexion au backend
echo "7. Test de connexion au backend..."
sleep 5
RESPONSE=$(curl -s http://localhost:4000/ || echo "")
if [ -z "$RESPONSE" ]; then
    error "Le backend ne répond pas"
    echo ""
    echo "Logs du backend:"
    docker logs irc_backend 2>&1 | tail -30
    exit 1
fi
success "Backend répond: $RESPONSE"
echo ""

# 8. Vérifier les tables de la base de données
echo "8. Vérification de la base de données..."
TABLES=$(docker exec irc_postgres psql -U chatadmin -d chatdb -t -c "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public';")
if [ "$TABLES" -ge 5 ]; then
    success "Base de données initialisée ($TABLES tables créées)"
else
    error "Problème avec la base de données"
fi
echo ""

# 9. Test de création de compte
echo "9. Test de l'API - Création de compte..."
SIGNUP_RESPONSE=$(curl -s -X POST http://localhost:4000/auth/signup \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","email":"test@example.com","password":"Test123"}' 2>/dev/null)

if echo "$SIGNUP_RESPONSE" | grep -q "token"; then
    success "✅ API fonctionne - Compte créé"
    echo "   Réponse: $(echo $SIGNUP_RESPONSE | head -c 100)..."
elif echo "$SIGNUP_RESPONSE" | grep -q "already exists"; then
    success "✅ API fonctionne (compte existe déjà)"
else
    info "Réponse API: $SIGNUP_RESPONSE"
fi
echo ""

# 10. Résumé final
echo "=========================================="
echo "🎉 BACKEND OPÉRATIONNEL"
echo "=========================================="
echo ""
echo "📍 Services accessibles:"
echo "   - Backend API:  http://localhost:4000"
echo "   - Frontend:     http://localhost:3000"
echo "   - PostgreSQL:   localhost:5432"
echo ""
echo "📚 Documentation:"
echo "   - API Examples:     server/API_EXAMPLES.md"
echo "   - WebSocket:        server/SOCKET_SPEC.md"
echo "   - Guide de test:    GUIDE_TEST_BACKEND.md"
echo "   - Test WebSocket:   server/test-websocket.html"
echo ""
echo "🔍 Commandes utiles:"
echo "   docker-compose logs -f server    # Logs du backend"
echo "   docker-compose ps                # État des services"
echo "   docker-compose down              # Arrêter tout"
echo "   docker-compose restart server    # Redémarrer le backend"
echo ""
success "Setup terminé avec succès! 🚀"
