# Agricultural Monitoring AI System

Ce projet implémente un système d'intelligence artificielle pour la surveillance agricole, développé en Rust. Le système fournit des prédictions pour divers aspects de l'agriculture, en utilisant à la fois des données brutes de capteurs et des images.

## Fonctionnalités

Le système offre des prédictions pour les aspects suivants de l'agriculture:

1. **Détection de maladies des plantes**
   - Par données brutes (capteurs)
   - Par images (RGB ou multispectrales)

2. **Taux d'humidité/eau dans le sol**
   - Par données brutes (capteurs)
   - Par images (satellites/multispectrales)

3. **Fertilité du sol**
   - Par données brutes (capteurs)
   - Par images (satellites)

4. **Qualité de l'air environnant**
   - Par données brutes (capteurs)
   - Par images (satellites)

5. **Analyse de rentabilité économique**
   - Par données brutes (rendements, coûts)
   - Par images (estimation de biomasse)

6. **Date de récolte/production**
   - Par données brutes (température cumulative, humidité)
   - Par images (stade de maturité)

7. **Taux de rendement/production**
   - Par données brutes (humidité, température, fertilité)
   - Par images (comptage/densité)

8. **Détection de catastrophes naturelles**
   - Par données brutes (pluie, vent, température)
   - Par images (satellites avant/après)

## Architecture du projet

Le projet est organisé en un workspace Cargo avec plusieurs crates:

- **common**: Fonctionnalités partagées entre tous les modules
- **disease_detection**: Détection de maladies des plantes
- **water_level**: Analyse du taux d'humidité/eau dans le sol
- **fertility**: Analyse de la fertilité du sol
- **air_quality**: Analyse de la qualité de l'air
- **profitability**: Analyse de rentabilité économique
- **production_date**: Prédiction de la date de récolte
- **production_rate**: Prédiction du taux de rendement
- **disaster_detection**: Détection de catastrophes naturelles
- **api**: API web pour accéder aux fonctionnalités

## Technologies utilisées

- **Rust**: Langage de programmation principal
- **Cargo**: Gestionnaire de paquets et système de build
- **Actix-web**: Framework web pour l'API
- **Diesel**: ORM pour l'accès à la base de données PostgreSQL
- **InfluxDB**: Base de données pour les séries temporelles (données de capteurs)
- **MongoDB**: Base de données pour les images et métadonnées
- **tch-rs**: Bindings PyTorch pour les modèles CNN
- **linfa**: Framework de machine learning pour les modèles classiques
- **burn**: Framework de deep learning

## Installation

### Prérequis

- Rust et Cargo (version 1.70.0 ou supérieure)
- PostgreSQL
- InfluxDB
- MongoDB

### Installation

1. Cloner le dépôt:
   ```bash
   git clone https://github.com/votre-utilisateur/agri_monitor_ai.git
   cd agri_monitor_ai
   ```

2. Compiler le projet:
   ```bash
   cargo build --release
   ```

3. Configurer les variables d'environnement (créer un fichier `.env` à la racine du projet):
   ```
   DATABASE_URL=postgres://utilisateur:mot_de_passe@localhost/agri_monitor
   INFLUXDB_URL=http://localhost:8086
   INFLUXDB_ORG=agri_monitor
   INFLUXDB_BUCKET=sensor_data
   INFLUXDB_TOKEN=votre_token
   MONGODB_URI=mongodb://localhost:27017
   MONGODB_DB=agri_monitor
   MODELS_DIR=./models
   ```

4. Lancer l'API:
   ```bash
   cargo run --release -p api
   ```

## Utilisation de l'API

L'API est accessible à l'adresse `http://localhost:8080/api/v1/`.

### Endpoints

- **Détection de maladies**:
  - `POST /api/v1/disease-detection/sensor`: Détection par données de capteurs
  - `POST /api/v1/disease-detection/image`: Détection par image

- **Taux d'eau**:
  - `POST /api/v1/water-level/sensor`: Analyse par données de capteurs
  - `POST /api/v1/water-level/image`: Analyse par image

- **Fertilité**:
  - `POST /api/v1/fertility/sensor`: Analyse par données de capteurs
  - `POST /api/v1/fertility/image`: Analyse par image

- **Qualité d'air**:
  - `POST /api/v1/air-quality/sensor`: Analyse par données de capteurs
  - `POST /api/v1/air-quality/image`: Analyse par image

- **Rentabilité**:
  - `POST /api/v1/profitability/data`: Analyse par données économiques
  - `POST /api/v1/profitability/image`: Analyse par image

- **Date de production**:
  - `POST /api/v1/production-date/sensor`: Prédiction par données de capteurs
  - `POST /api/v1/production-date/image`: Prédiction par image

- **Taux de production**:
  - `POST /api/v1/production-rate/sensor`: Prédiction par données de capteurs
  - `POST /api/v1/production-rate/image`: Prédiction par image

- **Catastrophes naturelles**:
  - `POST /api/v1/disaster-detection/sensor`: Détection par données de capteurs
  - `POST /api/v1/disaster-detection/image`: Détection par image

## Exemples d'utilisation

### Détection de maladie par données de capteurs

```bash
curl -X POST http://localhost:8080/api/v1/disease-detection/sensor \
  -H "Content-Type: application/json" \
  -d '{
    "latitude": 48.8566,
    "longitude": 2.3522,
    "soil_moisture": 35.5,
    "air_humidity": 65.2,
    "temperature": 22.3,
    "soil_ph": 6.8,
    "nitrogen": 120.5,
    "phosphorus": 45.2,
    "potassium": 200.1
  }'
```

### Détection de maladie par image

```bash
curl -X POST http://localhost:8080/api/v1/disease-detection/image \
  -H "Content-Type: application/json" \
  -d '{
    "latitude": 48.8566,
    "longitude": 2.3522,
    "image_data": "base64_encoded_image_data",
    "image_type": "RGB"
  }'
```

## Développement

### Structure du code

- `crates/common/`: Fonctionnalités partagées
  - `src/data/`: Structures de données communes
  - `src/db/`: Connexions aux bases de données
  - `src/error/`: Types d'erreurs
  - `src/models/`: Interfaces de modèles
  - `src/utils/`: Utilitaires

- `crates/disease_detection/`: Module de détection de maladies
  - `src/models/`: Modèles de détection
  - `src/types/`: Types spécifiques
  - `src/utils/`: Utilitaires

- `api/`: API web
  - `src/handlers/`: Gestionnaires de requêtes
  - `src/routes/`: Configuration des routes
  - `src/models/`: Modèles d'API
  - `src/config/`: Configuration

### Ajout d'un nouveau modèle

1. Créer une nouvelle structure de modèle dans le module approprié
2. Implémenter les traits `SensorDataModel` ou `ImageModel` selon le cas
3. Ajouter les endpoints correspondants dans l'API

## Licence

Ce projet est sous licence MIT. Voir le fichier LICENSE pour plus de détails.

## Auteurs

- Tsanta - [atnastsanta@gmail.com](mailto:atnastsanta@gmail.com)

