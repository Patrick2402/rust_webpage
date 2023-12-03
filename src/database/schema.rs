// @generated automatically by Diesel CLI.

diesel::table! {
    groups (group_id) {
        group_id -> Int4,
        group_name -> Varchar,
    }
}

diesel::table! {
    permissions (group_id) {
        group_id -> Int4,
        group_name -> Varchar,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        session_id -> Varchar,
    }
}

diesel::table! {
    user_groups (user_id, group_id) {
        user_id -> Int4,
        group_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        salt -> Varchar,
    }
}

diesel::joinable!(user_groups -> permissions (group_id));
diesel::joinable!(user_groups -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(groups, permissions, sessions, user_groups, users,);
