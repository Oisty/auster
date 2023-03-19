-- Add up migration script here
CREATE TABLE categories (
  category_id varchar PRIMARY KEY,
  name varchar NOT NULL UNIQUE
);