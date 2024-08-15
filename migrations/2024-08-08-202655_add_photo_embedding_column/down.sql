-- This file should undo anything in `up.sql`
DROP EXTENSION vector;

ALTER TABLE photos
DROP COLUMN embedding;