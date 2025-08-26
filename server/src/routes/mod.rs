use crate::auth::jwt::JwtMiddleware;
use crate::handlers::*;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Auth routes
    cfg.service(
        web::scope("/api/auth")
            .route("/login", web::post().to(auth_handler::login))
            .route("/register", web::post().to(auth_handler::register)),
    );

    // User routes
    cfg.service(
        web::scope("/api/users")
            .wrap(JwtMiddleware)
            .route("", web::post().to(user_handler::create_user))
            .route("", web::get().to(user_handler::get_all_users))
            .route("/me", web::get().to(user_handler::get_current_user))
            .route("/{id}", web::get().to(user_handler::get_user_by_id))
            .route("/{id}", web::put().to(user_handler::update_user))
            .route("/{id}", web::delete().to(user_handler::delete_user)),
    );

    // Person routes
    cfg.service(
        web::scope("/api/persons")
            .wrap(JwtMiddleware)
            .route("", web::post().to(person_handler::create_person))
            .route("", web::get().to(person_handler::get_all_persons))
            .route("/{id}", web::get().to(person_handler::get_person_by_id))
            .route("/user/{id}", web::get().to(person_handler::get_person_by_user_id))
            .route("/{id}", web::put().to(person_handler::update_person))
            .route("/{id}", web::delete().to(person_handler::delete_person)),
    );

    // Culture type routes
    cfg.service(
        web::scope("/api/culture-types")
            .wrap(JwtMiddleware)
            .route("", web::post().to(culture_type_handler::create_culture_type))
            .route("", web::get().to(culture_type_handler::get_all_culture_types))
            .route("/{id}", web::get().to(culture_type_handler::get_culture_type_by_id))
            .route("/{id}", web::put().to(culture_type_handler::update_culture_type))
            .route("/{id}", web::delete().to(culture_type_handler::delete_culture_type)),
    );

    // Location routes
    cfg.service(
        web::scope("/api/locations")
            .wrap(JwtMiddleware)
            .route("", web::post().to(location_handler::create_location))
            .route("", web::get().to(location_handler::get_all_locations))
            .route("/{id}", web::get().to(location_handler::get_location_by_id))
            .route("/{id}", web::put().to(location_handler::update_location))
            .route("/{id}", web::delete().to(location_handler::delete_location)),
    );

    // Sensor type routes
    cfg.service(
        web::scope("/api/sensor-types")
            .wrap(JwtMiddleware)
            .route("", web::post().to(sensor_type_handler::create_sensor_type))
            .route("", web::get().to(sensor_type_handler::get_all_sensor_types))
            .route("/{id}", web::get().to(sensor_type_handler::get_sensor_type_by_id))
            .route("/{id}", web::put().to(sensor_type_handler::update_sensor_type))
            .route("/{id}", web::delete().to(sensor_type_handler::delete_sensor_type)),
    );

    // Sensor routes
    cfg.service(
        web::scope("/api/sensors")
            .wrap(JwtMiddleware)
            .route("", web::post().to(sensor_handler::create_sensor))
            .route("", web::get().to(sensor_handler::get_all_sensors))
            .route("/{id}", web::get().to(sensor_handler::get_sensor_by_id))
            .route("/type/{id}", web::get().to(sensor_handler::get_sensors_by_type))
            .route("/{id}", web::put().to(sensor_handler::update_sensor))
            .route("/{id}", web::delete().to(sensor_handler::delete_sensor)),
    );

    // Sensor pack routes
    cfg.service(
        web::scope("/api/sensor-packs")
            .wrap(JwtMiddleware)
            .route("", web::post().to(sensor_pack_handler::create_sensor_pack))
            .route("", web::get().to(sensor_pack_handler::get_all_sensor_packs))
            .route("/{id}", web::get().to(sensor_pack_handler::get_sensor_pack_by_id))
            .route("/{id}", web::put().to(sensor_pack_handler::update_sensor_pack))
            .route("/{id}", web::delete().to(sensor_pack_handler::delete_sensor_pack)),
    );

    // Pack routes
    cfg.service(
        web::scope("/api/packs")
            .wrap(JwtMiddleware)
            .route("", web::post().to(pack_handler::create_pack))
            .route("", web::get().to(pack_handler::get_all_packs))
            .route("/{id}", web::get().to(pack_handler::get_pack_by_id))
            .route("/pack-id/{id}", web::get().to(pack_handler::get_packs_by_pack_id))
            .route("/sensor/{id}", web::get().to(pack_handler::get_packs_by_sensor_id))
            .route("/{id}", web::put().to(pack_handler::update_pack))
            .route("/{id}", web::delete().to(pack_handler::delete_pack)),
    );

    // Ground routes
    cfg.service(
        web::scope("/api/grounds")
            .wrap(JwtMiddleware)
            .route("", web::post().to(ground_handler::create_ground))
            .route("", web::get().to(ground_handler::get_all_grounds))
            .route("/{id}", web::get().to(ground_handler::get_ground_by_id))
            .route("/user/{id}", web::get().to(ground_handler::get_grounds_by_user_id))
            .route("/culture-type/{id}", web::get().to(ground_handler::get_grounds_by_culture_type))
            .route("/location/{id}", web::get().to(ground_handler::get_grounds_by_location))
            .route("/pack/{id}", web::get().to(ground_handler::get_grounds_by_pack))
            .route("/{id}", web::put().to(ground_handler::update_ground))
            .route("/{id}", web::delete().to(ground_handler::delete_ground)),
    );

    // Discussion routes
    cfg.service(
        web::scope("/api/discussions")
            .wrap(JwtMiddleware)
            .route("", web::post().to(discussion_handler::create_discussion))
            .route("", web::get().to(discussion_handler::get_all_discussions))
            .route("/my", web::get().to(discussion_handler::get_discussions_by_participant))
            .route("/{id}", web::get().to(discussion_handler::get_discussion_by_id))
            .route("/{id}", web::put().to(discussion_handler::update_discussion))
            .route("/{id}", web::delete().to(discussion_handler::delete_discussion)),
    );

    // Message routes
    cfg.service(
        web::scope("/api/messages")
            .wrap(JwtMiddleware)
            .route("", web::post().to(message_handler::create_message))
            .route("/{id}", web::get().to(message_handler::get_message_by_id))
            .route("/discussion/{id}", web::get().to(message_handler::get_messages_by_discussion))
            .route("/{id}", web::put().to(message_handler::update_message))
            .route("/{id}", web::delete().to(message_handler::delete_message)),
    );

    // Participant routes
    cfg.service(
        web::scope("/api/participants")
            .wrap(JwtMiddleware)
            .route("", web::post().to(participant_handler::add_participant))
            .route("/discussion/{id}", web::get().to(participant_handler::get_participants_by_discussion))
            .route("/{discussion_id}/{participant_id}", web::delete().to(participant_handler::remove_participant)),
    );

    // State routes
    cfg.service(
        web::scope("/api/states")
            .wrap(JwtMiddleware)
            .route("", web::post().to(state_handler::create_state))
            .route("", web::get().to(state_handler::get_all_states))
            .route("/{id}", web::get().to(state_handler::get_state_by_id))
            .route("/pack/{id}", web::get().to(state_handler::get_states_by_pack_id))
            .route("/pack/{id}/latest", web::get().to(state_handler::get_latest_state_by_pack_id))
            .route("/{id}", web::put().to(state_handler::update_state))
            .route("/{id}", web::delete().to(state_handler::delete_state)),
    );

    // Alert routes
    cfg.service(
        web::scope("/api/alerts")
            .wrap(JwtMiddleware)
            .route("", web::post().to(alert_handler::create_alert))
            .route("", web::get().to(alert_handler::get_all_alerts))
            .route("/unseen", web::get().to(alert_handler::get_unseen_alerts))
            .route("/{id}", web::get().to(alert_handler::get_alert_by_id))
            .route("/state/{id}", web::get().to(alert_handler::get_alerts_by_state))
            .route("/type/{type}", web::get().to(alert_handler::get_alerts_by_type))
            .route("/level/{level}", web::get().to(alert_handler::get_alerts_by_level))
            .route("/{id}", web::put().to(alert_handler::update_alert))
            .route("/{id}/seen", web::put().to(alert_handler::mark_alert_as_seen))
            .route("/{id}", web::delete().to(alert_handler::delete_alert)),
    );

    // Planning routes
    cfg.service(
        web::scope("/api/plannings")
            .wrap(JwtMiddleware)
            .route("", web::post().to(planning_handler::create_planning))
            .route("", web::get().to(planning_handler::get_all_plannings))
            .route("/date-range", web::get().to(planning_handler::get_plannings_by_date_range))
            .route("/{id}", web::get().to(planning_handler::get_planning_by_id))
            .route("/ground/{id}", web::get().to(planning_handler::get_plannings_by_ground))
            .route("/{id}", web::put().to(planning_handler::update_planning))
            .route("/{id}", web::delete().to(planning_handler::delete_planning)),
    );
}

