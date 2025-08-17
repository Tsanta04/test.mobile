# Modèle de Données - MongoDB

Ce document décrit le modèle de données pour la base de données MongoDB utilisée dans le système de surveillance agricole.

## Collections et Schémas

### Collection: images

```javascript
{
  _id: ObjectId,
  timestamp: ISODate,
  location: {
    type: "Point",
    coordinates: [longitude, latitude]  // Format GeoJSON
  },
  field_id: UUID,  // Référence au champ dans PostgreSQL
  planting_id: UUID,  // Référence optionnelle à la plantation dans PostgreSQL
  image_type: String,  // "rgb", "multispectral", "thermal", "satellite", etc.
  capture_device: String,  // "drone", "fixed_camera", "satellite", "mobile", etc.
  resolution: {
    width: Number,
    height: Number
  },
  format: String,  // "jpeg", "png", "tiff", etc.
  size_bytes: Number,
  url: String,  // URL de stockage de l'image (peut être dans un stockage externe)
  binary_data: Binary,  // Données binaires de l'image (pour les petites images)
  metadata: {
    altitude: Number,  // Pour les images de drone
    weather_conditions: String,
    device_id: String,
    capture_settings: Object  // Paramètres spécifiques à l'appareil
  },
  tags: [String],
  created_at: ISODate,
  updated_at: ISODate
}
```

### Collection: disease_detections

```javascript
{
  _id: ObjectId,
  image_id: ObjectId,  // Référence à l'image analysée
  timestamp: ISODate,
  field_id: UUID,  // Référence au champ dans PostgreSQL
  planting_id: UUID,  // Référence à la plantation dans PostgreSQL
  disease_type: String,  // "bacterial_blight", "fungal_rust", "viral_mosaic", etc.
  probability: Number,  // 0.0 à 1.0
  severity: String,  // "low", "medium", "high"
  affected_area_percentage: Number,  // 0.0 à 100.0
  bounding_boxes: [
    {
      x: Number,
      y: Number,
      width: Number,
      height: Number,
      label: String,
      confidence: Number
    }
  ],
  recommendations: [String],
  model_version: String,
  created_at: ISODate
}
```

### Collection: water_level_maps

```javascript
{
  _id: ObjectId,
  timestamp: ISODate,
  field_id: UUID,  // Référence au champ dans PostgreSQL
  source_type: String,  // "sensor_data", "image_analysis", "satellite"
  source_id: ObjectId,  // Référence à l'image ou aux données de capteur source
  water_level_map: [
    [Number]  // Matrice 2D représentant les niveaux d'eau
  ],
  resolution: {
    width: Number,
    height: Number,
    meters_per_pixel: Number
  },
  geo_reference: {
    top_left: {
      type: "Point",
      coordinates: [longitude, latitude]
    },
    bottom_right: {
      type: "Point",
      coordinates: [longitude, latitude]
    }
  },
  average_water_level: Number,
  status: String,  // "dry", "normal", "wet", "flooded"
  recommendations: [String],
  model_version: String,
  created_at: ISODate
}
```

### Collection: fertility_maps

```javascript
{
  _id: ObjectId,
  timestamp: ISODate,
  field_id: UUID,  // Référence au champ dans PostgreSQL
  source_type: String,  // "sensor_data", "image_analysis", "satellite"
  source_id: ObjectId,  // Référence à l'image ou aux données de capteur source
  fertility_map: [
    [Number]  // Matrice 2D représentant les niveaux de fertilité
  ],
  resolution: {
    width: Number,
    height: Number,
    meters_per_pixel: Number
  },
  geo_reference: {
    top_left: {
      type: "Point",
      coordinates: [longitude, latitude]
    },
    bottom_right: {
      type: "Point",
      coordinates: [longitude, latitude]
    }
  },
  average_fertility: Number,
  nutrient_levels: {
    nitrogen: String,  // "low", "adequate", "high"
    phosphorus: String,
    potassium: String,
    ph: String
  },
  recommendations: [String],
  model_version: String,
  created_at: ISODate
}
```

### Collection: air_quality_maps

```javascript
{
  _id: ObjectId,
  timestamp: ISODate,
  location: {
    type: "Point",
    coordinates: [longitude, latitude]
  },
  radius_meters: Number,  // Rayon de la zone couverte
  source_type: String,  // "sensor_data", "image_analysis", "satellite"
  source_id: ObjectId,  // Référence à l'image ou aux données de capteur source
  air_quality_map: [
    [Number]  // Matrice 2D représentant les indices de qualité de l'air
  ],
  resolution: {
    width: Number,
    height: Number,
    meters_per_pixel: Number
  },
  geo_reference: {
    top_left: {
      type: "Point",
      coordinates: [longitude, latitude]
    },
    bottom_right: {
      type: "Point",
      coordinates: [longitude, latitude]
    }
  },
  average_air_quality_index: Number,
  category: String,  // "good", "moderate", "unhealthy", etc.
  pollutant_levels: {
    co2: String,  // "normal", "elevated", "high"
    pm25: String,
    pm10: String
  },
  recommendations: [String],
  model_version: String,
  created_at: ISODate
}
```

