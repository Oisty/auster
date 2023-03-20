-- Add up migration script here
CREATE TABLE rooms(
  id text PRIMARY KEY,
  name text NOT NULL UNIQUE,
  category_id integer NOT NULL,
  slots text not null,
  foreign key (category_id) references categories(id)
);