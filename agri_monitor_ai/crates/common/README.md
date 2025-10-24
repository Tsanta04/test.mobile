# Module Commun pour la Surveillance Agricole

Ce module fournit des fonctionnalités partagées entre tous les modules de prédiction du système de surveillance agricole.

## Composants

### Structures de données

Le module définit des structures de données communes utilisées par tous les modules de prédiction:

- `SensorData`: Données de capteurs (humidité, température, etc.)
- `ImageData`: Données d'images (chemin, type, métadonnées)
- `GeoLocation`: Localisation géographique (latitude, longitude, altitude)
- `PredictionResult<T>`: Résultat de prédiction générique avec confiance

### Connexions aux bases de données

Le module fournit des clients pour se connecter aux différentes bases de données:

- `InfluxClient`: Client pour InfluxDB (données de séries temporelles)
- `MongoClient`: Client pour MongoDB (images et métadonnées)
- `PostgresClient`: Client pour PostgreSQL avec Diesel ORM (données relationnelles)

### Gestion des erreurs

Le module définit un type d'erreur commun utilisé par tous les modules:

- `AgriMonitorError`: Erreurs spécifiques au système
- `AgriResult<T>`: Type de résultat avec `AgriMonitorError`

### Interfaces de modèles

Le module définit des traits pour les modèles de prédiction:

- `SensorDataModel<T>`: Modèles qui font des prédictions à partir de données de capteurs
- `ImageModel<T>`: Modèles qui font des prédictions à partir d'images
- `IncrementalModel`: Modèles qui peuvent être mis à jour incrémentalement
- `ExplainableModel<T, E>`: Modèles qui peuvent expliquer leurs prédictions

### Utilitaires

Le module fournit des fonctions utilitaires:

- `image_utils`: Fonctions pour le traitement d'images
- `tensor_utils`: Fonctions pour le traitement de tenseurs
- `validation`: Fonctions pour la validation des données

## Utilisation

### Structures de données

```rust
use common::data::{SensorData, GeoLocation};
use chrono::Utc;
use std::collections::HashMap;

// Créer des données de capteurs
let sensor_data = SensorData {
    timestamp: Utc::now(),
    location: GeoLocation {
        latitude: 48.8566,
        longitude: 2.3522,
        altitude: Some(35.0),
    },
    soil_moisture: Some(35.5),
    air_humidity: Some(65.2),
    temperature: Some(22.3),
    soil_ph: Some(6.8),
    nitrogen: Some(120.5),
    phosphorus: Some(45.2),
    potassium: Some(200.1),
    co2: None,
    pm25: None,
    pm10: None,
    wind_speed: None,
    rainfall: None,
    solar_radiation: None,
    additional_data: HashMap::new(),
};
```

### Connexion à InfluxDB

```rust
use common::db::influx::init_influx_client;

async fn connect_to_influxdb() {
    // Initialiser le client InfluxDB
    let influx_client = init_influx_client().unwrap();
    
    // Écrire des données
    influx_client.write(/* ... */).await.unwrap();
    
    // Lire des données
    let result = influx_client.query("from(bucket: \"sensor_data\") |> range(start: -1h)").await.unwrap();
    println!("Result: {}", result);
}
```

### Connexion à MongoDB

```rust
use common::db::mongo::init_mongo_client;
use mongodb::bson::doc;

async fn connect_to_mongodb() {
    // Initialiser le client MongoDB
    let mongo_client = init_mongo_client().await.unwrap();
    
    // Insérer un document
    let document = doc! {
        "name": "Plant image",
        "timestamp": chrono::Utc::now(),
        "metadata": {
            "camera": "DJI Phantom 4",
            "altitude": "10m"
        }
    };
    
    let id = mongo_client.insert_one("images", document).await.unwrap();
    println!("Inserted document with ID: {}", id);
    
    // Trouver des documents
    let filter = doc! { "name": "Plant image" };
    let documents = mongo_client.find("images", filter).await.unwrap();
    println!("Found {} documents", documents.len());
}
```

### Connexion à PostgreSQL

```rust
use common::db::postgres::init_postgres_client;
use diesel::prelude::*;

async fn connect_to_postgres() {
    // Initialiser le client PostgreSQL
    let postgres_client = init_postgres_client().unwrap();
    
    // Obtenir une connexion
    let conn = postgres_client.get_connection().unwrap();
    
    // Exécuter une requête
    // (Ceci est un exemple, vous devriez utiliser les modèles Diesel)
    diesel::sql_query("SELECT * FROM crops")
        .load::<Crop>(&conn)
        .unwrap();
}
```

### Traitement d'images

```rust
use common::utils::image_utils;
use std::path::Path;

fn process_image() {
    // Charger une image
    let image = image_utils::load_image(Path::new("plant.jpg")).unwrap();
    
    // Redimensionner l'image
    let resized = image_utils::resize_image(&image, 224, 224);
    
    // Convertir en tableau
    let array = image_utils::image_to_array(&resized);
    
    println!("Image shape: {:?}", array.shape());
}
```

### Traitement de tenseurs

```rust
use common::utils::tensor_utils;
use ndarray::Array1;

fn process_tensor() {
    // Créer un tableau
    let array = Array1::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    
    // Normaliser le tableau
    let normalized = tensor_utils::normalize(&array).unwrap();
    
    println!("Normalized: {:?}", normalized);
    
    // Standardiser le tableau
    let standardized = tensor_utils::standardize(&array).unwrap();
    
    println!("Standardized: {:?}", standardized);
}
```

### Validation des données

```rust
use common::data::{SensorData, GeoLocation};
use common::utils::validation;
use chrono::Utc;
use std::collections::HashMap;

fn validate_data() {
    // Créer des données de capteurs
    let sensor_data = SensorData {
        timestamp: Utc::now(),
        location: GeoLocation {
            latitude: 48.8566,
            longitude: 2.3522,
            altitude: Some(35.0),
        },
        soil_moisture: Some(35.5),
        air_humidity: Some(65.2),
        temperature: Some(22.3),
        soil_ph: Some(6.8),
        nitrogen: Some(120.5),
        phosphorus: Some(45.2),
        potassium: Some(200.1),
        co2: None,
        pm25: None,
        pm10: None,
        wind_speed: None,
        rainfall: None,
        solar_radiation: None,
        additional_data: HashMap::new(),
    };
    
    // Valider les données
    validation::validate_sensor_data(&sensor_data).unwrap();
}
```

