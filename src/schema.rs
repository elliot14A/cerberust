// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "token_type_enum"))]
    pub struct TokenTypeEnum;
}

diesel::table! {
    privilege (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
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
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    role_privilege (role_id, privilege_id, resource_id) {
        role_id -> Uuid,
        privilege_id -> Uuid,
        resource_id -> Uuid,
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

diesel::table! {
    user_role (user_id, role_id) {
        user_id -> Uuid,
        role_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(refresh_token -> session (session_id));
diesel::joinable!(role_privilege -> privilege (privilege_id));
diesel::joinable!(role_privilege -> resource (resource_id));
diesel::joinable!(role_privilege -> role (role_id));
diesel::joinable!(session -> user (user_id));
diesel::joinable!(token -> user (user_id));
diesel::joinable!(user_role -> role (role_id));
diesel::joinable!(user_role -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    privilege,
    refresh_token,
    resource,
    role,
    role_privilege,
    session,
    token,
    user,
    user_role,
);
