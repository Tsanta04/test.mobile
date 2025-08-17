//! Utilitaires pour le module de prédiction de la fertilité du sol
//!
//! Ce module fournit des fonctions utilitaires pour les calculs et transformations
//! liés à la fertilité du sol.

use crate::types::{
    CropFertilityData, FertilityMap, FertilityPrediction, FertilizationPriority,
    FertilizationRecommendation, GeoReference, MapResolution, NutrientLevels, SoilData,
    SoilFertilityStatus,
};
use chrono::Utc;
use common::error::{AgriMonitorError, AgriResult};
use image::{DynamicImage, GenericImageView};
use ndarray::{Array1, Array2};
use std::collections::HashMap;

/// Calcule un score de fertilité à partir des niveaux de nutriments
pub fn calculate_fertility_score(
    ph: f32,
    nutrient_levels: &NutrientLevels,
    crop_data: &CropFertilityData,
) -> AgriResult<f32> {
    // Vérifier que le pH est dans une plage valide
    if ph < 0.0 || ph > 14.0 {
        return Err(AgriMonitorError::InvalidInput(
            "Le pH doit être compris entre 0 et 14".to_string(),
        ));
    }

    // Calculer le score de pH (1.0 si dans la plage optimale, diminue en s'éloignant)
    let (min_ph, max_ph) = crop_data.optimal_ph_range;
    let ph_score = if ph >= min_ph && ph <= max_ph {
        1.0
    } else if ph < min_ph {
        1.0 - ((min_ph - ph) / min_ph).min(1.0)
    } else {
        1.0 - ((ph - max_ph) / (14.0 - max_ph)).min(1.0)
    };

    // Calculer les scores de nutriments (ratio entre niveau actuel et besoin)
    let n_ratio = (nutrient_levels.nitrogen / crop_data.nitrogen_requirement).min(2.0);
    let p_ratio = (nutrient_levels.phosphorus / crop_data.phosphorus_requirement).min(2.0);
    let k_ratio = (nutrient_levels.potassium / crop_data.potassium_requirement).min(2.0);

    // Normaliser les ratios (0.0 à 1.0)
    let n_score = if n_ratio > 1.0 {
        1.0 - (n_ratio - 1.0) * 0.5 // Pénalité légère pour excès
    } else {
        n_ratio
    };

    let p_score = if p_ratio > 1.0 {
        1.0 - (p_ratio - 1.0) * 0.5
    } else {
        p_ratio
    };

    let k_score = if k_ratio > 1.0 {
        1.0 - (k_ratio - 1.0) * 0.5
    } else {
        k_ratio
    };

    // Calculer le score de matière organique si disponible
    let om_score = if let Some(om) = nutrient_levels.organic_matter {
        // La matière organique idéale est généralement entre 3% et 5%
        if om < 1.0 {
            om / 3.0
        } else if om <= 5.0 {
            om / 5.0
        } else {
            1.0
        }
    } else {
        0.8 // Valeur par défaut si non disponible
    };

    // Pondération des facteurs
    let weights = [0.25, 0.2, 0.2, 0.2, 0.15]; // pH, N, P, K, OM
    let scores = [ph_score, n_score, p_score, k_score, om_score];

    // Calculer le score pondéré
    let fertility_score = weights
        .iter()
        .zip(scores.iter())
        .map(|(w, s)| w * s)
        .sum::<f32>();

    Ok(fertility_score)
}

/// Calcule une recommandation de fertilisation à partir d'une prédiction de fertilité
pub fn calculate_fertilization_recommendation(
    prediction: &FertilityPrediction,
    soil_data: &SoilData,
    crop_data: &CropFertilityData,
) -> AgriResult<FertilizationRecommendation> {
    // Calculer les déficits en nutriments
    let n_deficit = calculate_nutrient_deficit(
        prediction.nutrient_levels.nitrogen,
        crop_data.nitrogen_requirement,
    );
    let p_deficit = calculate_nutrient_deficit(
        prediction.nutrient_levels.phosphorus,
        crop_data.phosphorus_requirement,
    );
    let k_deficit = calculate_nutrient_deficit(
        prediction.nutrient_levels.potassium,
        crop_data.potassium_requirement,
    );

    // Créer la carte des déficits
    let mut fertilizer_amounts = HashMap::new();
    fertilizer_amounts.insert("nitrogen".to_string(), n_deficit);
    fertilizer_amounts.insert("phosphorus".to_string(), p_deficit);
    fertilizer_amounts.insert("potassium".to_string(), k_deficit);

    // Ajouter les autres nutriments si nécessaire
    for (nutrient, requirement) in &crop_data.other_requirements {
        if let Some(level) = prediction.nutrient_levels.others.get(nutrient) {
            let deficit = calculate_nutrient_deficit(*level, *requirement);
            if deficit > 0.0 {
                fertilizer_amounts.insert(nutrient.clone(), deficit);
            }
        }
    }

    // Déterminer si la fertilisation est nécessaire
    let fertilization_needed = n_deficit > 5.0 || p_deficit > 5.0 || k_deficit > 5.0;

    // Déterminer la priorité
    let priority = determine_fertilization_priority(prediction, crop_data);

    // Déterminer le type d'engrais recommandé
    let recommended_fertilizer_type = recommend_fertilizer_type(&fertilizer_amounts);

    // Générer des notes
    let notes = generate_fertilization_notes(prediction, soil_data, crop_data);

    // Créer la recommandation
    let recommendation = FertilizationRecommendation {
        fertilization_needed,
        priority,
        fertilizer_amounts,
        recommended_fertilizer_type,
        optimal_time: Some(determine_optimal_fertilization_time(crop_data)),
        notes,
    };

    Ok(recommendation)
}

