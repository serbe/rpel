CREATE TABLE IF NOT EXISTS 
    companies (
        id         bigserial PRIMARY KEY,
        name       text,
        fullname   text,
        address    text,
        scope_id   bigint,
        note       text,
        created_at timestamp without time zone,
        updated_at timestamp without time zone DEFAULT now(),
        UNIQUE(name, scope_id)
    );