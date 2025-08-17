# Documentation des Bases de Données

Ce répertoire contient la documentation des modèles de données utilisés dans le système de surveillance agricole.

## Architecture Polyglotte

Le système utilise une architecture de base de données polyglotte, où chaque type de base de données est choisi pour ses forces spécifiques :

1. **PostgreSQL** - Base de données relationnelle pour les données structurées
2. **MongoDB** - Base de données NoSQL pour les données non structurées (images, cartes)
3. **InfluxDB** - Base de données de séries temporelles pour les données de capteurs

## Fichiers de Documentation

- [**postgres_mcd.md**](./postgres_mcd.md) - Modèle Conceptuel de Données pour PostgreSQL
- [**mongodb_model.md**](./mongodb_model.md) - Modèle de données pour MongoDB
- [**influxdb_model.md**](./influxdb_model.md) - Modèle de données pour InfluxDB

## Résumé des Cas d'Utilisation

### PostgreSQL - Pour les données structurées et relationnelles
- **Données économiques** : Coûts, prix, rendements, analyses de rentabilité
- **Relations entre entités** : Parcelles, cultures, traitements, historique
- **Données transactionnelles** : Opérations agricoles, applications d'engrais, irrigations
- **Données de configuration** : Paramètres des capteurs, calibrations, seuils d'alerte
- **Données utilisateurs** : Comptes, préférences, autorisations

### MongoDB - Pour les données non structurées et visuelles
- **Stockage des images** : Images capturées par drones, caméras fixes et satellites
- **Métadonnées d'images** : Coordonnées GPS, horodatage, conditions de capture
- **Cartes générées** : Cartes de distribution d'humidité, fertilité, rendement
- **Résultats d'analyse visuelle** : Détections de maladies avec bounding boxes, évaluations de dommages
- **Données géospatiales** : Analyses par zone géographique avec indexation spatiale

### InfluxDB - Pour les séries temporelles
- **Données de capteurs** : Mesures à haute fréquence des capteurs (humidité, température, etc.)
- **Prédictions temporelles** : Résultats de prédictions basées sur les données de capteurs
- **Métriques système** : Surveillance des performances du système
- **Agrégations temporelles** : Données agrégées par heure, jour, semaine, etc.

## Intégration entre les Bases de Données

Les trois bases de données sont intégrées via des identifiants communs :

- `field_id` : Identifiant unique des champs agricoles
- `planting_id` : Identifiant unique des plantations

Ces identifiants permettent de lier les données entre les différentes bases de données, offrant une vue complète et cohérente des données agricoles.

## Diagramme Conceptuel

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   PostgreSQL    │     │     MongoDB     │     │    InfluxDB     │
│  (Relationnel)  │     │     (NoSQL)     │     │ (Séries Temp.)  │
└────────┬────────┘     └────────┬────────┘     └────────┬────────┘
         │                       │                       │
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Données        │     │  Images         │     │  Données de     │
│  structurées    │     │  Cartes         │     │  capteurs       │
│  Utilisateurs   │     │  Analyses       │     │  Mesures        │
│  Champs         │     │  visuelles      │     │  temporelles    │
│  Plantations    │     │  Détections     │     │  Agrégations    │
│  Traitements    │     │  géospatiales   │     │  Prédictions    │
└─────────────────┘     └─────────────────┘     └─────────────────┘
         │                       │                       │
         │                       │                       │
         └───────────────┬───────────────┬───────────────┘
                         │               │
                         ▼               ▼
                ┌─────────────────┐    ┌─────────────────┐
                │    field_id     │    │   planting_id   │
                │  (Identifiant   │    │  (Identifiant   │
                │   des champs)   │    │  des plantations│
                └─────────────────┘    └─────────────────┘
```

## Évolution du Modèle de Données

Le modèle de données est conçu pour évoluer avec le système :

- **PostgreSQL** : Les migrations de schéma sont gérées via Diesel ORM
- **MongoDB** : Le schéma flexible permet d'ajouter de nouveaux champs sans migration
- **InfluxDB** : De nouvelles mesures peuvent être ajoutées sans modifier le schéma existant

