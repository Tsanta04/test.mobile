# Agricultural Monitoring Backend

Ce projet est un backend développé avec Actix-web et Diesel ORM en Rust pour une application de surveillance agricole. Il fournit une API RESTful pour gérer les utilisateurs, les terrains, les capteurs, les alertes et les données de surveillance.

## Structure du Projet

```
server/
├── migrations/
│   └── 2023-08-26-000000_create_tables/
│       ├── up.sql
│       └── down.sql
├── src/
│   ├── dto/
│   │   ├── alert_dto.rs
│   │   ├── ground_dto.rs
│   │   ├── state_dto.rs
│   │   ├── user_dto.rs
│   │   └── mod.rs
│   ├── handlers/
│   │   ├── alert_handler.rs
│   │   ├── ground_handler.rs
│   │   ├── state_handler.rs
│   │   ├── user_handler.rs
│   │   └── mod.rs
│   ├── models/
│   │   ├── alert.rs
│   │   ├── discussion.rs
│   │   ├── ground.rs
│   │   ├── message.rs
│   │   ├── participant.rs
│   │   ├── state.rs
│   │   ├── user.rs
│   │   └── mod.rs
│   ├── services/
│   │   ├── alert_service.rs
│   │   ├── discussion_service.rs
│   │   ├── ground_service.rs
│   │   ├── state_service.rs
│   │   ├── user_service.rs
│   │   └── mod.rs
│   ├── error.rs
│   ├── schema.rs
│   └── main.rs
├── Cargo.toml
├── diesel.toml
└── .env
```

## Prérequis

- Rust (édition 2021 ou supérieure)
- PostgreSQL
- Diesel CLI (`cargo install diesel_cli --no-default-features --features postgres`)

## Configuration

1. Créez une base de données PostgreSQL
2. Configurez le fichier `.env` avec vos informations de connexion:
   ```
   DATABASE_URL=postgres://username:password@localhost/database_name
   RUST_LOG=debug
   JWT_SECRET=your_secret_key_here
   ```

## Installation et Exécution

1. Clonez le dépôt
2. Exécutez les migrations de base de données:
   ```
   diesel migration run
   ```
3. Compilez et exécutez le serveur:
   ```
   cargo run
   ```
4. Le serveur sera accessible à l'adresse `http://localhost:8080`

## Fonctionnalités

### Authentification
- Inscription et connexion des utilisateurs
- Authentification basée sur JWT
- Gestion des rôles (vendeur, acheteur, fournisseur)

### Gestion des Terrains
- CRUD complet pour les terrains agricoles
- Filtrage par utilisateur, type de culture, emplacement
- Association avec des packs de capteurs

### Surveillance des États
- Suivi des métriques (température, humidité, santé, etc.)
- Historique des données par plage de dates
- Dernières données par pack de capteurs

### Système d'Alertes
- Alertes basées sur différents types (santé, production, etc.)
- Niveaux d'urgence (faible, moyen, élevé, urgent)
- Marquage des alertes comme vues
- Recommandations associées

### Système de Discussion
- Discussions entre utilisateurs
- Messagerie
- Gestion des participants

## API Endpoints

### Utilisateurs
- `GET /api/users` - Liste tous les utilisateurs
- `GET /api/users/{id}` - Obtient un utilisateur par ID
- `POST /api/users` - Crée un nouvel utilisateur
- `PUT /api/users/{id}` - Met à jour un utilisateur
- `DELETE /api/users/{id}` - Supprime un utilisateur
- `POST /api/login` - Authentifie un utilisateur

### Terrains
- `GET /api/grounds` - Liste tous les terrains
- `GET /api/grounds/{id}` - Obtient un terrain par ID
- `POST /api/grounds` - Crée un nouveau terrain
- `PUT /api/grounds/{id}` - Met à jour un terrain
- `DELETE /api/grounds/{id}` - Supprime un terrain
- `GET /api/grounds/user/{user_id}` - Liste les terrains d'un utilisateur
- `GET /api/grounds/culture/{culture_type_id}` - Liste les terrains par type de culture
- `GET /api/grounds/location/{location_id}` - Liste les terrains par emplacement
- `GET /api/grounds/pack/{pack_id}` - Liste les terrains par pack de capteurs

### États
- `GET /api/states` - Liste tous les états
- `GET /api/states/{id}` - Obtient un état par ID
- `POST /api/states` - Crée un nouvel état
- `PUT /api/states/{id}` - Met à jour un état
- `DELETE /api/states/{id}` - Supprime un état
- `GET /api/states/pack/{pack_id}` - Liste les états d'un pack de capteurs
- `GET /api/states/pack/{pack_id}/latest` - Obtient le dernier état d'un pack
- `POST /api/states/pack/{pack_id}/date-range` - Liste les états dans une plage de dates

### Alertes
- `GET /api/alerts` - Liste toutes les alertes
- `GET /api/alerts/{id}` - Obtient une alerte par ID
- `POST /api/alerts` - Crée une nouvelle alerte
- `PUT /api/alerts/{id}` - Met à jour une alerte
- `DELETE /api/alerts/{id}` - Supprime une alerte
- `GET /api/alerts/state/{state_id}` - Liste les alertes d'un état
- `GET /api/alerts/type/{alert_type}` - Liste les alertes par type
- `GET /api/alerts/unseen` - Liste les alertes non vues
- `PUT /api/alerts/{id}/seen` - Marque une alerte comme vue

## Modèle de Données

Le système est basé sur les entités suivantes:

- **Users**: Gestion des utilisateurs et authentification
- **Person**: Informations personnelles liées aux utilisateurs
- **Culture_type**: Types de cultures agricoles
- **Location**: Emplacements géographiques
- **Sensor_type**: Types de capteurs
- **Sensor**: Capteurs individuels
- **Sensor_pack**: Packs de capteurs
- **Pack**: Association entre capteurs et packs
- **Ground**: Terrains agricoles
- **Discussion**: Système de discussion
- **Message**: Messages de communication
- **Participant**: Participants aux discussions
- **State**: Métriques de surveillance
- **Alert**: Système d'alertes
- **Planning**: Gestion des plannings

## Technologies Utilisées

- **Actix-web**: Framework web performant en Rust
- **Diesel**: ORM pour Rust avec support PostgreSQL
- **Serde**: Sérialisation/désérialisation
- **Jsonwebtoken**: Gestion des JWT
- **Argon2**: Hachage sécurisé des mots de passe
- **Chrono**: Gestion des dates et heures
- **Dotenv**: Gestion des variables d'environnement
- **Env_logger**: Journalisation

## Sécurité

- Mots de passe hachés avec Argon2
- Authentification par JWT
- Contrôle d'accès basé sur les rôles
- Validation des entrées
- Configuration CORS

