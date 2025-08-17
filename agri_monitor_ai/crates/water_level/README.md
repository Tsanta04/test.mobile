# Module de Prédiction du Taux d'Eau et Recommandation d'Arrosage

Ce module fournit des fonctionnalités pour prédire le taux d'humidité du sol et générer des recommandations d'arrosage pour les cultures agricoles.

## Fonctionnalités

- **Prédiction du taux d'humidité du sol** à partir de données de capteurs
- **Génération de cartes d'humidité** à partir d'images multispectrales
- **Recommandations d'arrosage** (décision et quantité d'eau)
- **Calcul du déficit hydrique** basé sur les besoins des cultures

## Architecture

Le module est organisé en plusieurs composants :

- **WaterLevelPredictor** : Gestionnaire principal qui coordonne les prédictions
- **SensorModel** : Modèle pour prédire le taux d'humidité à partir de données de capteurs
- **ImageModel** : Modèle pour générer des cartes d'humidité à partir d'images
- **Types** : Définitions des types de données spécifiques
- **Utils** : Fonctions utilitaires pour les calculs et transformations

## Modèles de Prédiction

### Prédiction à partir de données de capteurs

Le modèle utilise un algorithme de Random Forest pour prédire le taux d'humidité du sol à partir de données de capteurs telles que :
- Humidité du sol
- Humidité de l'air
- Température
- Précipitations récentes

### Prédiction à partir d'images

Le modèle analyse des images multispectrales pour générer des cartes d'humidité en utilisant :
- Calcul de l'indice NDVI (Normalized Difference Vegetation Index)
- Conversion de l'indice NDVI en taux d'humidité estimé
- Génération de cartes de distribution spatiale de l'humidité

## Recommandations d'Arrosage

Les recommandations d'arrosage sont générées en tenant compte de :
- Taux d'humidité actuel du sol
- Besoins en eau de la culture (coefficient cultural)
- Conditions météorologiques (température, humidité, précipitations prévues)
- Stade de croissance de la culture
- Profondeur des racines

## Utilisation

### Prédiction à partir de données de capteurs

```rust
use water_level::{WaterLevelPredictor, types::*};
use common::data::SensorData;

async fn predict_water_level(sensor_data: SensorData) {
    // Créer le prédicteur
    let predictor = WaterLevelPredictor::new();
    
    // Créer la requête
    let request = SensorPredictionRequest {
        sensor_data,
        weather_data: WeatherData { /* ... */ },
        crop_data: CropData { /* ... */ },
        field_id: Some("field_123".to_string()),
    };
    
    // Effectuer la prédiction
    let (prediction, recommendation) = predictor.predict_from_sensor_data(request).await.unwrap();
    
    println!("Taux d'humidité: {}%", prediction.water_level * 100.0);
    println!("Irrigation nécessaire: {}", recommendation.irrigation_needed);
    println!("Quantité d'eau recommandée: {} mm", recommendation.water_amount_mm);
}
```

### Prédiction à partir d'images

```rust
use water_level::{WaterLevelPredictor, types::*};
use common::data::ImageData;

async fn predict_water_level_from_image(image_data: ImageData) {
    // Créer le prédicteur
    let predictor = WaterLevelPredictor::new();
    
    // Créer la requête
    let request = ImagePredictionRequest {
        image_data,
        weather_data: WeatherData { /* ... */ },
        crop_data: CropData { /* ... */ },
        field_id: Some("field_123".to_string()),
    };
    
    // Effectuer la prédiction
    let (water_map, recommendation) = predictor.predict_from_image(request).await.unwrap();
    
    println!("Taux d'humidité moyen: {}%", water_map.average_water_level * 100.0);
    println!("Irrigation nécessaire: {}", recommendation.irrigation_needed);
    println!("Quantité d'eau recommandée: {} mm", recommendation.water_amount_mm);
}
```

## API REST

Le module expose les endpoints suivants via l'API REST :

- `POST /api/v1/water-level/predict/sensor` : Prédiction à partir de données de capteurs
- `POST /api/v1/water-level/predict/image` : Prédiction à partir d'une image
- `GET /api/v1/water-level/history/{field_id}` : Historique des prédictions pour un champ
- `GET /api/v1/water-level/recommendations/{field_id}` : Recommandations d'arrosage pour un champ

## Stockage des Données

- **InfluxDB** : Stockage des séries temporelles (données de capteurs, prédictions)
- **MongoDB** : Stockage des cartes d'humidité générées à partir d'images
- **PostgreSQL** : Références aux champs et plantations

## Tests

Le module inclut des tests unitaires et d'intégration pour valider les fonctionnalités :

```bash
cargo test -p water_level
```

