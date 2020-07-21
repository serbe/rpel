CREATE TABLE IF NOT EXISTS
    users (
        id         bigserial primary key,
        name       text NOT NULL,
        key        text NOT NULL,
        role       bigint NOT NULL,
        created_at TIMESTAMP without time zone,
        updated_at TIMESTAMP without time zone,
        UNIQUE (name)
    );