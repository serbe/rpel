CREATE TABLE IF NOT EXISTS
    kinds (
        id         bigserial PRIMARY KEY,
        name       text,
        short_name text,
        note       text,
        created_at TIMESTAMP without time zone,
        updated_at TIMESTAMP without time zone DEFAULT now(),
        UNIQUE (name)
    );