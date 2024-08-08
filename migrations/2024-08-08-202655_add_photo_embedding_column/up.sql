-- Your SQL goes here
CREATE EXTENSION vector;

ALTER TABLE photos
ADD embedding VECTOR(512);