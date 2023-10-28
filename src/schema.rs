// @generated automatically by Diesel CLI.

diesel::table! {
    machines (id) {
        id -> Uuid,
        name -> Varchar,
        property -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    properties (id) {
        id -> Uuid,
        name -> Varchar,
        address -> Varchar,
        address2 -> Nullable<Varchar>,
        city -> Varchar,
        zip -> Varchar,
        owner -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    reservations (id) {
        id -> Uuid,
        machine -> Uuid,
        owner -> Uuid,
        start_time -> Timestamp,
        end_time -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        role -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(machines -> properties (property));
diesel::joinable!(properties -> users (owner));
diesel::joinable!(reservations -> machines (machine));
diesel::joinable!(reservations -> users (owner));
diesel::joinable!(users -> roles (role));

diesel::allow_tables_to_appear_in_same_query!(
    machines,
    properties,
    reservations,
    roles,
    users,
);
