-- Add up migration script here
CREATE TABLE categories (
  id text PRIMARY KEY,
  name text NOT NULL UNIQUE
);