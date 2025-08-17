#!/bin/bash

# Script pour exécuter les tests du projet

# Exécuter les tests unitaires
echo "Exécution des tests unitaires..."
cargo test

# Exécuter les vérifications de style
echo "Exécution des vérifications de style..."
cargo fmt -- --check

# Exécuter les vérifications de linting
echo "Exécution des vérifications de linting..."
cargo clippy -- -D warnings

# Si toutes les vérifications sont passées
if [ $? -eq 0 ]; then
    echo "Tous les tests ont réussi !"
    exit 0
else
    echo "Des erreurs ont été détectées. Veuillez les corriger avant de continuer."
    exit 1
fi

