// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "token_type_enum"))]
    pub struct TokenTypeEnum;
}

diesel::table! {
    refresh_token (id) {
        id -> Uuid,
        token -> Text,
        session_id -> Uuid,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    relation (user_id, resource_id) {
        user_id -> Uuid,
        resource_id -> Uuid,
        role_id -> Uuid,
    }
}

diesel::table! {
    resource (id) {
        id -> Uuid,
        parent_resource_id -> Nullable<Uuid>,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    role (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        privileges -> Jsonb,
        is_default -> Bool,
        resource_id -> Nullable<Uuid>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    session (id) {
        id -> Uuid,
        user_id -> Uuid,
        valid -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TokenTypeEnum;

    token (id) {
        id -> Uuid,
        user_id -> Uuid,
        token_text -> Text,
        token_type -> TokenTypeEnum,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    user (id) {
        id -> Uuid,
        username -> Text,
        email -> Text,
        password -> Text,
        email_verified -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(refresh_token -> session (session_id));
diesel::joinable!(relation -> resource (resource_id));
diesel::joinable!(relation -> role (role_id));
diesel::joinable!(relation -> user (user_id));
diesel::joinable!(role -> resource (resource_id));
diesel::joinable!(session -> user (user_id));
diesel::joinable!(token -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    refresh_token,
    relation,
    resource,
    role,
    session,
    token,
    user,
);
