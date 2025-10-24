# Système de Surveillance Agricole avec IA

Ce projet implémente un système de surveillance agricole basé sur l'intelligence artificielle, développé en Rust. Il utilise des modèles de machine learning pour analyser les données de capteurs et les images afin de fournir des prédictions sur différents aspects de l'agriculture.

## Fonctionnalités

Le système offre plusieurs modules de prédiction, chacun capable d'analyser les données brutes des capteurs et les images :

1. **Détection de maladies** - Identifie les maladies des plantes
2. **Taux d'eau** - Surveille l'humidité du sol et fournit des recommandations d'arrosage
3. **Fertilité** - Analyse la fertilité du sol
4. **Qualité d'air** - Surveille la qualité de l'air environnant
5. **Analyse de rentabilité** - Évalue la rentabilité économique
6. **Date de production** - Prédit les dates de récolte
7. **Taux de production** - Estime les rendements
8. **Détection de catastrophes naturelles** - Alerte sur les risques environnementaux

## Architecture

Le projet est organisé en plusieurs crates Rust :

- **common** - Fonctionnalités partagées entre les modules
- **disease_detection** - Module de détection de maladies
- **water_level** - Module de prédiction du taux d'eau et recommandation d'arrosage
- **api** - Interface API web pour accéder aux fonctionnalités

### Technologies utilisées

- **Rust** - Langage de programmation principal
- **Cargo** - Gestionnaire de paquets et système de build
- **Actix-web** - Framework web pour l'API
- **Diesel** - ORM pour l'accès aux bases de données
- **Docker** - Conteneurisation pour le déploiement

### Bases de données

Le système utilise une architecture polyglotte avec trois bases de données spécialisées :

- **PostgreSQL** - Données structurées (utilisateurs, fermes, champs, cultures)
- **MongoDB** - Données non structurées (images, cartes de distribution)
- **InfluxDB** - Séries temporelles (données de capteurs, prédictions)

## Installation

### Prérequis

- Rust (version 1.70.0 ou supérieure)
- Docker et Docker Compose
- Git

### Étapes d'installation

1. Cloner le dépôt :
   ```bash
   git clone https://github.com/votre-utilisateur/agri_monitor_ai.git
   cd agri_monitor_ai
   ```

2. Configurer les variables d'environnement :
   ```bash
   cp .env.example .env
   # Modifier les valeurs dans .env selon votre environnement
   ```

3. Exécuter le script de démarrage :
   ```bash
   ./run.sh
   ```

## Utilisation de l'API

L'API est accessible à l'adresse `http://localhost:8080` par défaut.

### Endpoints disponibles

#### Détection de maladies
- `POST /api/v1/disease-detection/predict/sensor` - Prédire les maladies à partir de données de capteurs
- `POST /api/v1/disease-detection/predict/image` - Prédire les maladies à partir d'images

#### Taux d'eau
- `POST /api/v1/water-level/predict/sensor` - Prédire le taux d'eau à partir de données de capteurs
- `POST /api/v1/water-level/predict/image` - Prédire le taux d'eau à partir d'images
- `GET /api/v1/water-level/history/{field_id}` - Historique des prédictions pour un champ
- `GET /api/v1/water-level/recommendations/{field_id}` - Recommandations d'arrosage pour un champ

#### Autres modules (à implémenter)
- `POST /api/v1/fertility/predict/sensor` - Prédire la fertilité du sol
- `POST /api/v1/air-quality/predict/sensor` - Prédire la qualité de l'air
- `POST /api/v1/profitability/predict/sensor` - Analyser la rentabilité
- `POST /api/v1/production-date/predict/sensor` - Prédire la date de production
- `POST /api/v1/production-rate/predict/sensor` - Prédire le taux de production
- `POST /api/v1/disaster-detection/predict/sensor` - Détecter les catastrophes naturelles

### Exemple de requête pour le taux d'eau

```bash
curl -X POST http://localhost:8080/api/v1/water-level/predict/sensor \
  -H "Content-Type: application/json" \
  -d '{
    "sensor_data": {
      "timestamp": "2023-08-17T14:30:00Z",
      "location": {
        "latitude": 48.8566,
        "longitude": 2.3522
      },
      "values": {
        "soil_moisture": "45",
        "air_humidity": "65",
        "temperature": "25",
        "rainfall": "0"
      }
    },
    "weather_data": {
      "temperature": 25.0,
      "air_humidity": 0.65,
      "forecast_precipitation_24h": 0.0,
      "wind_speed": 3.0,
      "evapotranspiration": 5.0
    },
    "crop_data": {
      "crop_type": "wheat",
      "growth_stage": "mature",
      "crop_coefficient": 0.8,
      "root_depth_cm": 30.0,
      "critical_moisture_threshold": 0.4
    },
    "field_id": "field_123"
  }'
```

## Structure du projet

```
agri_monitor_ai/
├── api/                    # API web
│   ├── src/
│   │   ├── config.rs       # Configuration de l'API
│   │   ├── handlers/       # Gestionnaires de requêtes
│   │   │   ├── disease_detection/  # Handlers pour la détection de maladies
│   │   │   └── water_level/        # Handlers pour le taux d'eau
│   │   ├── main.rs         # Point d'entrée de l'API
│   │   └── routes.rs       # Configuration des routes
│   └── Cargo.toml
├── crates/
│   ├── common/             # Fonctionnalités partagées
│   │   ├── src/
│   │   │   ├── data/       # Structures de données
│   │   │   ├── db/         # Accès aux bases de données
│   │   │   ├── error.rs    # Gestion des erreurs
│   │   │   ├── models/     # Traits pour les modèles
│   │   │   └── utils/      # Utilitaires
│   │   └── Cargo.toml
│   ├── disease_detection/  # Module de détection de maladies
│   │   ├── src/
│   │   │   ├── models/     # Modèles de ML
│   │   │   ├── types.rs    # Types de données
│   │   │   └── utils.rs    # Utilitaires
│   │   └── Cargo.toml
│   └── water_level/        # Module de taux d'eau
│       ├── src/
│       │   ├── models/     # Modèles de ML
│       │   ├── types.rs    # Types de données
│       │   ├── utils.rs    # Utilitaires
│       │   └── tests/      # Tests unitaires
│       └── Cargo.toml
├── docs/                   # Documentation
│   └── database/           # Modèles conceptuels de données
├── .env                    # Variables d'environnement
├── Cargo.toml              # Configuration du workspace
├── docker-compose.yml      # Configuration Docker Compose
├── Dockerfile              # Configuration Docker
└── README.md               # Documentation
```

## Développement

### Compilation

```bash
cargo build
```

### Tests

```bash
cargo test
```

### Linting

```bash
cargo clippy
```

## Licence

Ce projet est sous licence MIT. Voir le fichier LICENSE pour plus de détails.

## Contributeurs

- Votre Nom - Développeur principal

