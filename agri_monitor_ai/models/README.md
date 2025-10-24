# Répertoire des modèles

Ce répertoire contient les modèles de machine learning utilisés par le système de surveillance agricole.

## Structure

Les modèles sont organisés par module de prédiction, avec un modèle pour les données de capteurs et un modèle pour les images pour chaque module :

- `disease_sensor_model.bin` - Modèle pour la détection de maladies à partir des données de capteurs
- `disease_image_model.bin` - Modèle pour la détection de maladies à partir des images
- `water_sensor_model.bin` - Modèle pour la prédiction du taux d'eau à partir des données de capteurs
- `water_image_model.bin` - Modèle pour la prédiction du taux d'eau à partir des images
- `fertility_sensor_model.bin` - Modèle pour l'analyse de la fertilité à partir des données de capteurs
- `fertility_image_model.bin` - Modèle pour l'analyse de la fertilité à partir des images
- `air_quality_sensor_model.bin` - Modèle pour la surveillance de la qualité de l'air à partir des données de capteurs
- `air_quality_image_model.bin` - Modèle pour la surveillance de la qualité de l'air à partir des images
- `profitability_sensor_model.bin` - Modèle pour l'analyse de rentabilité à partir des données de capteurs
- `profitability_image_model.bin` - Modèle pour l'analyse de rentabilité à partir des images
- `production_date_sensor_model.bin` - Modèle pour la prédiction de la date de production à partir des données de capteurs
- `production_date_image_model.bin` - Modèle pour la prédiction de la date de production à partir des images
- `production_rate_sensor_model.bin` - Modèle pour la prédiction du taux de production à partir des données de capteurs
- `production_rate_image_model.bin` - Modèle pour la prédiction du taux de production à partir des images
- `disaster_sensor_model.bin` - Modèle pour la détection de catastrophes naturelles à partir des données de capteurs
- `disaster_image_model.bin` - Modèle pour la détection de catastrophes naturelles à partir des images

## Entraînement des modèles

Les modèles peuvent être entraînés à l'aide des scripts fournis dans le répertoire `scripts/training`. Consultez la documentation de chaque module pour plus d'informations sur l'entraînement des modèles.

## Utilisation des modèles

Les modèles sont chargés automatiquement par le système lors du démarrage. Les chemins des modèles sont configurés dans le fichier `.env`.

