CREATE TABLE IF NOT EXISTS
    phones (
        id         bigserial PRIMARY KEY,
        contact_id bigint,
        company_id bigint,
        phone      bigint,
        fax        bool NOT NULL DEFAULT false,
        note       text,
        created_at timestamp without time zone,
        updated_at timestamp without time zone DEFAULT now()
    );