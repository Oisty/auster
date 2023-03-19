-- Add up migration script here
CREATE TABLE users(
  user_id varchar PRIMARY KEY,
  username varchar NOT NULL UNIQUE,
  password_hash varchar NOT NULL
);

INSERT INTO users (user_id, username, password_hash)
VALUES ('6e93ba3e-7d5c-4dfb-be4d-698f1c05204d',
        'admin',
        '$argon2id$v=19$m=15000,t=2,p=1$NAt9jqhXnqxZqUqc/30m7w$7vcnmcTfdC1X043Lm4BDkaGVR0o/EpFx3QsU0pH0Ok4');