CREATE TABLE IF NOT EXISTS
    scopes (
        id         bigserial PRIMARY KEY,
        name       text,
        note       text,
        created_at timestamp without time zone,
        updated_at timestamp without time zone DEFAULT now(),
        UNIQUE (name)
    );