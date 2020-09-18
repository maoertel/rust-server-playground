DROP SCHEMA IF EXISTS testing CASCADE;
CREATE SCHEMA testing;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE testing.users
(
    uuid       uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    first_name VARCHAR(200) NOT NULL,
    last_name  VARCHAR(200) NOT NULL,
    age        INT          NOT NULL,
    UNIQUE (uuid)
);