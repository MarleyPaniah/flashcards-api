// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "audit_operation"))]
    pub struct AuditOperation;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "deck_permission_role"))]
    pub struct DeckPermissionRole;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AuditOperation;

    audit_cards (audit_id) {
        audit_id -> Int4,
        operation -> AuditOperation,
        changed_at -> Timestamptz,
        id -> Nullable<Uuid>,
        deck_id -> Nullable<Uuid>,
        title -> Nullable<Text>,
        position -> Nullable<Int4>,
        content_front -> Nullable<Text>,
        content_back -> Nullable<Text>,
        difficulty -> Nullable<Int4>,
        metadata -> Nullable<Jsonb>,
        is_deleted -> Nullable<Bool>,
        created_by -> Nullable<Uuid>,
        updated_by -> Nullable<Uuid>,
        deleted_by -> Nullable<Uuid>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AuditOperation;
    use super::sql_types::DeckPermissionRole;

    audit_deck_permissions (audit_id) {
        audit_id -> Int4,
        operation -> AuditOperation,
        changed_at -> Timestamptz,
        id -> Nullable<Uuid>,
        deck_id -> Nullable<Uuid>,
        user_id -> Nullable<Uuid>,
        role -> Nullable<DeckPermissionRole>,
        is_deleted -> Nullable<Bool>,
        created_by -> Nullable<Uuid>,
        updated_by -> Nullable<Uuid>,
        deleted_by -> Nullable<Uuid>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AuditOperation;

    audit_decks (audit_id) {
        audit_id -> Int4,
        operation -> AuditOperation,
        changed_at -> Timestamptz,
        id -> Nullable<Uuid>,
        short_id -> Nullable<Text>,
        title -> Nullable<Text>,
        metadata -> Nullable<Jsonb>,
        is_public -> Nullable<Bool>,
        is_deleted -> Nullable<Bool>,
        created_by -> Nullable<Uuid>,
        updated_by -> Nullable<Uuid>,
        deleted_by -> Nullable<Uuid>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AuditOperation;

    audit_users (audit_id) {
        audit_id -> Int4,
        operation -> AuditOperation,
        changed_at -> Timestamptz,
        id -> Nullable<Uuid>,
        username -> Nullable<Text>,
        password_hash -> Nullable<Text>,
        email -> Nullable<Text>,
        last_login_at -> Nullable<Timestamp>,
        is_verified -> Nullable<Bool>,
        is_deleted -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    cards (id) {
        id -> Uuid,
        deck_id -> Uuid,
        title -> Text,
        position -> Int4,
        content_front -> Text,
        content_back -> Text,
        difficulty -> Int4,
        metadata -> Jsonb,
        is_deleted -> Bool,
        created_by -> Uuid,
        updated_by -> Uuid,
        deleted_by -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::DeckPermissionRole;

    deck_permissions (id) {
        id -> Uuid,
        deck_id -> Uuid,
        user_id -> Uuid,
        role -> DeckPermissionRole,
        is_deleted -> Bool,
        created_by -> Uuid,
        updated_by -> Uuid,
        deleted_by -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    decks (id) {
        id -> Uuid,
        short_id -> Text,
        title -> Text,
        metadata -> Jsonb,
        is_public -> Bool,
        is_deleted -> Bool,
        created_by -> Uuid,
        updated_by -> Uuid,
        deleted_by -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        password_hash -> Text,
        email -> Text,
        last_login_at -> Timestamp,
        is_verified -> Bool,
        is_deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(cards -> decks (deck_id));
diesel::joinable!(deck_permissions -> decks (deck_id));

diesel::allow_tables_to_appear_in_same_query!(
    audit_cards,
    audit_deck_permissions,
    audit_decks,
    audit_users,
    cards,
    deck_permissions,
    decks,
    users,
);
