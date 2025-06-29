-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "users" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    "username" TEXT NOT NULL UNIQUE,
    "password_hash" TEXT NOT NULL,
    "email" TEXT NOT NULL UNIQUE,
    "last_login_at" TIMESTAMP NOT NULL DEFAULT now(),
    "is_verified" BOOL NOT NULL DEFAULT false,
    "is_deleted" BOOL NOT NULL DEFAULT false,
    "created_at" TIMESTAMP NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT now(),
    "deleted_at" TIMESTAMP DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS "decks" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    "short_id" TEXT NOT NULL UNIQUE,
    "title" TEXT NOT NULL,
    "metadata" JSONB NOT NULL DEFAULT '{}'::jsonb,
    "is_public" BOOL NOT NULL DEFAULT false,
    "is_deleted" BOOL NOT NULL DEFAULT false,
    "created_by" UUID NOT NULL REFERENCES users (id),
    "updated_by" UUID NOT NULL REFERENCES users (id),
    "deleted_by" UUID REFERENCES users (id) DEFAULT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT now(),
    "deleted_at" TIMESTAMP DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS "cards" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    "deck_id" UUID NOT NULL REFERENCES decks (id) ON DELETE CASCADE,
    "title" TEXT NOT NULL,
    "position" INTEGER NOT NULL,
    "content_front" TEXT NOT NULL,
    "content_back" TEXT NOT NULL,
    "difficulty" INTEGER NOT NULL,
    "metadata" JSONB NOT NULL DEFAULT '{}'::jsonb,
    "is_deleted" BOOL NOT NULL DEFAULT false,
    "created_by" UUID NOT NULL REFERENCES users (id),
    "updated_by" UUID NOT NULL REFERENCES users (id),
    "deleted_by" UUID REFERENCES users (id) DEFAULT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT now(),
    "deleted_at" TIMESTAMP DEFAULT NULL
);

DROP TYPE IF EXISTS deck_permission_role;

CREATE TYPE deck_permission_role AS ENUM ('owner', 'editor', 'viewer');

CREATE TABLE IF NOT EXISTS "deck_permissions" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    "deck_id" UUID NOT NULL REFERENCES decks (id) ON DELETE CASCADE,
    "user_id" UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    "role" deck_permission_role NOT NULL DEFAULT 'owner'::deck_permission_role,
    "is_deleted" BOOL NOT NULL DEFAULT false,
    "created_by" UUID NOT NULL REFERENCES users (id),
    "updated_by" UUID NOT NULL REFERENCES users (id),
    "deleted_by" UUID REFERENCES users (id) DEFAULT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT now(),
    "deleted_at" TIMESTAMP DEFAULT NULL
);

-- Create partial unique indexes
CREATE UNIQUE INDEX IF NOT EXISTS idx_unique_deck_position ON cards ("deck_id", "position")
WHERE
    "is_deleted" = false;

CREATE UNIQUE INDEX IF NOT EXISTS idx_unique_deck_permission ON deck_permissions ("deck_id", "user_id")
WHERE
    "is_deleted" = false;

-- functions
---- Automatically set updated_at on row change
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE PLPGSQL;

---- handle soft deletion
CREATE OR REPLACE FUNCTION handle_soft_deletion()
RETURNS TRIGGER AS $$
BEGIN
    -- if soft-deleted, update timestamp
    IF NEW.is_deleted AND NOT OLD.is_deleted THEN
        NEW.deleted_at := now();
    -- if restored, remove timestamp
    ELSEIF NOT NEW.is_deleted AND OLD.is_deleted THEN
        NEW.deleted_at := NULL;
        NEW.deleted_by := NULL;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE PLPGSQL;

-- triggers

---- auto set updated_at
DROP TRIGGER IF EXISTS trg_set_updated_at_users ON users;

CREATE TRIGGER trg_set_updated_at_users
BEFORE UPDATE ON users
FOR EACH ROW EXECUTE FUNCTION set_updated_at();

DROP TRIGGER IF EXISTS trg_set_updated_at_decks ON decks;

CREATE TRIGGER trg_set_updated_at_decks
BEFORE UPDATE ON decks
FOR EACH ROW EXECUTE FUNCTION set_updated_at();

DROP TRIGGER IF EXISTS trg_set_updated_at_cards ON cards;

CREATE TRIGGER trg_set_updated_at_cards
BEFORE UPDATE ON cards
FOR EACH ROW EXECUTE FUNCTION set_updated_at();

DROP TRIGGER IF EXISTS trg_set_updated_at_deck_permissions ON deck_permissions;

CREATE TRIGGER trg_set_updated_at_deck_permissions
BEFORE UPDATE ON deck_permissions
FOR EACH ROW EXECUTE FUNCTION set_updated_at();

---- soft deletion

DROP TRIGGER IF EXISTS trg_handle_soft_deletion_users ON users;

CREATE TRIGGER trg_handle_soft_deletion_users
BEFORE UPDATE ON users
FOR EACH ROW
WHEN (OLD.is_deleted IS DISTINCT FROM NEW.is_deleted)
EXECUTE FUNCTION handle_soft_deletion();

DROP TRIGGER IF EXISTS trg_handle_soft_deletion_decks ON decks;

CREATE TRIGGER trg_handle_soft_deletion_decks
BEFORE UPDATE ON decks
FOR EACH ROW
WHEN (OLD.is_deleted IS DISTINCT FROM NEW.is_deleted)
EXECUTE FUNCTION handle_soft_deletion();

DROP TRIGGER IF EXISTS trg_handle_soft_deletion_cards ON cards;

CREATE TRIGGER trg_handle_soft_deletion_cards
BEFORE UPDATE ON cards
FOR EACH ROW
WHEN (OLD.is_deleted IS DISTINCT FROM NEW.is_deleted)
EXECUTE FUNCTION handle_soft_deletion();

DROP TRIGGER IF EXISTS trg_handle_soft_deletion_deck_permissions ON deck_permissions;

CREATE TRIGGER trg_handle_soft_deletion_deck_permissions
BEFORE UPDATE ON deck_permissions
FOR EACH ROW
WHEN (OLD.is_deleted IS DISTINCT FROM NEW.is_deleted)
EXECUTE FUNCTION handle_soft_deletion();
