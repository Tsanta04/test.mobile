# Utilisation d'une image multi-stage pour optimiser la taille de l'image finale

# Étape de build
FROM rust:1.75 as builder

# Installer les dépendances nécessaires
RUN apt-get update && \
    apt-get install -y \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Créer un répertoire pour l'application
WORKDIR /app

# Copier les fichiers Cargo.toml et Cargo.lock
COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml ./api/
COPY crates/common/Cargo.toml ./crates/common/
COPY crates/disease_detection/Cargo.toml ./crates/disease_detection/

# Créer des fichiers vides pour tous les fichiers source
# Cela permet de compiler les dépendances sans le code source
RUN mkdir -p api/src && \
    mkdir -p crates/common/src && \
    mkdir -p crates/disease_detection/src && \
    touch api/src/lib.rs && \
    touch crates/common/src/lib.rs && \
    touch crates/disease_detection/src/lib.rs

# Compiler les dépendances
RUN cargo build --release

# Supprimer les fichiers source vides
RUN rm -rf api/src && \
    rm -rf crates/common/src && \
    rm -rf crates/disease_detection/src

# Copier le code source réel
COPY api/src ./api/src
COPY crates/common/src ./crates/common/src
COPY crates/disease_detection/src ./crates/disease_detection/src

# Compiler l'application
RUN cargo build --release

# Étape finale
FROM debian:bookworm-slim

# Installer les dépendances d'exécution
RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Créer un répertoire pour l'application
WORKDIR /app

# Copier l'exécutable compilé depuis l'étape de build
COPY --from=builder /app/target/release/api /app/api

# Créer un répertoire pour les modèles
RUN mkdir -p /app/models

# Exposer le port
EXPOSE 8080

# Définir la commande d'exécution
CMD ["/app/api"]

