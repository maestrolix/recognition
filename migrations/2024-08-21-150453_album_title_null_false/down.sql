-- This file should undo anything in `up.sql`
ALTER TABLE albums ALTER COLUMN title DROP NOT NULL;