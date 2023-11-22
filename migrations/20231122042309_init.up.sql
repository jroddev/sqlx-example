
-- for uuid_generate_v4
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE test (
    id uuid DEFAULT uuid_generate_v4 (),
    value TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);

