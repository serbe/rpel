CREATE TABLE IF NOT EXISTS hideout_types (
    id         bigserial primary key,
    name       text,
    note       text,
    created_at TIMESTAMP without time zone,
    updated_at TIMESTAMP without time zone default now(),
    UNIQUE(name)
);