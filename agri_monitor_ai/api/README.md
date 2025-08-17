# API de Surveillance Agricole

Cette API fournit des endpoints pour accéder aux fonctionnalités de prédiction du système de surveillance agricole.

## Architecture

L'API est construite avec Actix-web, un framework web rapide et léger pour Rust. Elle expose des endpoints RESTful pour chaque module de prédiction.

## Endpoints

### Santé de l'API

- `GET /api/v1/health`: Vérifier l'état de l'API

### Détection de maladies

- `POST /api/v1/disease-detection/sensor`: Détection par données de capteurs
- `POST /api/v1/disease-detection/image`: Détection par image

### Taux d'eau

- `POST /api/v1/water-level/sensor`: Analyse par données de capteurs
- `POST /api/v1/water-level/image`: Analyse par image

### Fertilité

- `POST /api/v1/fertility/sensor`: Analyse par données de capteurs
- `POST /api/v1/fertility/image`: Analyse par image

### Qualité d'air

- `POST /api/v1/air-quality/sensor`: Analyse par données de capteurs
- `POST /api/v1/air-quality/image`: Analyse par image

### Rentabilité

- `POST /api/v1/profitability/data`: Analyse par données économiques
- `POST /api/v1/profitability/image`: Analyse par image

### Date de production

- `POST /api/v1/production-date/sensor`: Prédiction par données de capteurs
- `POST /api/v1/production-date/image`: Prédiction par image

### Taux de production

- `POST /api/v1/production-rate/sensor`: Prédiction par données de capteurs
- `POST /api/v1/production-rate/image`: Prédiction par image

### Catastrophes naturelles

- `POST /api/v1/disaster-detection/sensor`: Détection par données de capteurs
- `POST /api/v1/disaster-detection/image`: Détection par image

## Format des requêtes et réponses

### Format de réponse général

Toutes les réponses suivent ce format:

```json
{
  "success": true,
  "data": { ... },
  "error": null,
  "timestamp": "2025-08-17T13:23:30Z"
}
```

En cas d'erreur:

```json
{
  "success": false,
  "data": null,
  "error": "Message d'erreur",
  "timestamp": "2025-08-17T13:23:30Z"
}
```

### Détection de maladies par capteurs

**Requête:**

```json
{
  "timestamp": "2025-08-17T13:23:30Z",
  "latitude": 48.8566,
  "longitude": 2.3522,
  "soil_moisture": 35.5,
  "air_humidity": 65.2,
  "temperature": 22.3,
  "soil_ph": 6.8,
  "nitrogen": 120.5,
  "phosphorus": 45.2,
  "potassium": 200.1,
  "co2": 410.2,
  "pm25": 15.3,
  "pm10": 25.7,
  "wind_speed": 3.2,
  "rainfall": 0.0,
  "solar_radiation": 850.5,
  "additional_data": {
    "leaf_wetness": 0.8
  }
}
```

**Réponse:**

```json
{
  "success": true,
  "data": {
    "disease_detected": true,
    "disease_type": "Fungal(PowderyMildew)",
    "severity": 0.8,
    "affected_area": null,
    "confidence": 0.9,
    "recommendations": [
      "Remove and destroy infected plant parts",
      "Apply appropriate fungicide",
      "Ensure good air circulation",
      "Avoid overhead irrigation",
      "Rotate crops in the affected area"
    ],
    "additional_info": {},
    "timestamp": "2025-08-17T13:23:30Z"
  },
  "error": null,
  "timestamp": "2025-08-17T13:23:35Z"
}
```

### Détection de maladies par image

**Requête:**

```json
{
  "timestamp": "2025-08-17T13:23:30Z",
  "latitude": 48.8566,
  "longitude": 2.3522,
  "image_data": "base64_encoded_image_data",
  "image_type": "RGB",
  "metadata": {
    "camera": "DJI Phantom 4",
    "altitude": "10m"
  }
}
```

**Réponse:**

```json
{
  "success": true,
  "data": {
    "disease_detected": true,
    "disease_type": "Fungal(PowderyMildew)",
    "severity": 0.8,
    "affected_area": 0.3,
    "confidence": 0.95,
    "recommendations": [
      "Remove and destroy infected plant parts",
      "Apply appropriate fungicide",
      "Ensure good air circulation",
      "Avoid overhead irrigation",
      "Rotate crops in the affected area"
    ],
    "additional_info": {
      "affected_regions": "Mainly on upper leaves"
    },
    "timestamp": "2025-08-17T13:23:30Z"
  },
  "error": null,
  "timestamp": "2025-08-17T13:23:35Z"
}
```

## Configuration

L'API peut être configurée via des variables d'environnement:

- `HOST`: Hôte du serveur (défaut: 0.0.0.0)
- `PORT`: Port du serveur (défaut: 8080)
- `DATABASE_URL`: URL de la base de données PostgreSQL
- `INFLUXDB_URL`: URL de la base de données InfluxDB
- `INFLUXDB_ORG`: Organisation InfluxDB
- `INFLUXDB_BUCKET`: Bucket InfluxDB
- `INFLUXDB_TOKEN`: Token d'authentification InfluxDB
- `MONGODB_URI`: URI de la base de données MongoDB
- `MONGODB_DB`: Nom de la base de données MongoDB
- `MODELS_DIR`: Répertoire des modèles

## Lancement

```bash
cargo run --release -p api
```

## Développement

### Ajout d'un nouvel endpoint

1. Créer un nouveau gestionnaire dans le module approprié
2. Ajouter la route dans `routes.rs`
3. Implémenter la logique de traitement

### Tests

```bash
cargo test -p api
```

