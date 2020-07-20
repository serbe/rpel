CREATE TABLE IF NOT EXISTS
    users (
        id         bigserial primary key,
        name       text,
        key        text,
        role       bigint,
        created_at TIMESTAMP without time zone,
        updated_at TIMESTAMP without time zone,
        UNIQUE (name)
    );