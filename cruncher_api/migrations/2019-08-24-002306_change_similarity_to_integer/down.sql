-- This file should undo anything in `up.sql`
ALTER TABLE similarity_ranking
ALTER COLUMN similarity TYPE numeric;