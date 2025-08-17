# Répertoire des données

Ce répertoire contient les données utilisées pour entraîner les modèles de machine learning du système de surveillance agricole.

## Structure

Les données sont organisées par module de prédiction :

- `disease/` - Données pour la détection de maladies
  - `sensor_data.csv` - Données de capteurs pour la détection de maladies
  - `images/` - Images pour la détection de maladies
- `water/` - Données pour la prédiction du taux d'eau
- `fertility/` - Données pour l'analyse de la fertilité
- `air_quality/` - Données pour la surveillance de la qualité de l'air
- `profitability/` - Données pour l'analyse de rentabilité
- `production_date/` - Données pour la prédiction de la date de production
- `production_rate/` - Données pour la prédiction du taux de production
- `disaster/` - Données pour la détection de catastrophes naturelles

## Format des données

### Données de capteurs

Les données de capteurs sont stockées au format CSV avec les colonnes suivantes :

- `timestamp` - Horodatage de la mesure (format ISO 8601)
- `latitude` - Latitude de la mesure
- `longitude` - Longitude de la mesure
- `soil_moisture` - Humidité du sol (%)
- `air_humidity` - Humidité de l'air (%)
- `temperature` - Température (°C)
- `soil_ph` - pH du sol
- `nitrogen` - Teneur en azote (ppm)
- `phosphorus` - Teneur en phosphore (ppm)
- `potassium` - Teneur en potassium (ppm)
- `label` - Étiquette pour l'entraînement (e.g., type de maladie)

### Images

Les images sont stockées au format JPEG ou PNG, organisées en répertoires par catégorie (e.g., type de maladie).

## Sources de données

Les données peuvent être obtenues à partir de diverses sources, notamment :

- PlantVillage - Images de maladies des plantes
- LUCAS 2018 TOPSOIL - Propriétés du sol
- Soil Moisture Remote Sensing Data - Données d'humidité du sol
- Global Gridded Crop Production Dataset - Données de production agricole
- Air Quality Dataset - Données de qualité de l'air

Consultez la documentation de chaque module pour plus d'informations sur les sources de données spécifiques.

