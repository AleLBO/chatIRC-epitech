# Utilisation de l'image officielle Rust
FROM rust:latest

WORKDIR /app

# Installation de cargo-watch pour le hot-reloading en développement
RUN cargo install cargo-watch

# Copie des fichiers
COPY . .

# Commande par défaut : surveille les changements et relance le serveur
CMD ["cargo", "watch", "-x", "run"]