/// Calcule le déficit en nutriment
fn calculate_nutrient_deficit(current_level: f32, requirement: f32) -> f32 {
    let deficit = requirement - current_level;
    if deficit > 0.0 {
        deficit
    } else {
        0.0
    }
}

/// Détermine la priorité de fertilisation
pub fn determine_fertilization_priority(
    prediction: &FertilityPrediction,
    crop_data: &CropFertilityData,
) -> FertilizationPriority {
    // Vérifier les niveaux critiques
    let n_ratio = prediction.nutrient_levels.nitrogen / crop_data.nitrogen_requirement;
    let p_ratio = prediction.nutrient_levels.phosphorus / crop_data.phosphorus_requirement;
    let k_ratio = prediction.nutrient_levels.potassium / crop_data.potassium_requirement;

    // Déterminer la priorité en fonction des ratios
    if n_ratio < 0.3 || p_ratio < 0.3 || k_ratio < 0.3 {
        FertilizationPriority::High
    } else if n_ratio < 0.6 || p_ratio < 0.6 || k_ratio < 0.6 {
        FertilizationPriority::Medium
    } else if n_ratio < 0.8 || p_ratio < 0.8 || k_ratio < 0.8 {
        FertilizationPriority::Low
    } else {
        FertilizationPriority::None
    }
}

/// Recommande un type d'engrais en fonction des déficits
fn recommend_fertilizer_type(fertilizer_amounts: &HashMap<String, f32>) -> Option<String> {
    let n = *fertilizer_amounts.get("nitrogen").unwrap_or(&0.0);
    let p = *fertilizer_amounts.get("phosphorus").unwrap_or(&0.0);
    let k = *fertilizer_amounts.get("potassium").unwrap_or(&0.0);

    // Déterminer le type d'engrais en fonction des besoins relatifs
    if n > p && n > k && n > 20.0 {
        Some("Engrais riche en azote (ex: urée, nitrate d'ammonium)".to_string())
    } else if p > n && p > k && p > 20.0 {
        Some("Engrais riche en phosphore (ex: superphosphate)".to_string())
    } else if k > n && k > p && k > 20.0 {
        Some("Engrais riche en potassium (ex: sulfate de potassium)".to_string())
    } else if n > 10.0 && p > 10.0 && k > 10.0 {
        Some("Engrais complet NPK équilibré".to_string())
    } else if n > 10.0 && p > 10.0 {
        Some("Engrais NP (ex: phosphate d'ammonium)".to_string())
    } else if n > 10.0 && k > 10.0 {
        Some("Engrais NK".to_string())
    } else if p > 10.0 && k > 10.0 {
        Some("Engrais PK".to_string())
    } else {
        None
    }
}

/// Détermine le moment optimal pour la fertilisation
fn determine_optimal_fertilization_time(crop_data: &CropFertilityData) -> String {
    match crop_data.growth_stage.as_str() {
        "germination" => "Avant semis ou plantation".to_string(),
        "seedling" => "Dès que les plantules sont établies".to_string(),
        "vegetative" => "Pendant la phase de croissance végétative".to_string(),
        "flowering" => "Juste avant la floraison".to_string(),
        "fruiting" => "Après la nouaison des fruits".to_string(),
        "mature" => "Après la récolte, en préparation du prochain cycle".to_string(),
        _ => "Au début de la saison de croissance".to_string(),
    }
}

