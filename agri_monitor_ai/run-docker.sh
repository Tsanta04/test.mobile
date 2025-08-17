#!/bin/bash

# Script pour lancer le projet avec Docker

# Vérifier si Docker est installé
if ! command -v docker &> /dev/null; then
    echo "Docker n'est pas installé. Veuillez l'installer avant de continuer."
    exit 1
fi

# Vérifier si Docker Compose est installé
if ! command -v docker-compose &> /dev/null; then
    echo "Docker Compose n'est pas installé. Veuillez l'installer avant de continuer."
    exit 1
fi

# Créer le répertoire des modèles s'il n'existe pas
mkdir -p models

# Construire et lancer les conteneurs
echo "Construction et lancement des conteneurs..."
docker-compose up -d --build

echo "Le projet est maintenant accessible à l'adresse http://localhost:8080"
echo "Pour arrêter les conteneurs, exécutez 'docker-compose down'"

