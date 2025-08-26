// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::alert::{AlertTypeMapping, LevelTypeMapping};

    alert (id) {
        id -> Int4,
        date -> Nullable<Timestamp>,
        title -> Varchar,
        description -> Varchar,
        #[sql_type = "AlertTypeMapping"]
        type_ -> AlertTypeMapping,
        #[sql_type = "LevelTypeMapping"]
        level -> LevelTypeMapping,
        recommandation -> Nullable<Varchar>,
        isseen -> Nullable<Bool>,
        state_id -> Nullable<Int4>,
    }
}

diesel::table! {
    culture_type (id) {
        id -> Int4,
        type_ -> Nullable<Varchar>,
    }
}

diesel::table! {
    discussion (id) {
        id -> Int4,
        name -> Varchar,
        initialised_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    ground (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        folder -> Nullable<Varchar>,
        culture_type -> Nullable<Int4>,
        location -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        pack -> Nullable<Varchar>,
    }
}

diesel::table! {
    location (id) {
        id -> Int4,
        longitude -> Float8,
        latitude -> Float8,
        city -> Varchar,
        country -> Varchar,
    }
}

diesel::table! {
    message (id) {
        id -> Int4,
        sender_id -> Nullable<Int4>,
        content -> Nullable<Varchar>,
        discussion_id -> Nullable<Int4>,
    }
}

diesel::table! {
    pack (id) {
        id -> Int4,
        pack_id -> Nullable<Varchar>,
        sensor_id -> Nullable<Int4>,
    }
}

diesel::table! {
    participant (id) {
        id -> Int4,
        discussion_id -> Nullable<Int4>,
        participant_id -> Nullable<Int4>,
    }
}

diesel::table! {
    person (id) {
        id -> Int4,
        name -> Varchar,
        firstname -> Nullable<Varchar>,
        date_birth -> Nullable<Timestamp>,
        location_birth -> Nullable<Varchar>,
        number -> Nullable<Varchar>,
        cin -> Nullable<Varchar>,
        user_id -> Nullable<Int4>,
    }
}

diesel::table! {
    planning (id) {
        id -> Int4,
        date -> Nullable<Timestamp>,
        title -> Varchar,
        description -> Varchar,
        start_date -> Nullable<Timestamp>,
        end_date -> Nullable<Timestamp>,
        ground -> Nullable<Int4>,
    }
}

diesel::table! {
    sensor (id) {
        id -> Int4,
        description -> Nullable<Varchar>,
        issue_date -> Nullable<Timestamp>,
        sensor_type -> Nullable<Int4>,
    }
}

diesel::table! {
    sensor_pack (id) {
        id -> Varchar,
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    sensor_type (id) {
        id -> Int4,
        type_ -> Nullable<Varchar>,
    }
}

diesel::table! {
    state (id) {
        id -> Int4,
        date -> Nullable<Timestamp>,
        temperature -> Nullable<Float8>,
        health -> Nullable<Float8>,
        production_progress -> Nullable<Float8>,
        humidity -> Nullable<Float8>,
        fertility -> Nullable<Float8>,
        rentability -> Nullable<Float8>,
        pack_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        username -> Text,
        email -> Text,
        hashed_password -> Text,
        role -> Text,
    }
}

diesel::joinable!(alert -> state (state_id));
diesel::joinable!(ground -> culture_type (culture_type));
diesel::joinable!(ground -> location (location));
diesel::joinable!(ground -> sensor_pack (pack));
diesel::joinable!(ground -> users (user_id));
diesel::joinable!(message -> discussion (discussion_id));
diesel::joinable!(message -> users (sender_id));
diesel::joinable!(pack -> sensor (sensor_id));
diesel::joinable!(pack -> sensor_pack (pack_id));
diesel::joinable!(participant -> discussion (discussion_id));
diesel::joinable!(participant -> users (participant_id));
diesel::joinable!(person -> users (user_id));
diesel::joinable!(planning -> ground (ground));
diesel::joinable!(sensor -> sensor_type (sensor_type));
diesel::joinable!(state -> sensor_pack (pack_id));

diesel::allow_tables_to_appear_in_same_query!(
    alert,
    culture_type,
    discussion,
    ground,
    location,
    message,
    pack,
    participant,
    person,
    planning,
    sensor,
    sensor_pack,
    sensor_type,
    state,
    users,
);

