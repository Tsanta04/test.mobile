# Module de prédiction de la fertilité du sol

Ce module fournit des fonctionnalités pour analyser et prédire la fertilité du sol dans un contexte agricole.

## Fonctionnalités

- **Analyse de la fertilité du sol** à partir de données de capteurs (pH, NPK, etc.)
- **Génération de cartes de fertilité** à partir d'images satellites/spectrales
- **Recommandations de fertilisation** adaptées aux cultures
- **Suivi de l'évolution de la fertilité** dans le temps

## Architecture

Le module est structuré de manière modulaire :

- `FertilityPredictor` : Gestionnaire principal qui coordonne les prédictions
- `SensorModel` : Modèle pour prédire la fertilité à partir de données de capteurs
- `ImageModel` : Modèle pour générer des cartes de fertilité à partir d'images
- Types de données spécifiques pour représenter les prédictions et recommandations
- Utilitaires pour les calculs spécialisés

## Utilisation

```rust
use fertility::{FertilityPredictor, types::{SensorPredictionRequest, ImagePredictionRequest}};

// Créer une instance du prédicteur
let mut predictor = FertilityPredictor::new();

// Charger les modèles
predictor.load_models("models/fertility_sensor.model", "models/fertility_image.model").await?;

// Prédire à partir de données de capteurs
let (prediction, recommendation) = predictor.predict_from_sensor_data(request).await?;

// Prédire à partir d'une image
let (fertility_map, recommendation) = predictor.predict_from_image(request).await?;
```

## API REST

Le module expose les endpoints suivants via l'API REST :

- `POST /api/v1/fertility/predict/sensor` : Prédiction à partir de données de capteurs
- `POST /api/v1/fertility/predict/image` : Prédiction à partir d'une image
- `GET /api/v1/fertility/history/{field_id}` : Historique des prédictions pour un champ
- `GET /api/v1/fertility/recommendations/{field_id}` : Recommandations de fertilisation pour un champ

