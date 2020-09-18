INSERT INTO testing.users(first_name, last_name, age)
VALUES ($1, $2, $3)
RETURNING $table_fields;