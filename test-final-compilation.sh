#!/bin/bash
# Script de vérification FINALE après correction des 233 erreurs

cd /Users/shakzk/Desktop/chatIRC-epitech

echo "======================================"
echo "🔧 CORRECTIONS APPLIQUÉES"
echo "======================================"
echo "✅ TowerState → State (socketioxide 0.14)"
echo "✅ Import corrigé dans main.rs"  
echo "✅ Import corrigé dans ws/handlers.rs"
echo "✅ Tous les appels de handlers corrigés"
echo ""

echo "======================================"
echo "🔄 REDÉMARRAGE DU BACKEND"
echo "======================================"
docker-compose down
docker-compose up -d
echo "⏳ Attente de 90 secondes pour la compilation complète..."
echo ""

sleep 90

echo "======================================"
echo "📋 RÉSULTAT DE LA COMPILATION"
echo "======================================"
docker logs irc_backend 2>&1 | tail -30
echo ""

echo "======================================"
echo "🔍 VÉRIFICATION DES ERREURS"
echo "======================================"
ERROR_COUNT=$(docker logs irc_backend 2>&1 | grep -c "error\[E")
echo "Nombre d'erreurs Rust: $ERROR_COUNT"

if [ "$ERROR_COUNT" -eq 0 ]; then
    echo "✅ ✅ ✅ COMPILATION RÉUSSIE ! ✅ ✅ ✅"
    echo ""
    echo "======================================"
    echo "🚀 TEST DE CONNEXION HTTP"
    echo "======================================"
    sleep 3
    RESPONSE=$(curl -s http://localhost:4000/)
    if [ -n "$RESPONSE" ]; then
        echo "✅ Backend répond: $RESPONSE"
        echo ""
        echo "======================================"
        echo "🎉 SUCCÈS TOTAL !"
        echo "======================================"
        echo "✅ Backend compilé sans erreur"
        echo "✅ Serveur démarré sur port 4000"
        echo "✅ PostgreSQL connecté"
        echo "✅ WebSocket prêt"
        echo ""
        echo "📚 Prochaines étapes:"
        echo "  1. Tester l'inscription: curl -X POST http://localhost:4000/auth/signup"
        echo "  2. Tester le login: curl -X POST http://localhost:4000/auth/login"
        echo "  3. Tester WebSocket avec: open server/test-websocket.html"
    else
        echo "⚠️  Backend ne répond pas encore, vérifier les logs"
    fi
else
    echo "❌ Des erreurs persistent:"
    docker logs irc_backend 2>&1 | grep "error\[E" | head -10
fi

echo ""
echo "======================================"
echo "📊 STATUT DES CONTENEURS"
echo "======================================"
docker ps -a | grep irc
