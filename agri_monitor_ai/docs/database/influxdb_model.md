# Modèle de Données - InfluxDB

Ce document décrit le modèle de données pour la base de données InfluxDB utilisée dans le système de surveillance agricole. InfluxDB est une base de données de séries temporelles optimisée pour stocker et interroger des données chronologiques à haute fréquence, comme les mesures de capteurs.

## Structure des Données

InfluxDB utilise un modèle de données différent des bases de données relationnelles et NoSQL. Les concepts clés sont :

- **Bucket** : Équivalent à une base de données, contient des mesures avec une politique de rétention
- **Measurement** : Équivalent à une table, regroupe des points de données similaires
- **Tag** : Métadonnées indexées pour filtrer et regrouper les données (clé-valeur)
- **Field** : Valeurs mesurées non indexées (clé-valeur)
- **Timestamp** : Horodatage précis de chaque point de données

## Buckets

Le système utilise un bucket principal nommé `agri_monitor` avec une politique de rétention de 5 ans.

## Measurements

### Measurement: sensor_data

Stocke toutes les données brutes des capteurs.

```
sensor_data,
  field_id=<uuid>,
  sensor_id=<string>,
  sensor_type=<string>,
  location_lat=<float>,
  location_lon=<float>
  soil_moisture=<float>,
  air_humidity=<float>,
  temperature=<float>,
  soil_ph=<float>,
  nitrogen=<float>,
  phosphorus=<float>,
  potassium=<float>,
  co2=<float>,
  pm25=<float>,
  pm10=<float>,
  rainfall=<float>,
  wind_speed=<float>,
  wind_direction=<float>,
  solar_radiation=<float>,
  battery_level=<float>
  <timestamp>
```

#### Tags (indexés)
- `field_id` : UUID du champ (référence à PostgreSQL)
- `sensor_id` : Identifiant unique du capteur
- `sensor_type` : Type de capteur (soil, weather, air_quality, etc.)
- `location_lat` : Latitude du capteur
- `location_lon` : Longitude du capteur

#### Fields (non indexés)
- `soil_moisture` : Humidité du sol (%)
- `air_humidity` : Humidité de l'air (%)
- `temperature` : Température (°C)
- `soil_ph` : pH du sol
- `nitrogen` : Teneur en azote (ppm)
- `phosphorus` : Teneur en phosphore (ppm)
- `potassium` : Teneur en potassium (ppm)
- `co2` : Niveau de CO2 (ppm)
- `pm25` : Particules fines PM2.5 (µg/m³)
- `pm10` : Particules fines PM10 (µg/m³)
- `rainfall` : Précipitations (mm)
- `wind_speed` : Vitesse du vent (m/s)
- `wind_direction` : Direction du vent (degrés)
- `solar_radiation` : Rayonnement solaire (W/m²)
- `battery_level` : Niveau de batterie du capteur (%)

### Measurement: disease_predictions

Stocke les prédictions de maladies basées sur les données de capteurs.

```
disease_predictions,
  field_id=<uuid>,
  planting_id=<uuid>,
  disease_type=<string>,
  model_version=<string>
  probability=<float>,
  severity=<float>
  <timestamp>
```

#### Tags (indexés)
- `field_id` : UUID du champ
- `planting_id` : UUID de la plantation
- `disease_type` : Type de maladie prédite
- `model_version` : Version du modèle de prédiction

#### Fields (non indexés)
- `probability` : Probabilité de la maladie (0.0 à 1.0)
- `severity` : Sévérité estimée (0.0 à 1.0)

### Measurement: water_level_predictions

Stocke les prédictions de taux d'eau basées sur les données de capteurs.

```
water_level_predictions,
  field_id=<uuid>,
  planting_id=<uuid>,
  model_version=<string>,
  status=<string>
  water_level=<float>,
  deficit=<float>,
  irrigation_needed=<boolean>
  <timestamp>
```

#### Tags (indexés)
- `field_id` : UUID du champ
- `planting_id` : UUID de la plantation
- `model_version` : Version du modèle de prédiction
- `status` : État d'humidité (dry, normal, wet, flooded)

#### Fields (non indexés)
- `water_level` : Taux d'humidité prédit (%)
- `deficit` : Déficit hydrique estimé (mm)
- `irrigation_needed` : Indicateur de besoin d'irrigation

### Measurement: fertility_predictions

Stocke les prédictions de fertilité basées sur les données de capteurs.

```
fertility_predictions,
  field_id=<uuid>,
  planting_id=<uuid>,
  model_version=<string>
  fertility_score=<float>,
  nitrogen_level=<string>,
  phosphorus_level=<string>,
  potassium_level=<string>,
  ph_level=<string>
  <timestamp>
```

#### Tags (indexés)
- `field_id` : UUID du champ
- `planting_id` : UUID de la plantation
- `model_version` : Version du modèle de prédiction

#### Fields (non indexés)
- `fertility_score` : Score de fertilité (0.0 à 10.0)
- `nitrogen_level` : Niveau d'azote (low, adequate, high)
- `phosphorus_level` : Niveau de phosphore (low, adequate, high)
- `potassium_level` : Niveau de potassium (low, adequate, high)
- `ph_level` : Niveau de pH (acidic, neutral, alkaline)

### Measurement: air_quality_predictions

Stocke les prédictions de qualité de l'air basées sur les données de capteurs.

```
air_quality_predictions,
  field_id=<uuid>,
  model_version=<string>,
  category=<string>
  air_quality_index=<float>,
  co2_level=<string>,
  pm25_level=<string>,
  pm10_level=<string>
  <timestamp>
```

#### Tags (indexés)
- `field_id` : UUID du champ
- `model_version` : Version du modèle de prédiction
- `category` : Catégorie de qualité de l'air (good, moderate, unhealthy, etc.)

