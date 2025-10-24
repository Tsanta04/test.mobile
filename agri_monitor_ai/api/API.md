# Documentation de l'API de surveillance agricole

Cette documentation décrit les endpoints de l'API de surveillance agricole et leur utilisation.

## Base URL

```
http://localhost:8080
```

## Authentification

L'API utilise l'authentification par jeton JWT. Pour accéder aux endpoints protégés, incluez le jeton dans l'en-tête `Authorization` :

```
Authorization: Bearer <token>
```

## Endpoints

### Santé de l'API

#### GET /health

Vérifie l'état de l'API.

**Réponse**

```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

### Détection de maladies

#### POST /api/v1/disease-detection/predict/sensor

Prédit les maladies des plantes à partir des données de capteurs.

**Corps de la requête**

```json
{
  "sensor_data": {
    "timestamp": "2023-08-17T14:30:00Z",
    "location": {
      "latitude": 48.8566,
      "longitude": 2.3522
    },
    "soil_moisture": 0.35,
    "air_humidity": 0.65,
    "temperature": 25.5,
    "soil_ph": 6.8,
    "nitrogen": 120,
    "phosphorus": 45,
    "potassium": 80
  }
}
```

**Réponse**

```json
{
  "prediction": {
    "disease_type": "bacterial_blight",
    "probability": 0.85,
    "timestamp": "2023-08-17T14:30:05Z"
  },
  "recommendations": [
    "Appliquer un fongicide à base de cuivre",
    "Augmenter l'espacement entre les plantes pour améliorer la circulation de l'air",
    "Éviter l'irrigation par aspersion"
  ]
}
```

#### POST /api/v1/disease-detection/predict/image

Prédit les maladies des plantes à partir d'une image.

**Corps de la requête**

```json
{
  "image_data": {
    "timestamp": "2023-08-17T14:30:00Z",
    "location": {
      "latitude": 48.8566,
      "longitude": 2.3522
    },
    "image": "base64_encoded_image_data",
    "image_type": "jpeg"
  }
}
```

**Réponse**

```json
{
  "prediction": {
    "disease_type": "bacterial_blight",
    "probability": 0.85,
    "timestamp": "2023-08-17T14:30:05Z",
    "bounding_boxes": [
      {
        "x": 100,
        "y": 150,
        "width": 200,
        "height": 150,
        "label": "bacterial_blight",
        "confidence": 0.92
      }
    ]
  },
  "recommendations": [
    "Appliquer un fongicide à base de cuivre",
    "Augmenter l'espacement entre les plantes pour améliorer la circulation de l'air",
    "Éviter l'irrigation par aspersion"
  ]
}
```

### Taux d'eau

#### POST /api/v1/water-level/predict/sensor

Prédit le taux d'eau à partir des données de capteurs.

**Corps de la requête**

```json
{
  "sensor_data": {
    "timestamp": "2023-08-17T14:30:00Z",
    "location": {
      "latitude": 48.8566,
      "longitude": 2.3522
    },
    "soil_moisture": 0.35,
    "air_humidity": 0.65,
    "temperature": 25.5
  }
}
```

**Réponse**

```json
{
  "prediction": {
    "water_level": 0.35,
    "timestamp": "2023-08-17T14:30:05Z",
    "status": "normal"
  },
  "recommendations": [
    "Maintenir le niveau d'irrigation actuel",
    "Surveiller les prévisions météorologiques pour ajuster l'irrigation si nécessaire"
  ]
}
```

#### POST /api/v1/water-level/predict/image

Prédit le taux d'eau à partir d'une image.

**Corps de la requête**

```json
{
  "image_data": {
    "timestamp": "2023-08-17T14:30:00Z",
    "location": {
      "latitude": 48.8566,
      "longitude": 2.3522
    },
    "image": "base64_encoded_image_data",
    "image_type": "jpeg"
  }
}
```

**Réponse**

```json
{
  "prediction": {
    "water_level_map": [
      [0.3, 0.35, 0.4],
      [0.32, 0.36, 0.42],
      [0.34, 0.38, 0.44]
    ],
    "average_water_level": 0.37,
    "timestamp": "2023-08-17T14:30:05Z",
    "status": "normal"
  },
  "recommendations": [
    "Maintenir le niveau d'irrigation actuel",
    "Surveiller les zones plus sèches au nord-est de la parcelle"
  ]
}
```

### Fertilité

#### POST /api/v1/fertility/predict/sensor

Prédit la fertilité du sol à partir des données de capteurs.

**Corps de la requête**

```json
{
  "sensor_data": {
    "timestamp": "2023-08-17T14:30:00Z",
    "location": {
      "latitude": 48.8566,
      "longitude": 2.3522
    },
    "soil_ph": 6.8,
    "nitrogen": 120,
    "phosphorus": 45,
    "potassium": 80,
    "soil_temperature": 22.5
  }
}
```

**Réponse**

```json
{
  "prediction": {
    "fertility_score": 7.5,
    "timestamp": "2023-08-17T14:30:05Z",
    "nutrient_levels": {
      "nitrogen": "adequate",
      "phosphorus": "low",
      "potassium": "adequate",
      "ph": "optimal"
    }
  },
  "recommendations": [
    "Ajouter un engrais riche en phosphore",
    "Maintenir le pH actuel du sol"
  ]
}
```

#### POST /api/v1/fertility/predict/image

Prédit la fertilité du sol à partir d'une image.

**Corps de la requête**

```json
{
  "image_data": {
    "timestamp": "2023-08-17T14:30:00Z",
    "location": {
      "latitude": 48.8566,
      "longitude": 2.3522
    },
    "image": "base64_encoded_image_data",
    "image_type": "jpeg"
  }
}
```

**Réponse**

```json
{
  "prediction": {
    "fertility_map": [
      [7.2, 7.3, 7.4],
      [7.3, 7.5, 7.6],
      [7.4, 7.6, 7.8]
    ],
    "average_fertility": 7.5,
    "timestamp": "2023-08-17T14:30:05Z"
  },
  "recommendations": [
    "Ajouter un engrais riche en phosphore dans les zones nord-ouest",
    "Maintenir le pH actuel du sol"
  ]
}
```

### Qualité de l'air

#### POST /api/v1/air-quality/predict/sensor

Prédit la qualité de l'air à partir des données de capteurs.

**Corps de la requête**

```json
{
  "sensor_data": {
    "timestamp": "2023-08-17T14:30:00Z",
    "location": {
      "latitude": 48.8566,
      "longitude": 2.3522
    },
    "co2": 400,
    "pm25": 10,
    "pm10": 20,
    "temperature": 25.5,
    "humidity": 0.65
  }
}
```

**Réponse**

```json
{
  "prediction": {
    "air_quality_index": 50,
    "timestamp": "2023-08-17T14:30:05Z",
    "category": "good",
    "pollutant_levels": {
      "co2": "normal",
      "pm25": "low",
      "pm10": "low"
    }
  },
  "recommendations": [
    "Aucune action nécessaire, la qualité de l'air est bonne"
  ]
}
```

#### POST /api/v1/air-quality/predict/image

Prédit la qualité de l'air à partir d'une image.

**Corps de la requête**

```json
{
  "image_data": {
    "timestamp": "2023-08-17T14:30:00Z",
    "location": {
      "latitude": 48.8566,
      "longitude": 2.3522
    },
    "image": "base64_encoded_image_data",
    "image_type": "jpeg"
  }
}
```

**Réponse**

```json
{
  "prediction": {
    "air_quality_map": [
      [45, 48, 52],
      [47, 50, 54],
      [49, 52, 56]
    ],
    "average_air_quality_index": 50,
    "timestamp": "2023-08-17T14:30:05Z",
    "category": "good"
  },
  "recommendations": [
    "Aucune action nécessaire, la qualité de l'air est bonne"
  ]
}
```

### Analyse de rentabilité

#### POST /api/v1/profitability/predict/sensor

Prédit la rentabilité à partir des données de capteurs et économiques.

**Corps de la requête**

```json
{
  "sensor_data": {
    "timestamp": "2023-08-17T14:30:00Z",
    "location": {
      "latitude": 48.8566,
      "longitude": 2.3522
    },
    "soil_moisture": 0.35,
    "air_humidity": 0.65,
    "temperature": 25.5,
    "soil_ph": 6.8,
    "nitrogen": 120,
    "phosphorus": 45,
    "potassium": 80
  },
  "economic_data": {
    "crop_type": "wheat",
    "field_size_ha": 10,
    "fertilizer_cost_per_ha": 200,
    "water_cost_per_ha": 150,
    "labor_cost_per_ha": 300,
    "expected_yield_per_ha": 5,
    "expected_price_per_ton": 250
  }
}
```

**Réponse**

```json
{
  "prediction": {
    "profitability_per_ha": 600,
    "total_profitability": 6000,
    "timestamp": "2023-08-17T14:30:05Z",
    "roi": 0.92,
    "breakdown": {
      "revenue": 12500,
      "costs": {
        "fertilizer": 2000,
        "water": 1500,
        "labor": 3000
      }
    }
  },
  "recommendations": [
    "Réduire les coûts d'engrais en utilisant des méthodes d'application plus précises",
    "Envisager d'augmenter la superficie cultivée pour améliorer les économies d'échelle"
  ]
}
```

#### POST /api/v1/profitability/predict/image

Prédit la rentabilité à partir d'une image et de données économiques.

**Corps de la requête**

```json
{
  "image_data": {
    "timestamp": "2023-08-17T14:30:00Z",
    "location": {
      "latitude": 48.8566,
      "longitude": 2.3522
    },
    "image": "base64_encoded_image_data",
    "image_type": "jpeg"
  },
  "economic_data": {
    "crop_type": "wheat",
    "field_size_ha": 10,
    "fertilizer_cost_per_ha": 200,
    "water_cost_per_ha": 150,
    "labor_cost_per_ha": 300,
    "expected_price_per_ton": 250
  }
}
```

**Réponse**

```json
{
  "prediction": {
    "profitability_map": [
      [550, 580, 610],
      [570, 600, 630],
      [590, 620, 650]
    ],
    "average_profitability_per_ha": 600,
    "total_profitability": 6000,
    "timestamp": "2023-08-17T14:30:05Z",
    "roi": 0.92
  },
  "recommendations": [
    "Concentrer les efforts sur les zones nord-est qui montrent une rentabilité plus élevée",
    "Réduire les coûts d'engrais en utilisant des méthodes d'application plus précises"
  ]
}
```

### Date de production

#### POST /api/v1/production-date/predict/sensor

Prédit la date de production à partir des données de capteurs.

**Corps de la requête**

```json
{
  "sensor_data": {
    "timestamp": "2023-08-17T14:30:00Z",
    "location": {
      "latitude": 48.8566,
      "longitude": 2.3522
    },
    "soil_moisture": 0.35,
    "air_humidity": 0.65,
    "temperature": 25.5,
    "soil_ph": 6.8,
    "growing_degree_days": 1200
  },
  "crop_data": {
    "crop_type": "wheat",
    "planting_date": "2023-03-15T00:00:00Z",
    "variety": "winter_wheat"
  }
}
```

**Réponse**

```json
{
  "prediction": {
    "estimated_harvest_date": "2023-08-30T00:00:00Z",
    "days_to_harvest": 13,
    "timestamp": "2023-08-17T14:30:05Z",
    "confidence": 0.85
  },
  "recommendations": [
    "Préparer l'équipement de récolte",
    "Surveiller les prévisions météorologiques pour planifier la récolte"
  ]
}
```

#### POST /api/v1/production-date/predict/image

Prédit la date de production à partir d'une image.

**Corps de la requête**

```json
{
  "image_data": {
    "timestamp": "2023-08-17T14:30:00Z",
    "location": {
      "latitude": 48.8566,
      "longitude": 2.3522
    },
    "image": "base64_encoded_image_data",
    "image_type": "jpeg"
  },
  "crop_data": {
    "crop_type": "wheat",
    "planting_date": "2023-03-15T00:00:00Z",
    "variety": "winter_wheat"
  }
}
```

**Réponse**

```json
{
  "prediction": {
    "maturity_map": [
      [0.8, 0.82, 0.84],
      [0.81, 0.83, 0.85],
      [0.82, 0.84, 0.86]
    ],
    "average_maturity": 0.83,
    "estimated_harvest_date": "2023-08-30T00:00:00Z",
    "days_to_harvest": 13,
    "timestamp": "2023-08-17T14:30:05Z",
    "confidence": 0.85
  },
  "recommendations": [
    "Commencer la récolte par les zones sud-est qui montrent une maturité plus avancée",
    "Préparer l'équipement de récolte"
  ]
}
```

### Taux de production

#### POST /api/v1/production-rate/predict/sensor

Prédit le taux de production à partir des données de capteurs.

**Corps de la requête**

```json
{
  "sensor_data": {
    "timestamp": "2023-08-17T14:30:00Z",
    "location": {
      "latitude": 48.8566,
      "longitude": 2.3522
    },
    "soil_moisture": 0.35,
    "air_humidity": 0.65,
    "temperature": 25.5,
    "soil_ph": 6.8,
    "nitrogen": 120,
    "phosphorus": 45,
    "potassium": 80,
    "growing_degree_days": 1200
  },
  "crop_data": {
    "crop_type": "wheat",
    "planting_date": "2023-03-15T00:00:00Z",
    "variety": "winter_wheat",
    "field_size_ha": 10
  }
}
```

**Réponse**

```json
{
  "prediction": {
    "estimated_yield_per_ha": 5.2,
    "total_estimated_yield": 52,
    "timestamp": "2023-08-17T14:30:05Z",
    "confidence": 0.8
  },
  "recommendations": [
    "Maintenir le niveau d'irrigation actuel pour maximiser le rendement",
    "Envisager une application supplémentaire d'engrais azoté pour augmenter le rendement"
  ]
}
```

#### POST /api/v1/production-rate/predict/image

Prédit le taux de production à partir d'une image.

**Corps de la requête**

```json
{
  "image_data": {
    "timestamp": "2023-08-17T14:30:00Z",
    "location": {
      "latitude": 48.8566,
      "longitude": 2.3522
    },
    "image": "base64_encoded_image_data",
    "image_type": "jpeg"
  },
  "crop_data": {
    "crop_type": "wheat",
    "planting_date": "2023-03-15T00:00:00Z",
    "variety": "winter_wheat",
    "field_size_ha": 10
  }
}
```

**Réponse**

```json
{
  "prediction": {
    "yield_map": [
      [4.8, 5.0, 5.2],
      [5.0, 5.2, 5.4],
      [5.2, 5.4, 5.6]
    ],
    "average_yield_per_ha": 5.2,
    "total_estimated_yield": 52,
    "timestamp": "2023-08-17T14:30:05Z",
    "confidence": 0.8
  },
  "recommendations": [
    "Concentrer les efforts sur les zones nord-ouest qui montrent un rendement plus faible",
    "Envisager une application supplémentaire d'engrais azoté dans ces zones"
  ]
}
```

### Détection de catastrophes naturelles

#### POST /api/v1/disaster-detection/predict/sensor

Prédit les risques de catastrophes naturelles à partir des données de capteurs.

**Corps de la requête**

```json
{
  "sensor_data": {
    "timestamp": "2023-08-17T14:30:00Z",
    "location": {
      "latitude": 48.8566,
      "longitude": 2.3522
    },
    "soil_moisture": 0.35,
    "air_humidity": 0.65,
    "temperature": 25.5,
    "rainfall": 10,
    "wind_speed": 5
  }
}
```

**Réponse**

```json
{
  "prediction": {
    "disaster_risks": {
      "flood": 0.05,
      "drought": 0.1,
      "frost": 0.01,
      "hail": 0.02,
      "wind_damage": 0.15
    },
    "highest_risk": "wind_damage",
    "timestamp": "2023-08-17T14:30:05Z"
  },
  "recommendations": [
    "Risque faible de dommages dus au vent, mais surveiller les prévisions météorologiques",
    "Aucune action immédiate nécessaire"
  ]
}
```

#### POST /api/v1/disaster-detection/predict/image

Prédit les risques de catastrophes naturelles à partir d'une image.

**Corps de la requête**

```json
{
  "image_data": {
    "timestamp": "2023-08-17T14:30:00Z",
    "location": {
      "latitude": 48.8566,
      "longitude": 2.3522
    },
    "image": "base64_encoded_image_data",
    "image_type": "jpeg"
  }
}
```

**Réponse**

```json
{
  "prediction": {
    "disaster_detected": false,
    "damage_assessment": {
      "affected_area_percentage": 0,
      "severity": "none"
    },
    "timestamp": "2023-08-17T14:30:05Z"
  },
  "recommendations": [
    "Aucun dommage détecté",
    "Continuer la surveillance régulière"
  ]
}
```

## Codes d'erreur

- `400 Bad Request` - La requête est mal formée ou contient des données invalides
- `401 Unauthorized` - Authentification requise
- `403 Forbidden` - Accès refusé
- `404 Not Found` - Ressource non trouvée
- `500 Internal Server Error` - Erreur interne du serveur

## Limites de l'API

- Taille maximale des images : 10 Mo
- Nombre maximal de requêtes par minute : 100
- Nombre maximal de requêtes par jour : 10 000

