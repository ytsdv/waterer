-- Add migration script here

ALTER TABLE sips ADD COLUMN notified_user BOOLEAN NOT NULL DEFAULT FALSE;
