-- Add up migration script here
CREATE TABLE rooms(
  room_id varchar PRIMARY KEY,
  name varchar NOT NULL UNIQUE,
  category varchar NOT NULL,
  slots integer not null,
  foreign key (category) references categories(category_id)
);