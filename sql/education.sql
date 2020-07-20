CREATE TABLE IF NOT EXISTS
    educations (
        id         bigserial PRIMARY KEY,
        contact_id bigint,
        post_id    bigint,
        start_date time,
        end_date   time,
        note       text,
        created_at timestamp without time zone,
        updated_at timestamp without time zone DEFAULT now()
    );