/// Génère des notes pour la recommandation de fertilisation
fn generate_fertilization_notes(
    prediction: &FertilityPrediction,
    soil_data: &SoilData,
    crop_data: &CropFertilityData,
) -> Vec<String> {
    let mut notes = Vec::new();

    // Note sur le statut général
    notes.push(format!(
        "Statut de fertilité du sol: {}.",
        prediction.status.to_string()
    ));

    // Note sur le pH
    let (min_ph, max_ph) = crop_data.optimal_ph_range;
    if prediction.ph_level < min_ph {
        notes.push(format!(
            "Le pH du sol ({:.1}) est inférieur à l'optimal ({:.1}-{:.1}) pour {}. Envisager un chaulage.",
            prediction.ph_level, min_ph, max_ph, crop_data.crop_type
        ));
    } else if prediction.ph_level > max_ph {
        notes.push(format!(
            "Le pH du sol ({:.1}) est supérieur à l'optimal ({:.1}-{:.1}) pour {}. Envisager l'ajout de soufre ou de matière organique.",
            prediction.ph_level, min_ph, max_ph, crop_data.crop_type
        ));
    }

    // Notes sur les nutriments
    if prediction.nutrient_levels.nitrogen < crop_data.nitrogen_requirement * 0.5 {
        notes.push(format!(
            "Niveau d'azote (N) faible. Priorité à la fertilisation azotée."
        ));
    }

    if prediction.nutrient_levels.phosphorus < crop_data.phosphorus_requirement * 0.5 {
        notes.push(format!(
            "Niveau de phosphore (P) faible. Envisager l'application de phosphates."
        ));
    }

    if prediction.nutrient_levels.potassium < crop_data.potassium_requirement * 0.5 {
        notes.push(format!(
            "Niveau de potassium (K) faible. Envisager l'application de potasse."
        ));
    }

    // Note sur la matière organique
    if let Some(om) = prediction.nutrient_levels.organic_matter {
        if om < 2.0 {
            notes.push(format!(
                "Faible teneur en matière organique ({}%). Envisager l'ajout de compost ou de fumier.",
                om
            ));
        }
    }

    // Note sur le type de sol
    notes.push(format!(
        "Type de sol: {} ({}). Adapter les pratiques de fertilisation en conséquence.",
        soil_data.soil_type, soil_data.soil_texture
    ));

    notes
}

/// Calcule l'indice NDVI à partir des bandes rouge et proche infrarouge
pub fn calculate_ndvi(red_band: &[f32], nir_band: &[f32]) -> AgriResult<Vec<f32>> {
    if red_band.len() != nir_band.len() {
        return Err(AgriMonitorError::InvalidInput(
            "Les bandes rouge et proche infrarouge doivent avoir la même taille".to_string(),
        ));
    }

    let mut ndvi = Vec::with_capacity(red_band.len());

    for (red, nir) in red_band.iter().zip(nir_band.iter()) {
        if *red + *nir == 0.0 {
            ndvi.push(0.0);
        } else {
            let value = (nir - red) / (nir + red);
            ndvi.push(value);
        }
    }

    Ok(ndvi)
}

/// Convertit un indice NDVI en score de fertilité
pub fn ndvi_to_fertility_score(ndvi: f32) -> f32 {
    // L'indice NDVI varie de -1 à 1
    // Convertir en score de fertilité (0 à 1)
    // Un NDVI élevé indique généralement une végétation saine, ce qui peut être corrélé à une bonne fertilité
    if ndvi < -0.5 {
        0.0 // Eau, nuages, etc.
    } else if ndvi < 0.0 {
        0.1 // Sol nu, très faible fertilité
    } else if ndvi < 0.2 {
        0.3 // Végétation clairsemée, fertilité faible
    } else if ndvi < 0.4 {
        0.5 // Végétation modérée, fertilité moyenne
    } else if ndvi < 0.6 {
        0.7 // Végétation dense, bonne fertilité
    } else {
        0.9 // Végétation très dense, excellente fertilité
    }
}

/// Crée une carte de fertilité à partir d'une matrice de scores
pub fn create_fertility_map(
    fertility_matrix: Vec<Vec<f32>>,
    nutrient_matrices: HashMap<String, Vec<Vec<f32>>>,
    timestamp: chrono::DateTime<Utc>,
    geo_reference: GeoReference,
    resolution: MapResolution,
) -> AgriResult<FertilityMap> {
    // Vérifier que la matrice n'est pas vide
    if fertility_matrix.is_empty() || fertility_matrix[0].is_empty() {
        return Err(AgriMonitorError::InvalidInput(
            "La matrice de fertilité ne peut pas être vide".to_string(),
        ));
    }

    // Calculer le score moyen de fertilité
    let mut sum = 0.0;
    let mut count = 0;

    for row in &fertility_matrix {
        for &value in row {
            sum += value;
            count += 1;
        }
    }

    let average_fertility = if count > 0 { sum / count as f32 } else { 0.0 };

    // Déterminer le statut global
    let status = SoilFertilityStatus::from_score(average_fertility);

    // Créer la carte
    let fertility_map = FertilityMap {
        timestamp,
        fertility_matrix,
        nutrient_matrices,
        average_fertility,
        resolution,
        geo_reference,
        status,
    };

    Ok(fertility_map)
}
