# Module de Détection de Maladies des Plantes

Ce module fournit des fonctionnalités pour détecter les maladies des plantes en utilisant:
1. Des données brutes de capteurs (humidité du sol, température, etc.)
2. Des images (RGB ou multispectrales) de plantes

## Fonctionnalités

### Détection par données brutes

- **Données d'entrée**: Valeurs de capteurs (humidité du sol/air, température, pH, nutriments NPK)
- **Données de sortie**: Probabilité de maladie, type de maladie identifié
- **Fréquence de prédiction**: Toutes les heures (basée sur mises à jour des capteurs toutes les 15-30 min)
- **Base de données**: InfluxDB pour stocker les séries temporelles des capteurs
- **Méthode IA**: Random Forest pour la classification des maladies

### Détection par image

- **Données d'entrée**: Images de feuilles/plantes (RGB ou multispectrales) capturées par caméras fixes/drones
- **Données de sortie**: Localisation et type de maladie sur l'image
- **Fréquence de prédiction**: Quotidienne (ou toutes les 2-3 heures en cas de risque élevé)
- **Base de données**: MongoDB pour stocker les images et métadonnées
- **Méthode IA**: CNN (Convolutional Neural Network) pour la classification des maladies

## Architecture du module

### Modèles

- `DiseaseSensorModel`: Modèle Random Forest pour la détection par données de capteurs
- `DiseaseImageModel`: Modèle CNN pour la détection par images

### Types

- `DiseaseDetection`: Résultat de la détection de maladie
- `DiseaseType`: Types de maladies (bactériennes, fongiques, virales, etc.)

### Utilitaires

- Fonctions pour combiner les prédictions des deux modèles
- Fonctions pour charger et sauvegarder les modèles

## Utilisation

### Détection par données de capteurs

```rust
use common::data::SensorData;
use disease_detection::models::sensor_model::DiseaseSensorModel;
use disease_detection::utils;

async fn detect_disease(sensor_data: SensorData) {
    // Charger le modèle
    let sensor_model = DiseaseSensorModel::default();
    
    // Faire une prédiction
    let prediction = sensor_model.predict(&sensor_data).await.unwrap();
    
    println!("Maladie détectée: {}", prediction.prediction.disease_detected);
    if let Some(disease_type) = &prediction.prediction.disease_type {
        println!("Type de maladie: {:?}", disease_type);
    }
    println!("Confiance: {}", prediction.confidence);
}
```

### Détection par image

```rust
use common::data::ImageData;
use disease_detection::models::image_model::DiseaseImageModel;
use disease_detection::utils;

async fn detect_disease(image_data: ImageData) {
    // Charger le modèle
    let image_model = DiseaseImageModel::default();
    
    // Faire une prédiction
    let prediction = image_model.predict(&image_data).await.unwrap();
    
    println!("Maladie détectée: {}", prediction.prediction.disease_detected);
    if let Some(disease_type) = &prediction.prediction.disease_type {
        println!("Type de maladie: {:?}", disease_type);
    }
    println!("Confiance: {}", prediction.confidence);
}
```

### Combinaison des deux modèles

```rust
use common::data::{ImageData, SensorData};
use disease_detection::models::{image_model::DiseaseImageModel, sensor_model::DiseaseSensorModel};
use disease_detection::utils;

async fn detect_disease(sensor_data: SensorData, image_data: ImageData) {
    // Charger les modèles
    let sensor_model = DiseaseSensorModel::default();
    let image_model = DiseaseImageModel::default();
    
    // Faire une prédiction combinée
    let prediction = utils::detect_disease(
        &sensor_model,
        &image_model,
        Some(&sensor_data),
        Some(&image_data),
    ).await.unwrap();
    
    println!("Maladie détectée: {}", prediction.prediction.disease_detected);
    if let Some(disease_type) = &prediction.prediction.disease_type {
        println!("Type de maladie: {:?}", disease_type);
    }
    println!("Confiance: {}", prediction.confidence);
}
```

## Entraînement des modèles

### Données d'entraînement

- **Datasets suggérés**:
  - PlantVillage (87K images de 38 classes de maladies sur cultures)
  - CCMT Dataset pour pests et maladies
  - New Plant Diseases Dataset (87K images RGB)
  - PlantDoc (2.5K images pour 30 classes)

### Entraînement du modèle de capteurs

```rust
use common::data::SensorData;
use disease_detection::models::sensor_model::DiseaseSensorModel;
use disease_detection::types::DiseaseDetection;

async fn train_model(training_data: Vec<SensorData>, labels: Vec<DiseaseDetection>) {
    // Créer le modèle
    let mut model = DiseaseSensorModel::default();
    
    // Entraîner le modèle
    model.train(&training_data, &labels).await.unwrap();
    
    // Sauvegarder le modèle
    model.save("models/disease_sensor_model.bin").await.unwrap();
}
```

### Entraînement du modèle d'images

```rust
use common::data::ImageData;
use disease_detection::models::image_model::DiseaseImageModel;
use disease_detection::types::DiseaseDetection;

async fn train_model(training_data: Vec<ImageData>, labels: Vec<DiseaseDetection>) {
    // Créer le modèle
    let mut model = DiseaseImageModel::default();
    
    // Entraîner le modèle
    model.train(&training_data, &labels).await.unwrap();
    
    // Sauvegarder le modèle
    model.save("models/disease_image_model.bin").await.unwrap();
}
```

