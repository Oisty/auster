CREATE TYPE header_pair AS
(
    name  TEXT,
    value BYTEA
);
--
create table public.users
(
    user_id       uuid not null primary key,
    username      text not null unique,
    password_hash text not null
);
--
create table public.idempotency
(
    user_id              uuid                     not null references public.users,
    idempotency_key      text                     not null,
    response_status_code smallint,
    response_headers     header_pair[],
    response_body        bytea,
    created_at           timestamp with time zone not null,
    primary key (user_id, idempotency_key)
);
--
INSERT INTO users (user_id, username, password_hash)
VALUES ('6e93ba3e-7d5c-4dfb-be4d-698f1c05204d',
        'admin',
        '$argon2id$v=19$m=15000,t=2,p=1$NAt9jqhXnqxZqUqc/30m7w$7vcnmcTfdC1X043Lm4BDkaGVR0o/EpFx3QsU0pH0Ok4');

