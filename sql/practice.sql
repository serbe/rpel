CREATE TABLE IF NOT EXISTS
    practices (
        id               bigserial PRIMARY KEY,
        company_id       bigint,
        kind_id          bigint,
        topic            text,
        date_of_practice time,
        note             text,
        created_at       timestamp without time zone,
        updated_at       timestamp without time zone DEFAULT now(),
        UNIQUE (company_id, kind_id, date_of_practice)
    );