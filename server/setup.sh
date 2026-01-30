#!/bin/bash

# Script de setup du projet Chat RTC Backend

echo "🚀 Setup du backend Chat RTC"
echo ""

# Couleurs
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Vérifier si Rust est installé
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Cargo n'est pas installé${NC}"
    echo ""
    echo "Pour installer Rust et Cargo, exécutez :"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    echo "Puis relancez ce script."
    exit 1
fi

echo -e "${GREEN}✓ Cargo est installé${NC}"

# Vérifier si PostgreSQL est accessible
if ! command -v psql &> /dev/null; then
    echo -e "${YELLOW}⚠ PostgreSQL CLI n'est pas installé (optionnel)${NC}"
else
    echo -e "${GREEN}✓ PostgreSQL CLI est installé${NC}"
fi

# Vérifier si le fichier .env existe
if [ ! -f .env ]; then
    echo -e "${YELLOW}⚠ Fichier .env manquant${NC}"
    echo "Copie de .env.example vers .env..."
    cp .env.example .env
    echo -e "${GREEN}✓ Fichier .env créé${NC}"
    echo ""
    echo -e "${YELLOW}⚠ Pensez à modifier les variables dans .env !${NC}"
else
    echo -e "${GREEN}✓ Fichier .env existe${NC}"
fi

# Installer SQLx CLI si pas déjà installé
if ! command -v sqlx &> /dev/null; then
    echo ""
    echo "Installation de SQLx CLI pour les migrations..."
    cargo install sqlx-cli --no-default-features --features postgres
    echo -e "${GREEN}✓ SQLx CLI installé${NC}"
else
    echo -e "${GREEN}✓ SQLx CLI est installé${NC}"
fi

# Vérifier les dépendances
echo ""
echo "Vérification des dépendances Cargo..."
cargo check
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Toutes les dépendances sont OK${NC}"
else
    echo -e "${RED}❌ Erreur lors de la vérification des dépendances${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}✅ Setup terminé avec succès !${NC}"
echo ""
echo "Prochaines étapes :"
echo "1. Assurez-vous que PostgreSQL est lancé"
echo "2. Créez la base de données : sqlx database create"
echo "3. Exécutez les migrations : sqlx migrate run"
echo "4. Lancez le serveur : cargo run"
echo ""
echo "Ou utilisez Docker : docker-compose up"
