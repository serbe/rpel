CREATE TABLE IF NOT EXISTS
    departments (
        id         bigserial primary key,
        name       text,
        note       text,
        created_at TIMESTAMP without time zone,
        updated_at TIMESTAMP without time zone,
        UNIQUE (name)
    );