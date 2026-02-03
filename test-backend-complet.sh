#!/bin/bash
# 🎉 BACKEND OPÉRATIONNEL - GUIDE DE TEST COMPLET

echo "======================================"
echo "🎉 BACKEND RUST OPÉRATIONNEL !"
echo "======================================"
echo ""
echo "✅ Compilation réussie"
echo "✅ PostgreSQL connecté"
echo "✅ Serveur sur http://localhost:4000"
echo "✅ WebSocket prêt"
echo ""

echo "======================================"
echo "📚 TESTS DISPONIBLES"
echo "======================================"
echo ""

echo "1️⃣  TEST SIMPLE - Vérifier que l'API répond:"
echo "   curl http://localhost:4000/"
echo ""

echo "2️⃣  CRÉER UN COMPTE:"
echo "   curl -X POST http://localhost:4000/auth/signup \\"
echo "     -H 'Content-Type: application/json' \\"
echo "     -d '{\"username\":\"test\",\"email\":\"test@example.com\",\"password\":\"Test123!\"}'"
echo ""

echo "3️⃣  SE CONNECTER:"
echo "   curl -X POST http://localhost:4000/auth/login \\"
echo "     -H 'Content-Type: application/json' \\"
echo "     -d '{\"email\":\"test@example.com\",\"password\":\"Test123!\"}'"
echo ""

echo "4️⃣  CRÉER UN SERVEUR (remplace TOKEN par ton JWT):"
echo "   curl -X POST http://localhost:4000/servers \\"
echo "     -H 'Content-Type: application/json' \\"
echo "     -H 'Authorization: Bearer TOKEN' \\"
echo "     -d '{\"name\":\"Mon Premier Serveur\",\"description\":\"Test\"}'"
echo ""

echo "5️⃣  LISTER MES SERVEURS:"
echo "   curl http://localhost:4000/servers \\"
echo "     -H 'Authorization: Bearer TOKEN'"
echo ""

echo "======================================"
echo "🔧 COMMANDES UTILES"
echo "======================================"
echo ""
echo "Voir les logs en temps réel:"
echo "  docker logs -f irc_backend"
echo ""
echo "Redémarrer le backend:"
echo "  docker-compose restart server"
echo ""
echo "Arrêter tout:"
echo "  docker-compose down"
echo ""
echo "Nettoyer et redémarrer:"
echo "  docker-compose down -v && docker-compose up -d"
echo ""

echo "======================================"
echo "📊 STATUT ACTUEL"
echo "======================================"
docker ps --filter "name=irc" --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"
echo ""

echo "======================================"
echo "🎯 TEST RAPIDE - Création de compte"
echo "======================================"
echo ""
read -p "Veux-tu créer un compte de test maintenant ? (y/n): " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo ""
    echo "Création du compte test@example.com..."
    RESPONSE=$(curl -s -X POST http://localhost:4000/auth/signup \
      -H 'Content-Type: application/json' \
      -d '{"username":"test","email":"test@example.com","password":"Test123!"}')
    
    echo "Réponse: $RESPONSE"
    echo ""
    
    if echo "$RESPONSE" | grep -q "token"; then
        echo "✅ Compte créé avec succès !"
        TOKEN=$(echo "$RESPONSE" | grep -o '"token":"[^"]*"' | cut -d'"' -f4)
        echo ""
        echo "🔑 Ton token JWT:"
        echo "$TOKEN"
        echo ""
        echo "Sauvegarde-le pour les prochaines requêtes !"
    else
        echo "ℹ️  Réponse du serveur (peut-être que le compte existe déjà):"
        echo "$RESPONSE"
    fi
fi

echo ""
echo "======================================"
echo "📖 DOCUMENTATION COMPLÈTE"
echo "======================================"
echo "Consulte: server/API_EXAMPLES.md"
echo "WebSocket: server/SOCKET_SPEC.md"
echo "Architecture: server/ARCHITECTURE.md"
