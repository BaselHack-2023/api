-- Your SQL goes here
ALTER TABLE machines ADD COLUMN state VARCHAR NOT NULL DEFAULT 'stopped';
ALTER TABLE machines ADD COLUMN eta TIMESTAMP DEFAULT NOW();