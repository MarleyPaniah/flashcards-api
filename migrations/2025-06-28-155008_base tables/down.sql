-- This file should undo anything in `up.sql`

-- Drop triggers first
DROP TRIGGER IF EXISTS trg_handle_soft_deletion_deck_permissions ON deck_permissions;

DROP TRIGGER IF EXISTS trg_handle_soft_deletion_cards ON cards;

DROP TRIGGER IF EXISTS trg_handle_soft_deletion_decks ON decks;

DROP TRIGGER IF EXISTS trg_handle_soft_deletion_users ON users;

DROP TRIGGER IF EXISTS trg_set_updated_at_deck_permissions ON deck_permissions;

DROP TRIGGER IF EXISTS trg_set_updated_at_cards ON cards;

DROP TRIGGER IF EXISTS trg_set_updated_at_decks ON decks;

DROP TRIGGER IF EXISTS trg_set_updated_at_users ON users;

-- Drop functions
DROP FUNCTION IF EXISTS handle_soft_deletion ();

DROP FUNCTION IF EXISTS set_updated_at ();

-- Drop tables
DROP TABLE IF EXISTS deck_permissions;

DROP TABLE IF EXISTS cards;

DROP TABLE IF EXISTS decks;

DROP TABLE IF EXISTS users;

-- Drop custom types
DROP TYPE IF EXISTS deck_permission_role;
