-- Add migration script here

CREATE TABLE IF NOT EXISTS sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL UNIQUE,
    session_start TEXT NOT NULL
);

INSERT OR IGNORE INTO sessions (session_id, session_start)
SELECT '00000000-0000-0000-0000-000000000000', MIN(created_at)
FROM sips;

-- Add the session_id column to sips table
ALTER TABLE sips ADD COLUMN session_id INTEGER;

-- Update all existing sips to reference the legacy session
UPDATE sips SET session_id = (
    SELECT id FROM sessions WHERE session_id = '00000000-0000-0000-0000-000000000000'
);

-- In SQLite, we need to recreate the table to add NOT NULL constraint and foreign key
-- First, create the new table structure
CREATE TABLE sips_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    amount INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    notified_user BOOLEAN NOT NULL DEFAULT 0,
    session_id INTEGER NOT NULL,
    FOREIGN KEY(session_id) REFERENCES sessions(id)
);

-- Copy data from old table to new table
INSERT INTO sips_new (id, amount, created_at, notified_user, session_id)
SELECT id, amount, created_at, notified_user, session_id FROM sips;

-- Drop old table and rename new table
DROP TABLE sips;
ALTER TABLE sips_new RENAME TO sips;