#!/bin/bash

echo "=== État des conteneurs Docker ==="
docker ps -a | grep irc

echo ""
echo "=== Logs du backend (dernières 30 lignes) ==="
docker logs irc_backend 2>&1 | tail -30

echo ""
echo "=== Test connexion au backend ==="
curl -s http://localhost:4000/ || echo "❌ Backend ne répond pas"

echo ""
echo "=== Test de la base de données ==="
docker exec irc_postgres psql -U chatadmin -d chatdb -c "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public';" 2>&1 | head -20
