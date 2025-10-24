#!/bin/bash

# Script d'initialisation pour le projet de surveillance agricole

# Créer les répertoires nécessaires
mkdir -p models

# Vérifier si .env existe, sinon le créer à partir de .env.example
if [ ! -f .env ]; then
    echo "Création du fichier .env à partir de .env.example..."
    cp .env.example .env
    echo "Fichier .env créé. Veuillez le modifier selon vos besoins."
fi

# Vérifier si Rust est installé
if ! command -v rustc &> /dev/null; then
    echo "Rust n'est pas installé. Installation de Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    echo "Rust installé avec succès."
fi

# Vérifier la version de Rust
RUST_VERSION=$(rustc --version | cut -d ' ' -f 2)
echo "Version de Rust: $RUST_VERSION"

# Installer les outils de développement Rust
echo "Installation des outils de développement Rust..."
rustup component add rustfmt clippy

# Compiler le projet
echo "Compilation du projet..."
cargo build

echo "Initialisation terminée. Vous pouvez maintenant exécuter le projet avec ./run.sh"

