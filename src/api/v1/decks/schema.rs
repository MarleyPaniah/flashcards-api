diesel::table! {
    decks (id) {
        id -> Integer,
        title -> Varchar,
        created_by -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_deleted -> Bool,
    }
}

diesel::table! {
    cards (id) {
        id -> Integer,
        deck -> Integer,
        position -> Integer,
        content -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_deleted -> Bool,
    }
}
