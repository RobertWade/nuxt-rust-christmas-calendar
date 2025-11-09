BEGIN;

DROP TYPE IF EXISTS calendar_status CASCADE;
CREATE TYPE calendar_status AS ENUM ('draft', 'ready', 'published');

DROP TYPE IF EXISTS calendar_door_state CASCADE;
CREATE TYPE calendar_door_state AS ENUM ('locked', 'available', 'opened');

CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    display_name TEXT,
    avatar_url TEXT,
    password_hash TEXT NOT NULL,
    reset_token TEXT,
    reset_token_expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS calendars (
    id BIGSERIAL PRIMARY KEY,
    owner BIGINT REFERENCES users(id) ON DELETE SET NULL,
    name TEXT NOT NULL,
    description TEXT,
    recipient_name TEXT,
    status calendar_status NOT NULL DEFAULT 'draft',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    published_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS presents (
    id BIGSERIAL PRIMARY KEY,
    calendar_id BIGINT NOT NULL REFERENCES calendars(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    media_url TEXT,
    link_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS calendar_doors (
    id BIGSERIAL PRIMARY KEY,
    calendar_id BIGINT NOT NULL REFERENCES calendars(id) ON DELETE CASCADE,
    day SMALLINT NOT NULL CHECK (day BETWEEN 1 AND 31),
    title TEXT NOT NULL,
    opens_at TIMESTAMPTZ,
    state calendar_door_state NOT NULL DEFAULT 'locked',
    present_id BIGINT REFERENCES presents(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (calendar_id, day)
);

CREATE INDEX IF NOT EXISTS idx_users_reset_token ON users(reset_token) WHERE reset_token IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_calendars_owner ON calendars(owner);
CREATE INDEX IF NOT EXISTS idx_presents_calendar ON presents(calendar_id);
CREATE INDEX IF NOT EXISTS idx_doors_calendar ON calendar_doors(calendar_id);
CREATE INDEX IF NOT EXISTS idx_doors_present ON calendar_doors(present_id);

COMMIT;