#### Fields (non indexés)
- `air_quality_index` : Indice de qualité de l'air
- `co2_level` : Niveau de CO2 (normal, elevated, high)
- `pm25_level` : Niveau de PM2.5 (low, moderate, high)
- `pm10_level` : Niveau de PM10 (low, moderate, high)

### Measurement: production_date_predictions

Stocke les prédictions de date de production basées sur les données de capteurs.

```
production_date_predictions,
  field_id=<uuid>,
  planting_id=<uuid>,
  crop_type=<string>,
  model_version=<string>
  days_to_harvest=<integer>,
  confidence=<float>
  <timestamp>
```

#### Tags (indexés)
- `field_id` : UUID du champ
- `planting_id` : UUID de la plantation
- `crop_type` : Type de culture
- `model_version` : Version du modèle de prédiction

#### Fields (non indexés)
- `days_to_harvest` : Jours estimés avant la récolte
- `confidence` : Niveau de confiance de la prédiction (0.0 à 1.0)

### Measurement: production_rate_predictions

Stocke les prédictions de taux de production basées sur les données de capteurs.

```
production_rate_predictions,
  field_id=<uuid>,
  planting_id=<uuid>,
  crop_type=<string>,
  model_version=<string>
  estimated_yield_per_ha=<float>,
  confidence=<float>
  <timestamp>
```

#### Tags (indexés)
- `field_id` : UUID du champ
- `planting_id` : UUID de la plantation
- `crop_type` : Type de culture
- `model_version` : Version du modèle de prédiction

#### Fields (non indexés)
- `estimated_yield_per_ha` : Rendement estimé par hectare (tonnes)
- `confidence` : Niveau de confiance de la prédiction (0.0 à 1.0)

### Measurement: disaster_risk_predictions

Stocke les prédictions de risques de catastrophes basées sur les données de capteurs.

```
disaster_risk_predictions,
  field_id=<uuid>,
  disaster_type=<string>,
  risk_level=<string>,
  model_version=<string>
  probability=<float>,
  severity=<float>
  <timestamp>
```

#### Tags (indexés)
- `field_id` : UUID du champ
- `disaster_type` : Type de catastrophe (flood, drought, frost, etc.)
- `risk_level` : Niveau de risque (low, medium, high, critical)
- `model_version` : Version du modèle de prédiction

#### Fields (non indexés)
- `probability` : Probabilité du risque (0.0 à 1.0)
- `severity` : Sévérité potentielle (0.0 à 1.0)

### Measurement: system_metrics

Stocke les métriques système pour la surveillance des performances.

```
system_metrics,
  service=<string>,
  host=<string>,
  region=<string>
  cpu_usage=<float>,
  memory_usage=<float>,
  disk_usage=<float>,
  api_requests=<integer>,
  prediction_count=<integer>,
  error_count=<integer>,
  response_time_ms=<float>
  <timestamp>
```

#### Tags (indexés)
- `service` : Nom du service (api, disease_detection, etc.)
- `host` : Nom de l'hôte ou conteneur
- `region` : Région de déploiement

#### Fields (non indexés)
- `cpu_usage` : Utilisation CPU (%)
- `memory_usage` : Utilisation mémoire (%)
- `disk_usage` : Utilisation disque (%)
- `api_requests` : Nombre de requêtes API
- `prediction_count` : Nombre de prédictions effectuées
- `error_count` : Nombre d'erreurs
- `response_time_ms` : Temps de réponse moyen (ms)

## Continuous Queries

InfluxDB permet de définir des requêtes continues qui s'exécutent automatiquement à intervalles réguliers pour agréger les données. Voici quelques exemples :

### Agrégation horaire des données de capteurs

```sql
CREATE CONTINUOUS QUERY "cq_hourly_sensor_data" ON "agri_monitor"
BEGIN
  SELECT 
    mean("soil_moisture") AS "mean_soil_moisture",
    mean("air_humidity") AS "mean_air_humidity",
    mean("temperature") AS "mean_temperature",
    mean("soil_ph") AS "mean_soil_ph",
    min("soil_moisture") AS "min_soil_moisture",
    max("temperature") AS "max_temperature"
  INTO "hourly_sensor_data"
  FROM "sensor_data"
  GROUP BY time(1h), "field_id", "sensor_type"
END
```

### Agrégation journalière des données de capteurs

```sql
CREATE CONTINUOUS QUERY "cq_daily_sensor_data" ON "agri_monitor"
BEGIN
  SELECT 
    mean("soil_moisture") AS "mean_soil_moisture",
    mean("air_humidity") AS "mean_air_humidity",
    mean("temperature") AS "mean_temperature",
    mean("soil_ph") AS "mean_soil_ph",
    min("soil_moisture") AS "min_soil_moisture",
    max("temperature") AS "max_temperature",
    sum("rainfall") AS "total_rainfall"
  INTO "daily_sensor_data"
  FROM "sensor_data"
  GROUP BY time(1d), "field_id", "sensor_type"
END
```

## Politiques de Rétention

- Données brutes des capteurs : 6 mois
- Données agrégées horaires : 2 ans
- Données agrégées journalières : 10 ans
- Prédictions : 2 ans
- Métriques système : 3 mois

## Intégration avec PostgreSQL et MongoDB

Les données de séries temporelles stockées dans InfluxDB sont liées aux données structurées de PostgreSQL et aux données non structurées de MongoDB via les identifiants communs :

- `field_id` : Référence aux champs dans PostgreSQL
- `planting_id` : Référence aux plantations dans PostgreSQL

Ces références permettent de corréler les mesures de capteurs avec les informations sur les champs, les cultures, et les analyses d'images stockées dans les autres bases de données.

