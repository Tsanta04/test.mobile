#!/bin/bash

# Script pour exécuter l'API de surveillance agricole

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

# Construire et démarrer les conteneurs
echo "Démarrage des services..."
docker-compose up -d

# Attendre que les services soient prêts
echo "Attente du démarrage des services..."
sleep 5

# Afficher les logs
echo "Logs des services:"
docker-compose logs

echo "L'API est accessible à l'adresse http://localhost:8080"
echo "Pour arrêter les services, exécutez: docker-compose down"

