CREATE TABLE IF NOT EXISTS
    emails (
        id         bigserial PRIMARY KEY,
        company_id bigint,
        contact_id bigint,
        email      text,
        note       text,
        created_at timestamp without time zone,
        updated_at timestamp without time zone DEFAULT now()
    );