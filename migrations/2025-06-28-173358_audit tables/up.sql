-- Your SQL goes here

DROP TYPE IF EXISTS audit_operation;

CREATE TYPE audit_operation AS ENUM ('I', 'U', 'D');

CREATE TABLE IF NOT EXISTS audit_users (
    audit_id SERIAL PRIMARY KEY,
    operation audit_operation NOT NULL,
    changed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    id UUID,
    username TEXT,
    password_hash TEXT,
    email TEXT,
    last_login_at TIMESTAMP,
    is_verified BOOL,
    is_deleted BOOL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS audit_decks (
    audit_id SERIAL PRIMARY KEY,
    operation audit_operation NOT NULL,
    changed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    id UUID,
    short_id TEXT,
    title TEXT,
    metadata JSONB,
    is_public BOOL,
    is_deleted BOOL,
    created_by UUID,
    updated_by UUID,
    deleted_by UUID,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS audit_cards (
    audit_id SERIAL PRIMARY KEY,
    operation audit_operation NOT NULL,
    changed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    id UUID,
    deck_id UUID,
    title TEXT,
    position INTEGER,
    content_front TEXT,
    content_back TEXT,
    difficulty INTEGER,
    metadata JSONB,
    is_deleted BOOL,
    created_by UUID,
    updated_by UUID,
    deleted_by UUID,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS audit_deck_permissions (
    audit_id SERIAL PRIMARY KEY,
    operation audit_operation NOT NULL,
    changed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    id UUID,
    deck_id UUID,
    user_id UUID,
    role deck_permission_role,
    is_deleted BOOL,
    created_by UUID,
    updated_by UUID,
    deleted_by UUID,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);

-- Optimized audit triggers

-- Users audit trigger
CREATE OR REPLACE FUNCTION audit_users_trigger() RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        INSERT INTO audit_users (operation, changed_at, id, username, password_hash, email, last_login_at, is_verified, is_deleted, created_at, updated_at, deleted_at)
        VALUES ('D', now(), OLD.id, OLD.username, OLD.password_hash, OLD.email, OLD.last_login_at, OLD.is_verified, OLD.is_deleted, OLD.created_at, OLD.updated_at, OLD.deleted_at);
        RETURN OLD;
    ELSIF TG_OP = 'UPDATE' THEN
        INSERT INTO audit_users (operation, changed_at, id, username, password_hash, email, last_login_at, is_verified, is_deleted, created_at, updated_at, deleted_at)
        VALUES ('U', now(), NEW.id, NEW.username, NEW.password_hash, NEW.email, NEW.last_login_at, NEW.is_verified, NEW.is_deleted, NEW.created_at, NEW.updated_at, NEW.deleted_at);
        RETURN NEW;
    ELSIF TG_OP = 'INSERT' THEN
        INSERT INTO audit_users (operation, changed_at, id, username, password_hash, email, last_login_at, is_verified, is_deleted, created_at, updated_at, deleted_at)
        VALUES ('I', now(), NEW.id, NEW.username, NEW.password_hash, NEW.email, NEW.last_login_at, NEW.is_verified, NEW.is_deleted, NEW.created_at, NEW.updated_at, NEW.deleted_at);
        RETURN NEW;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Decks audit trigger
CREATE OR REPLACE FUNCTION audit_decks_trigger() RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        INSERT INTO audit_decks (operation, changed_at, id, short_id, title, metadata, is_public, is_deleted, created_by, updated_by, deleted_by, created_at, updated_at, deleted_at)
        VALUES ('D', now(), OLD.id, OLD.short_id, OLD.title, OLD.metadata, OLD.is_public, OLD.is_deleted, OLD.created_by, OLD.updated_by, OLD.deleted_by, OLD.created_at, OLD.updated_at, OLD.deleted_at);
        RETURN OLD;
    ELSIF TG_OP = 'UPDATE' THEN
        INSERT INTO audit_decks (operation, changed_at, id, short_id, title, metadata, is_public, is_deleted, created_by, updated_by, deleted_by, created_at, updated_at, deleted_at)
        VALUES ('U', now(), NEW.id, NEW.short_id, NEW.title, NEW.metadata, NEW.is_public, NEW.is_deleted, NEW.created_by, NEW.updated_by, NEW.deleted_by, NEW.created_at, NEW.updated_at, NEW.deleted_at);
        RETURN NEW;
    ELSIF TG_OP = 'INSERT' THEN
        INSERT INTO audit_decks (operation, changed_at, id, short_id, title, metadata, is_public, is_deleted, created_by, updated_by, deleted_by, created_at, updated_at, deleted_at)
        VALUES ('I', now(), NEW.id, NEW.short_id, NEW.title, NEW.metadata, NEW.is_public, NEW.is_deleted, NEW.created_by, NEW.updated_by, NEW.deleted_by, NEW.created_at, NEW.updated_at, NEW.deleted_at);
        RETURN NEW;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Cards audit trigger
CREATE OR REPLACE FUNCTION audit_cards_trigger() RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        INSERT INTO audit_cards (operation, changed_at, id, deck_id, title, position, content_front, content_back, difficulty, metadata, is_deleted, created_by, updated_by, deleted_by, created_at, updated_at, deleted_at)
        VALUES ('D', now(), OLD.id, OLD.deck_id, OLD.title, OLD.position, OLD.content_front, OLD.content_back, OLD.difficulty, OLD.metadata, OLD.is_deleted, OLD.created_by, OLD.updated_by, OLD.deleted_by, OLD.created_at, OLD.updated_at, OLD.deleted_at);
        RETURN OLD;
    ELSIF TG_OP = 'UPDATE' THEN
        INSERT INTO audit_cards (operation, changed_at, id, deck_id, title, position, content_front, content_back, difficulty, metadata, is_deleted, created_by, updated_by, deleted_by, created_at, updated_at, deleted_at)
        VALUES ('U', now(), NEW.id, NEW.deck_id, NEW.title, NEW.position, NEW.content_front, NEW.content_back, NEW.difficulty, NEW.metadata, NEW.is_deleted, NEW.created_by, NEW.updated_by, NEW.deleted_by, NEW.created_at, NEW.updated_at, NEW.deleted_at);
        RETURN NEW;
    ELSIF TG_OP = 'INSERT' THEN
        INSERT INTO audit_cards (operation, changed_at, id, deck_id, title, position, content_front, content_back, difficulty, metadata, is_deleted, created_by, updated_by, deleted_by, created_at, updated_at, deleted_at)
        VALUES ('I', now(), NEW.id, NEW.deck_id, NEW.title, NEW.position, NEW.content_front, NEW.content_back, NEW.difficulty, NEW.metadata, NEW.is_deleted, NEW.created_by, NEW.updated_by, NEW.deleted_by, NEW.created_at, NEW.updated_at, NEW.deleted_at);
        RETURN NEW;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Deck permissions audit trigger
CREATE OR REPLACE FUNCTION audit_deck_permissions_trigger() RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        INSERT INTO audit_deck_permissions (operation, changed_at, id, deck_id, user_id, role, is_deleted, created_by, updated_by, deleted_by, created_at, updated_at, deleted_at)
        VALUES ('D', now(), OLD.id, OLD.deck_id, OLD.user_id, OLD.role, OLD.is_deleted, OLD.created_by, OLD.updated_by, OLD.deleted_by, OLD.created_at, OLD.updated_at, OLD.deleted_at);
        RETURN OLD;
    ELSIF TG_OP = 'UPDATE' THEN
        INSERT INTO audit_deck_permissions (operation, changed_at, id, deck_id, user_id, role, is_deleted, created_by, updated_by, deleted_by, created_at, updated_at, deleted_at)
        VALUES ('U', now(), NEW.id, NEW.deck_id, NEW.user_id, NEW.role, NEW.is_deleted, NEW.created_by, NEW.updated_by, NEW.deleted_by, NEW.created_at, NEW.updated_at, NEW.deleted_at);
        RETURN NEW;
    ELSIF TG_OP = 'INSERT' THEN
        INSERT INTO audit_deck_permissions (operation, changed_at, id, deck_id, user_id, role, is_deleted, created_by, updated_by, deleted_by, created_at, updated_at, deleted_at)
        VALUES ('I', now(), NEW.id, NEW.deck_id, NEW.user_id, NEW.role, NEW.is_deleted, NEW.created_by, NEW.updated_by, NEW.deleted_by, NEW.created_at, NEW.updated_at, NEW.deleted_at);
        RETURN NEW;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- trigger definitions
DROP TRIGGER IF EXISTS audit_users_trigger ON users;

CREATE TRIGGER audit_users_trigger
    AFTER INSERT OR UPDATE OR DELETE ON users
    FOR EACH ROW EXECUTE FUNCTION audit_users_trigger();

DROP TRIGGER IF EXISTS audit_decks_trigger ON decks;

CREATE TRIGGER audit_decks_trigger
    AFTER INSERT OR UPDATE OR DELETE ON decks
    FOR EACH ROW EXECUTE FUNCTION audit_decks_trigger();

DROP TRIGGER IF EXISTS audit_cards_trigger ON cards;

CREATE TRIGGER audit_cards_trigger
    AFTER INSERT OR UPDATE OR DELETE ON cards
    FOR EACH ROW EXECUTE FUNCTION audit_cards_trigger();

DROP TRIGGER IF EXISTS audit_deck_permissions_trigger ON deck_permissions;

CREATE TRIGGER audit_deck_permissions_trigger
    AFTER INSERT OR UPDATE OR DELETE ON deck_permissions
    FOR EACH ROW EXECUTE FUNCTION audit_deck_permissions_trigger();
