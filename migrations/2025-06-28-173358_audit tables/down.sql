-- This file should undo anything in `up.sql`
-- Drop triggers first
DROP TRIGGER IF EXISTS audit_deck_permissions_trigger ON deck_permissions;

DROP TRIGGER IF EXISTS audit_cards_trigger ON cards;

DROP TRIGGER IF EXISTS audit_decks_trigger ON decks;

DROP TRIGGER IF EXISTS audit_users_trigger ON users;

-- Drop function
DROP FUNCTION IF EXISTS audit_trigger ();

-- Drop audit tables
DROP TABLE IF EXISTS audit_deck_permissions;

DROP TABLE IF EXISTS audit_cards;

DROP TABLE IF EXISTS audit_decks;

DROP TABLE IF EXISTS audit_users;

-- Drop custom type
DROP TYPE IF EXISTS audit_operation;
