#!/bin/bash

# Script pour entraîner les modèles de détection de maladies

# Vérifier si le répertoire des modèles existe
if [ ! -d "../../models" ]; then
    echo "Création du répertoire des modèles..."
    mkdir -p ../../models
fi

# Vérifier si les données d'entraînement existent
if [ ! -d "../../data/disease" ]; then
    echo "Erreur: Le répertoire des données d'entraînement n'existe pas."
    echo "Veuillez créer le répertoire data/disease et y placer les données d'entraînement."
    exit 1
fi

# Entraîner le modèle de détection de maladies à partir des données de capteurs
echo "Entraînement du modèle de détection de maladies à partir des données de capteurs..."
cargo run --bin train_disease_sensor_model -- \
    --data ../../data/disease/sensor_data.csv \
    --output ../../models/disease_sensor_model.bin

# Entraîner le modèle de détection de maladies à partir des images
echo "Entraînement du modèle de détection de maladies à partir des images..."
cargo run --bin train_disease_image_model -- \
    --data ../../data/disease/images \
    --output ../../models/disease_image_model.bin

echo "Entraînement terminé. Les modèles ont été enregistrés dans le répertoire models."

