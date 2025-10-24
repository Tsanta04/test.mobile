# Modèle Conceptuel de Données - PostgreSQL

Ce document décrit le modèle conceptuel de données pour la base de données PostgreSQL utilisée dans le système de surveillance agricole.

## Entités et Relations

### Utilisateurs et Authentification

#### Table: users
- **id**: UUID (PK)
- **username**: VARCHAR(50) NOT NULL UNIQUE
- **email**: VARCHAR(100) NOT NULL UNIQUE
- **password_hash**: VARCHAR(255) NOT NULL
- **role**: VARCHAR(20) NOT NULL
- **created_at**: TIMESTAMP NOT NULL
- **updated_at**: TIMESTAMP NOT NULL

#### Table: user_sessions
- **id**: UUID (PK)
- **user_id**: UUID (FK -> users.id)
- **token**: VARCHAR(255) NOT NULL UNIQUE
- **expires_at**: TIMESTAMP NOT NULL
- **created_at**: TIMESTAMP NOT NULL

### Données Agricoles

#### Table: farms
- **id**: UUID (PK)
- **name**: VARCHAR(100) NOT NULL
- **owner_id**: UUID (FK -> users.id)
- **location**: GEOGRAPHY(POINT) NOT NULL
- **total_area_ha**: DECIMAL(10,2) NOT NULL
- **created_at**: TIMESTAMP NOT NULL
- **updated_at**: TIMESTAMP NOT NULL

#### Table: fields
- **id**: UUID (PK)
- **farm_id**: UUID (FK -> farms.id)
- **name**: VARCHAR(100) NOT NULL
- **area_ha**: DECIMAL(10,2) NOT NULL
- **boundary**: GEOGRAPHY(POLYGON) NOT NULL
- **soil_type**: VARCHAR(50)
- **created_at**: TIMESTAMP NOT NULL
- **updated_at**: TIMESTAMP NOT NULL

#### Table: crops
- **id**: UUID (PK)
- **name**: VARCHAR(100) NOT NULL
- **scientific_name**: VARCHAR(100)
- **crop_type**: VARCHAR(50) NOT NULL
- **growing_season**: VARCHAR(50)
- **created_at**: TIMESTAMP NOT NULL
- **updated_at**: TIMESTAMP NOT NULL

#### Table: plantings
- **id**: UUID (PK)
- **field_id**: UUID (FK -> fields.id)
- **crop_id**: UUID (FK -> crops.id)
- **planting_date**: DATE NOT NULL
- **expected_harvest_date**: DATE
- **status**: VARCHAR(20) NOT NULL
- **variety**: VARCHAR(100)
- **created_at**: TIMESTAMP NOT NULL
- **updated_at**: TIMESTAMP NOT NULL

### Données Économiques

#### Table: economic_data
- **id**: UUID (PK)
- **planting_id**: UUID (FK -> plantings.id)
- **fertilizer_cost_per_ha**: DECIMAL(10,2)
- **water_cost_per_ha**: DECIMAL(10,2)
- **labor_cost_per_ha**: DECIMAL(10,2)
- **other_costs_per_ha**: DECIMAL(10,2)
- **expected_yield_per_ha**: DECIMAL(10,2)
- **expected_price_per_ton**: DECIMAL(10,2)
- **created_at**: TIMESTAMP NOT NULL
- **updated_at**: TIMESTAMP NOT NULL

#### Table: market_prices
- **id**: UUID (PK)
- **crop_id**: UUID (FK -> crops.id)
- **price_date**: DATE NOT NULL
- **price_per_ton**: DECIMAL(10,2) NOT NULL
- **market_location**: VARCHAR(100)
- **created_at**: TIMESTAMP NOT NULL

### Opérations Agricoles

#### Table: treatments
- **id**: UUID (PK)
- **planting_id**: UUID (FK -> plantings.id)
- **treatment_type**: VARCHAR(50) NOT NULL
- **treatment_date**: DATE NOT NULL
- **product_name**: VARCHAR(100)
- **application_rate**: DECIMAL(10,2)
- **application_unit**: VARCHAR(20)
- **cost**: DECIMAL(10,2)
- **notes**: TEXT
- **created_at**: TIMESTAMP NOT NULL
- **updated_at**: TIMESTAMP NOT NULL

