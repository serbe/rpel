CREATE TABLE IF NOT EXISTS
    sirens (
        id            bigserial PRIMARY KEY,
        num_id        bigint,
        num_pass      text,
        siren_type_id bigint,
        address       text,
        radio         text,
        desk          text,
        contact_id    bigint,
        company_id    bigint,
        latitude      text,
        longitude     text,
        stage         bigint,
        own           text,
        note          text,
        created_at    TIMESTAMP without time zone,
        updated_at    TIMESTAMP without time zone,
        UNIQUE(num_id, num_pass, type_id)
    );