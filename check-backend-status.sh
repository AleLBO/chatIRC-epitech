#!/bin/bash

echo "==================================="
echo "DIAGNOSTIC BACKEND - $(date)"
echo "==================================="
echo ""

echo "1. État des conteneurs Docker:"
docker ps -a | grep irc
echo ""

echo "2. Statut du backend:"
docker inspect irc_backend --format='{{.State.Status}}' 2>/dev/null || echo "Conteneur introuvable"
echo ""

echo "3. Dernières 30 lignes des logs:"
docker logs irc_backend 2>&1 | tail -30
echo ""

echo "4. Recherche d'erreurs:"
docker logs irc_backend 2>&1 | grep -i "error" | tail -10
echo ""

echo "5. Test connexion HTTP:"
curl -s -m 3 http://localhost:4000/ || echo "❌ Backend ne répond pas"
echo ""

echo "==================================="
echo "Fin du diagnostic"
echo "==================================="
