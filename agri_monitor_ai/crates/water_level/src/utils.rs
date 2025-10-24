//! Utilitaires pour le module de prédiction du taux d'eau
//!
//! Ce module fournit des fonctions utilitaires pour:
//! - Calculer le déficit hydrique
//! - Générer des recommandations d'arrosage
//! - Traiter les cartes d'humidité

use crate::types::{
    CropData, GeoReference, IrrigationRecommendation, IrrigationUrgency,
    MapResolution, SoilMoistureStatus, WaterLevelMap, WaterLevelPrediction,
    WeatherData,
};
use common::error::{AgriMonitorError, AgriResult};

/// Calcule une recommandation d'arrosage basée sur une prédiction de taux d'humidité
pub fn calculate_irrigation_recommendation(
    prediction: &WaterLevelPrediction,
    crop_data: &CropData,
    weather_data: &WeatherData,
) -> AgriResult<IrrigationRecommendation> {
    // Déterminer si l'irrigation est nécessaire
    let irrigation_needed = needs_irrigation(prediction, crop_data);

    // Calculer la quantité d'eau nécessaire
    let water_amount_mm = if irrigation_needed {
        calculate_water_amount(prediction, crop_data, weather_data)?
    } else {
        0.0
    };

    // Déterminer le niveau d'urgence
    let urgency = determine_urgency(prediction, crop_data, weather_data);

    // Déterminer le meilleur moment pour irriguer
    let optimal_time = determine_optimal_time(weather_data);

    // Générer des recommandations supplémentaires
    let notes = generate_irrigation_notes(prediction, crop_data, weather_data);

    Ok(IrrigationRecommendation {
        irrigation_needed,
        water_amount_mm,
        urgency,
        optimal_time,
        notes,
    })
}

/// Calcule une recommandation d'arrosage basée sur une carte d'humidité
pub fn calculate_irrigation_recommendation_from_map(
    water_map: &WaterLevelMap,
    crop_data: &CropData,
    weather_data: &WeatherData,
) -> AgriResult<IrrigationRecommendation> {
    // Créer une prédiction basée sur la moyenne de la carte
    let prediction = WaterLevelPrediction {
        timestamp: water_map.timestamp,
        water_level: water_map.average_water_level,
        status: water_map.status.clone(),
        confidence: 0.8, // Valeur par défaut pour les cartes
    };

    // Utiliser la fonction existante pour calculer la recommandation
    calculate_irrigation_recommendation(&prediction, crop_data, weather_data)
}

/// Détermine si l'irrigation est nécessaire
fn needs_irrigation(prediction: &WaterLevelPrediction, crop_data: &CropData) -> bool {
    // Si le taux d'humidité est inférieur au seuil critique, l'irrigation est nécessaire
    prediction.water_level < crop_data.critical_moisture_threshold
        || matches!(prediction.status, SoilMoistureStatus::Dry | SoilMoistureStatus::SlightlyDry)
}

/// Calcule la quantité d'eau nécessaire en mm
fn calculate_water_amount(
    prediction: &WaterLevelPrediction,
    crop_data: &CropData,
    weather_data: &WeatherData,
) -> AgriResult<f32> {
    // Calculer le déficit hydrique
    let deficit = calculate_water_deficit(prediction, crop_data)?;

    // Ajuster en fonction des précipitations prévues
    let adjusted_deficit = deficit - weather_data.forecast_precipitation_24h;

    // Ajuster en fonction de l'évapotranspiration
    let et0 = weather_data.evapotranspiration.unwrap_or(5.0); // Valeur par défaut si non disponible
    let etc = et0 * crop_data.crop_coefficient; // Évapotranspiration de la culture

    // Quantité d'eau finale (ne pas irriguer si les précipitations sont suffisantes)
    let water_amount = (adjusted_deficit + etc).max(0.0);

    Ok(water_amount)
}

