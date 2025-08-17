#!/bin/bash

# Script d'initialisation pour le projet de surveillance agricole

# Vérifier si Rust est installé
if ! command -v rustc &> /dev/null; then
    echo "Rust n'est pas installé. Installation en cours..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rust est déjà installé."
fi

# Créer le répertoire des modèles
mkdir -p models

# Copier le fichier .env.example vers .env s'il n'existe pas
if [ ! -f .env ]; then
    cp .env.example .env
    echo "Fichier .env créé à partir de .env.example."
else
    echo "Le fichier .env existe déjà."
fi

# Compiler le projet
echo "Compilation du projet..."
cargo build

echo "Initialisation terminée. Vous pouvez maintenant lancer l'API avec 'cargo run -p api'."

