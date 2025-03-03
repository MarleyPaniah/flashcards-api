diesel::table! {
    users (id, username) {
        id -> Uuid,
        username -> Varchar,
        password_hash -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_login_at -> Timestamp,
        is_verified -> Bool,
        is_deleted -> Bool,
        }
}