/// Calcule le déficit hydrique en mm
fn calculate_water_deficit(
    prediction: &WaterLevelPrediction,
    crop_data: &CropData,
) -> AgriResult<f32> {
    // Capacité au champ (considérée comme 0.8 ou 80%)
    let field_capacity = 0.8;

    // Calculer le déficit en pourcentage
    let deficit_percentage = field_capacity - prediction.water_level;

    // Convertir en mm d'eau en fonction de la profondeur des racines
    // Hypothèse: 1% d'humidité = 1mm d'eau par 10cm de sol
    let deficit_mm = deficit_percentage * crop_data.root_depth_cm / 10.0;

    Ok(deficit_mm.max(0.0)) // Ne pas retourner de valeur négative
}

/// Détermine le niveau d'urgence de l'irrigation
fn determine_urgency(
    prediction: &WaterLevelPrediction,
    crop_data: &CropData,
    weather_data: &WeatherData,
) -> IrrigationUrgency {
    // Si le sol est déjà humide ou saturé, pas d'urgence
    if matches!(
        prediction.status,
        SoilMoistureStatus::Optimal | SoilMoistureStatus::Wet | SoilMoistureStatus::Saturated
    ) {
        return IrrigationUrgency::None;
    }

    // Si des précipitations importantes sont prévues, réduire l'urgence
    if weather_data.forecast_precipitation_24h > 10.0 {
        return IrrigationUrgency::Low;
    }

    // Déterminer l'urgence en fonction du taux d'humidité et du seuil critique
    let ratio = prediction.water_level / crop_data.critical_moisture_threshold;

    match ratio {
        r if r < 0.7 => IrrigationUrgency::High,
        r if r < 0.9 => IrrigationUrgency::Medium,
        _ => IrrigationUrgency::Low,
    }
}

/// Détermine le meilleur moment pour irriguer
fn determine_optimal_time(weather_data: &WeatherData) -> Option<String> {
    // Si la température est élevée, privilégier le matin ou le soir
    if weather_data.temperature > 25.0 {
        return Some("early_morning".to_string());
    }

    // Si le vent est fort, privilégier un moment calme
    if let Some(wind_speed) = weather_data.wind_speed {
        if wind_speed > 5.0 {
            return Some("evening".to_string());
        }
    }

    // Par défaut, recommander le matin
    Some("morning".to_string())
}

/// Génère des recommandations supplémentaires pour l'irrigation
fn generate_irrigation_notes(
    prediction: &WaterLevelPrediction,
    crop_data: &CropData,
    weather_data: &WeatherData,
) -> Vec<String> {
    let mut notes = Vec::new();

    // Ajouter des notes en fonction du statut d'humidité
    match prediction.status {
        SoilMoistureStatus::Dry => {
            notes.push("Le sol est très sec, irrigation urgente recommandée.".to_string());
        }
        SoilMoistureStatus::SlightlyDry => {
            notes.push("Le sol est légèrement sec, irrigation recommandée.".to_string());
        }
        SoilMoistureStatus::Optimal => {
            notes.push("Le niveau d'humidité est optimal, pas d'irrigation nécessaire.".to_string());
        }
        SoilMoistureStatus::Wet => {
            notes.push("Le sol est humide, éviter d'irriguer pour le moment.".to_string());
        }
        SoilMoistureStatus::Saturated => {
            notes.push("Le sol est saturé, risque d'engorgement. Vérifier le drainage.".to_string());
        }
    }

    // Ajouter des notes en fonction des prévisions météorologiques
    if weather_data.forecast_precipitation_24h > 5.0 {
        notes.push(format!(
            "Précipitations de {:.1} mm prévues dans les prochaines 24h.",
            weather_data.forecast_precipitation_24h
        ));
    }

    // Ajouter des notes spécifiques à la culture
    notes.push(format!(
        "Stade de croissance actuel: {}. Besoin en eau typique: {:.1} mm/jour.",
        crop_data.growth_stage,
        weather_data
            .evapotranspiration
            .unwrap_or(5.0) * crop_data.crop_coefficient
    ));

    notes
}

