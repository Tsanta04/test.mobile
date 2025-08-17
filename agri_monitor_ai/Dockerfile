# Utiliser l'image officielle Rust comme base
FROM rust:1.76-slim-bullseye as builder

# Installer les dépendances nécessaires
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev libpq-dev && \
    rm -rf /var/lib/apt/lists/*

# Créer un répertoire pour l'application
WORKDIR /app

# Copier les fichiers Cargo.toml et Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Copier les crates
COPY crates ./crates
COPY api ./api

# Construire l'application en mode release
RUN cargo build --release -p api

# Créer une image plus légère pour l'exécution
FROM debian:bullseye-slim

# Installer les dépendances d'exécution
RUN apt-get update && \
    apt-get install -y libssl-dev libpq-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Créer un répertoire pour l'application
WORKDIR /app

# Copier l'exécutable depuis l'étape de construction
COPY --from=builder /app/target/release/api ./

# Créer le répertoire des modèles
RUN mkdir -p models

# Copier le fichier .env.example
COPY .env.example .env

# Exposer le port
EXPOSE 8080

# Définir la commande d'exécution
CMD ["./api"]

