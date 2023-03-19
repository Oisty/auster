-- Add up migration script here
CREATE TABLE categories (
  category_id text PRIMARY KEY,
  name text NOT NULL UNIQUE
);