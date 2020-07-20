CREATE TABLE IF NOT EXISTS
    contacts (
        id            bigserial PRIMARY KEY,
        name          text,
        company_id    bigint,
        department_id bigint
        post_id       bigint,
        post_go_id    bigint,
        rank_id       bigint,
        birthday      date,
        note          text,
        created_at    timestamp without time zone,
        updated_at    timestamp without time zone DEFAULT now(),
        UNIQUE (name, birthday)
    );