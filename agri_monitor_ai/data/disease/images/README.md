# Images pour la détection de maladies

Ce répertoire contient des images pour l'entraînement et l'évaluation des modèles de détection de maladies des plantes.

## Structure

Les images sont organisées en répertoires par catégorie :

- `healthy/` - Images de plantes saines
- `bacterial_blight/` - Images de plantes atteintes de bactériose
- `fungal_rust/` - Images de plantes atteintes de rouille (maladie fongique)
- `viral_mosaic/` - Images de plantes atteintes de mosaïque (maladie virale)

## Format des images

Les images sont au format JPEG ou PNG, avec une résolution recommandée de 224x224 pixels pour une utilisation optimale avec les modèles de deep learning.

## Sources des images

Les images peuvent être obtenues à partir de diverses sources, notamment :

- PlantVillage - 87K images de 38 classes de maladies sur cultures
- CCMT Dataset - Images de ravageurs et maladies
- New Plant Diseases Dataset - 87K images RGB
- PlantDoc - 2.5K images pour 30 classes

## Utilisation

Pour utiliser ces images pour l'entraînement des modèles, exécutez le script `scripts/training/train_disease_models.sh`.