#### Table: irrigation_events
- **id**: UUID (PK)
- **field_id**: UUID (FK -> fields.id)
- **irrigation_date**: DATE NOT NULL
- **water_amount_mm**: DECIMAL(10,2) NOT NULL
- **irrigation_type**: VARCHAR(50)
- **duration_hours**: DECIMAL(5,2)
- **cost**: DECIMAL(10,2)
- **created_at**: TIMESTAMP NOT NULL

#### Table: harvests
- **id**: UUID (PK)
- **planting_id**: UUID (FK -> plantings.id)
- **harvest_date**: DATE NOT NULL
- **yield_tons**: DECIMAL(10,2) NOT NULL
- **quality_grade**: VARCHAR(20)
- **notes**: TEXT
- **created_at**: TIMESTAMP NOT NULL
- **updated_at**: TIMESTAMP NOT NULL

### Configuration et Paramètres

#### Table: sensor_configs
- **id**: UUID (PK)
- **field_id**: UUID (FK -> fields.id)
- **sensor_type**: VARCHAR(50) NOT NULL
- **sensor_id**: VARCHAR(100) NOT NULL
- **location**: GEOGRAPHY(POINT)
- **installation_date**: DATE NOT NULL
- **calibration_date**: DATE
- **status**: VARCHAR(20) NOT NULL
- **created_at**: TIMESTAMP NOT NULL
- **updated_at**: TIMESTAMP NOT NULL

#### Table: alert_configs
- **id**: UUID (PK)
- **user_id**: UUID (FK -> users.id)
- **alert_type**: VARCHAR(50) NOT NULL
- **field_id**: UUID (FK -> fields.id) NULL
- **crop_id**: UUID (FK -> crops.id) NULL
- **threshold_value**: DECIMAL(10,2)
- **threshold_unit**: VARCHAR(20)
- **notification_method**: VARCHAR(50) NOT NULL
- **is_active**: BOOLEAN NOT NULL DEFAULT TRUE
- **created_at**: TIMESTAMP NOT NULL
- **updated_at**: TIMESTAMP NOT NULL

### Prédictions et Analyses

#### Table: profitability_analyses
- **id**: UUID (PK)
- **planting_id**: UUID (FK -> plantings.id)
- **analysis_date**: DATE NOT NULL
- **predicted_yield_per_ha**: DECIMAL(10,2)
- **predicted_price_per_ton**: DECIMAL(10,2)
- **estimated_revenue_per_ha**: DECIMAL(10,2)
- **estimated_costs_per_ha**: DECIMAL(10,2)
- **estimated_profit_per_ha**: DECIMAL(10,2)
- **roi**: DECIMAL(5,2)
- **confidence_level**: DECIMAL(5,2)
- **created_at**: TIMESTAMP NOT NULL

#### Table: production_date_predictions
- **id**: UUID (PK)
- **planting_id**: UUID (FK -> plantings.id)
- **prediction_date**: DATE NOT NULL
- **estimated_harvest_date**: DATE NOT NULL
- **days_to_harvest**: INTEGER
- **confidence_level**: DECIMAL(5,2)
- **prediction_method**: VARCHAR(50)
- **created_at**: TIMESTAMP NOT NULL

#### Table: production_rate_predictions
- **id**: UUID (PK)
- **planting_id**: UUID (FK -> plantings.id)
- **prediction_date**: DATE NOT NULL
- **estimated_yield_per_ha**: DECIMAL(10,2) NOT NULL
- **confidence_level**: DECIMAL(5,2)
- **prediction_method**: VARCHAR(50)
- **created_at**: TIMESTAMP NOT NULL

## Diagramme des Relations

```
users 1--* user_sessions
users 1--* farms
users 1--* alert_configs

farms 1--* fields
fields 1--* plantings
fields 1--* sensor_configs
fields 1--* irrigation_events

crops 1--* plantings
crops 1--* market_prices

plantings 1--1 economic_data
plantings 1--* treatments
plantings 1--* harvests
plantings 1--* profitability_analyses
plantings 1--* production_date_predictions
plantings 1--* production_rate_predictions

fields 0--* alert_configs
crops 0--* alert_configs
```

## Indexes

- Index sur `users(email)`
- Index spatial sur `farms(location)`
- Index spatial sur `fields(boundary)`
- Index sur `plantings(planting_date)`
- Index sur `plantings(expected_harvest_date)`
- Index sur `market_prices(price_date, crop_id)`
- Index sur `treatments(treatment_date, planting_id)`
- Index sur `harvests(harvest_date, planting_id)`
- Index sur `sensor_configs(field_id, sensor_type)`

