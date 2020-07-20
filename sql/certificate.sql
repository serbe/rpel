CREATE TABLE IF NOT EXISTS
	certificates (
		id         BIGSERIAL PRIMARY KEY,
		num        TEXT,
		contact_id BIGINT,
		company_id BIGINT,
		cert_date  DATE,
		note       TEXT,
		created_at TIMESTAMP without time zone,
		updated_at TIMESTAMP without time zone default now(),
		UNIQUE(num)
	);
