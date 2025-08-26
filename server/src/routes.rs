use crate::handlers::*;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Auth routes
    cfg.service(
        web::scope("/api/auth")
            .service(register)
            .service(login),
    );

    // User routes
    cfg.service(
        web::scope("/api/users")
            .service(get_all_users)
            .service(get_user_by_id)
            .service(update_user)
            .service(delete_user)
            .service(get_current_user),
    );

    // Person routes
    cfg.service(
        web::scope("/api/persons")
            .service(create_person)
            .service(get_all_persons)
            .service(get_person_by_id)
            .service(update_person)
            .service(delete_person)
            .service(get_person_by_user_id),
    );

    // Culture type routes
    cfg.service(
        web::scope("/api/culture-types")
            .service(create_culture_type)
            .service(get_all_culture_types)
            .service(get_culture_type_by_id)
            .service(update_culture_type)
            .service(delete_culture_type),
    );

    // Location routes
    cfg.service(
        web::scope("/api/locations")
            .service(create_location)
            .service(get_all_locations)
            .service(get_location_by_id)
            .service(update_location)
            .service(delete_location),
    );

    // Sensor type routes
    cfg.service(
        web::scope("/api/sensor-types")
            .service(create_sensor_type)
            .service(get_all_sensor_types)
            .service(get_sensor_type_by_id)
            .service(update_sensor_type)
            .service(delete_sensor_type),
    );

    // Sensor routes
    cfg.service(
        web::scope("/api/sensors")
            .service(create_sensor)
            .service(get_all_sensors)
            .service(get_sensor_by_id)
            .service(update_sensor)
            .service(delete_sensor)
            .service(get_sensors_by_type),
    );

    // Sensor pack routes
    cfg.service(
        web::scope("/api/sensor-packs")
            .service(create_sensor_pack)
            .service(get_all_sensor_packs)
            .service(get_sensor_pack_by_id)
            .service(update_sensor_pack)
            .service(delete_sensor_pack),
    );

    // Pack routes
    cfg.service(
        web::scope("/api/packs")
            .service(create_pack)
            .service(get_all_packs)
            .service(get_pack_by_id)
            .service(update_pack)
            .service(delete_pack)
            .service(get_packs_by_sensor_pack_id)
            .service(get_packs_by_sensor_id),
    );

    // Ground routes
    cfg.service(
        web::scope("/api/grounds")
            .service(create_ground)
            .service(get_all_grounds)
            .service(get_ground_by_id)
            .service(update_ground)
            .service(delete_ground)
            .service(get_grounds_by_user_id)
            .service(get_grounds_by_culture_type)
            .service(get_grounds_by_location)
            .service(get_grounds_by_sensor_pack),
    );

    // Discussion routes
    cfg.service(
        web::scope("/api/discussions")
            .service(create_discussion)
            .service(get_all_discussions)
            .service(get_discussion_by_id)
            .service(update_discussion)
            .service(delete_discussion),
    );

    // Message routes
    cfg.service(
        web::scope("/api/messages")
            .service(create_message)
            .service(get_all_messages)
            .service(get_message_by_id)
            .service(update_message)
            .service(delete_message)
            .service(get_messages_by_discussion_id),
    );

    // Participant routes
    cfg.service(
        web::scope("/api/participants")
            .service(create_participant)
            .service(get_all_participants)
            .service(get_participant_by_id)
            .service(update_participant)
            .service(delete_participant)
            .service(get_participants_by_discussion_id)
            .service(get_discussions_by_participant_id),
    );

    // State routes
    cfg.service(
        web::scope("/api/states")
            .service(create_state)
            .service(get_all_states)
            .service(get_state_by_id)
            .service(update_state)
            .service(delete_state)
            .service(get_states_by_pack_id)
            .service(get_latest_state_by_pack_id),
    );

    // Alert routes
    cfg.service(
        web::scope("/api/alerts")
            .service(create_alert)
            .service(get_all_alerts)
            .service(get_alert_by_id)
            .service(update_alert)
            .service(delete_alert)
            .service(get_alerts_by_state_id)
            .service(get_alerts_by_type)
            .service(get_alerts_by_level)
            .service(get_unseen_alerts)
            .service(mark_alert_as_seen),
    );

    // Planning routes
    cfg.service(
        web::scope("/api/plannings")
            .service(create_planning)
            .service(get_all_plannings)
            .service(get_planning_by_id)
            .service(update_planning)
            .service(delete_planning)
            .service(get_plannings_by_ground_id)
            .service(get_current_plannings)
            .service(get_upcoming_plannings),
    );
}

