DROP SCHEMA IF EXISTS testing CASCADE;
CREATE SCHEMA testing;

CREATE TABLE testing.users (
    uuid        uuid PRIMARY KEY,
    first_name  VARCHAR(200) NOT NULL,
    last_name   VARCHAR(200) NOT NULL,
    age         INT UNIQUE NOT NULL,
    UNIQUE (uuid)
);