/// Calcule la moyenne d'une matrice 2D
pub fn calculate_matrix_average(matrix: &[Vec<f32>]) -> f32 {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0.0;
    }

    let mut sum = 0.0;
    let mut count = 0;

    for row in matrix {
        for &value in row {
            sum += value;
            count += 1;
        }
    }

    if count > 0 {
        sum / count as f32
    } else {
        0.0
    }
}

/// Crée une carte d'humidité à partir d'une matrice de valeurs
pub fn create_water_level_map(
    water_level_matrix: Vec<Vec<f32>>,
    timestamp: chrono::DateTime<chrono::Utc>,
    geo_reference: GeoReference,
    resolution: MapResolution,
) -> AgriResult<WaterLevelMap> {
    // Calculer la moyenne
    let average_water_level = calculate_matrix_average(&water_level_matrix);

    // Déterminer le statut global
    let status = SoilMoistureStatus::from_percentage(average_water_level);

    Ok(WaterLevelMap {
        timestamp,
        water_level_matrix,
        average_water_level,
        resolution,
        geo_reference,
        status,
    })
}

/// Normalise une matrice de valeurs entre 0 et 1
pub fn normalize_matrix(matrix: &mut [Vec<f32>]) {
    if matrix.is_empty() || matrix[0].is_empty() {
        return;
    }

    // Trouver le min et le max
    let mut min_val = f32::MAX;
    let mut max_val = f32::MIN;

    for row in matrix.iter() {
        for &value in row {
            min_val = min_val.min(value);
            max_val = max_val.max(value);
        }
    }

    // Éviter la division par zéro
    if (max_val - min_val).abs() < f32::EPSILON {
        return;
    }

    // Normaliser
    for row in matrix.iter_mut() {
        for value in row.iter_mut() {
            *value = (*value - min_val) / (max_val - min_val);
        }
    }
}

/// Calcule l'indice NDVI (Normalized Difference Vegetation Index) à partir des bandes rouge et proche infrarouge
pub fn calculate_ndvi(red_band: &[f32], nir_band: &[f32]) -> AgriResult<Vec<f32>> {
    if red_band.len() != nir_band.len() {
        return Err(AgriMonitorError::InvalidInput(
            "Les bandes rouge et proche infrarouge doivent avoir la même taille".to_string(),
        ));
    }

    let mut ndvi = Vec::with_capacity(red_band.len());

    for (r, nir) in red_band.iter().zip(nir_band.iter()) {
        let denominator = nir + r;
        let value = if denominator.abs() < f32::EPSILON {
            0.0
        } else {
            (nir - r) / denominator
        };
        ndvi.push(value);
    }

    Ok(ndvi)
}

/// Convertit un indice NDVI en taux d'humidité estimé
pub fn ndvi_to_water_level(ndvi: f32) -> f32 {
    // Relation simplifiée entre NDVI et humidité du sol
    // NDVI élevé (végétation saine) -> humidité adéquate
    // NDVI faible (végétation stressée) -> humidité faible
    // Cette relation est une approximation et devrait être calibrée avec des données réelles
    
    // Normaliser NDVI de [-1,1] à [0,1]
    let normalized_ndvi = (ndvi + 1.0) / 2.0;
    
    // Relation non linéaire entre NDVI et humidité
    // Valeurs typiques: NDVI < 0.2 -> sol nu ou très sec
    //                  NDVI > 0.6 -> végétation saine, bien irriguée
    match normalized_ndvi {
        n if n < 0.2 => 0.1 + n * 0.5,  // Sol très sec à sec
        n if n < 0.4 => 0.2 + (n - 0.2) * 1.5,  // Sec à modéré
        n if n < 0.7 => 0.5 + (n - 0.4) * 1.0,  // Modéré à humide
        _ => 0.8,  // Humide (plafond à 0.8 car NDVI très élevé n'implique pas saturation)
    }
}

