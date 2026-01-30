#!/bin/bash
# Script pour vérifier la compilation du backend après corrections

cd /Users/shakzk/Desktop/chatIRC-epitech

echo "======================================"
echo "🔄 REDÉMARRAGE DU BACKEND"
echo "======================================"
docker-compose restart server
echo "✅ Backend redémarré, attente de 60 secondes pour compilation..."
echo ""

sleep 60

echo "======================================"
echo "📋 LOGS DE COMPILATION (50 dernières lignes)"
echo "======================================"
docker logs irc_backend --tail 50 2>&1
echo ""

echo "======================================"
echo "🔍 RECHERCHE D'ERREURS"
echo "======================================"
ERROR_COUNT=$(docker logs irc_backend 2>&1 | grep -c "error\[E")
echo "Nombre d'erreurs trouvées: $ERROR_COUNT"

if [ "$ERROR_COUNT" -eq 0 ]; then
    echo "✅ Aucune erreur de compilation !"
    echo ""
    echo "======================================"
    echo "🚀 TEST DE CONNEXION HTTP"
    echo "======================================"
    sleep 5
    curl -s http://localhost:4000/ && echo "" || echo "❌ Backend ne répond pas encore"
else
    echo "❌ Des erreurs persistent:"
    docker logs irc_backend 2>&1 | grep "error\[E" | tail -5
fi

echo ""
echo "======================================"
echo "📊 STATUT DES CONTENEURS"
echo "======================================"
docker ps -a | grep irc