### Collection: yield_maps

```javascript
{
  _id: ObjectId,
  timestamp: ISODate,
  field_id: UUID,  // Référence au champ dans PostgreSQL
  planting_id: UUID,  // Référence à la plantation dans PostgreSQL
  source_type: String,  // "sensor_data", "image_analysis", "satellite", "prediction"
  source_id: ObjectId,  // Référence à l'image ou aux données de capteur source
  yield_map: [
    [Number]  // Matrice 2D représentant les rendements estimés
  ],
  resolution: {
    width: Number,
    height: Number,
    meters_per_pixel: Number
  },
  geo_reference: {
    top_left: {
      type: "Point",
      coordinates: [longitude, latitude]
    },
    bottom_right: {
      type: "Point",
      coordinates: [longitude, latitude]
    }
  },
  average_yield_per_ha: Number,
  total_estimated_yield: Number,
  confidence_level: Number,  // 0.0 à 1.0
  recommendations: [String],
  model_version: String,
  created_at: ISODate
}
```

### Collection: maturity_maps

```javascript
{
  _id: ObjectId,
  timestamp: ISODate,
  field_id: UUID,  // Référence au champ dans PostgreSQL
  planting_id: UUID,  // Référence à la plantation dans PostgreSQL
  source_type: String,  // "sensor_data", "image_analysis", "satellite"
  source_id: ObjectId,  // Référence à l'image ou aux données de capteur source
  maturity_map: [
    [Number]  // Matrice 2D représentant les niveaux de maturité (0.0 à 1.0)
  ],
  resolution: {
    width: Number,
    height: Number,
    meters_per_pixel: Number
  },
  geo_reference: {
    top_left: {
      type: "Point",
      coordinates: [longitude, latitude]
    },
    bottom_right: {
      type: "Point",
      coordinates: [longitude, latitude]
    }
  },
  average_maturity: Number,
  estimated_harvest_date: ISODate,
  days_to_harvest: Number,
  confidence_level: Number,  // 0.0 à 1.0
  recommendations: [String],
  model_version: String,
  created_at: ISODate
}
```

### Collection: disaster_assessments

```javascript
{
  _id: ObjectId,
  timestamp: ISODate,
  field_id: UUID,  // Référence au champ dans PostgreSQL
  disaster_type: String,  // "flood", "drought", "frost", "hail", "wind_damage", etc.
  source_type: String,  // "sensor_data", "image_analysis", "satellite"
  source_id: ObjectId,  // Référence à l'image ou aux données de capteur source
  disaster_detected: Boolean,
  probability: Number,  // 0.0 à 1.0
  damage_assessment: {
    affected_area_percentage: Number,
    severity: String,  // "none", "low", "medium", "high", "severe"
    estimated_loss_percentage: Number
  },
  damage_map: [
    [Number]  // Matrice 2D représentant les niveaux de dommages
  ],
  resolution: {
    width: Number,
    height: Number,
    meters_per_pixel: Number
  },
  geo_reference: {
    top_left: {
      type: "Point",
      coordinates: [longitude, latitude]
    },
    bottom_right: {
      type: "Point",
      coordinates: [longitude, latitude]
    }
  },
  recommendations: [String],
  model_version: String,
  created_at: ISODate
}
```

## Index

Pour optimiser les performances des requêtes, les index suivants sont créés :

```javascript
// Index géospatial pour les requêtes de localisation
db.images.createIndex({ "location": "2dsphere" });
db.air_quality_maps.createIndex({ "location": "2dsphere" });

// Index composites pour les requêtes fréquentes
db.images.createIndex({ "field_id": 1, "timestamp": -1 });
db.images.createIndex({ "planting_id": 1, "timestamp": -1 });
db.disease_detections.createIndex({ "field_id": 1, "timestamp": -1 });
db.disease_detections.createIndex({ "disease_type": 1, "timestamp": -1 });
db.water_level_maps.createIndex({ "field_id": 1, "timestamp": -1 });
db.fertility_maps.createIndex({ "field_id": 1, "timestamp": -1 });
db.yield_maps.createIndex({ "field_id": 1, "timestamp": -1 });
db.yield_maps.createIndex({ "planting_id": 1, "timestamp": -1 });
db.maturity_maps.createIndex({ "planting_id": 1, "timestamp": -1 });
db.disaster_assessments.createIndex({ "field_id": 1, "disaster_type": 1, "timestamp": -1 });

// Index pour les recherches par type
db.images.createIndex({ "image_type": 1 });
db.disease_detections.createIndex({ "disease_type": 1 });

// Index TTL pour la gestion automatique de la durée de vie des données
// (supprime automatiquement les documents après 1 an)
db.images.createIndex({ "created_at": 1 }, { expireAfterSeconds: 31536000 });
```

## Relations avec PostgreSQL

Les collections MongoDB font référence aux entités PostgreSQL via les champs suivants :
- `field_id` : Référence à la table `fields` dans PostgreSQL
- `planting_id` : Référence à la table `plantings` dans PostgreSQL

Ces références permettent de lier les données non structurées (images, cartes) aux données structurées (informations sur les champs, plantations, etc.).

