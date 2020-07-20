CREATE TABLE IF NOT EXISTS
    posts (
        id         bigserial PRIMARY KEY,
        name       text,
        go         bool NOT NULL DEFAULT FALSE,
        note       text,
        created_at timestamp without time zone,
        updated_at timestamp without time zone DEFAULT now(),
        UNIQUE (name, go)
    );