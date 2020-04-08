INSERT INTO document(filename, description)
VALUES ($1, $2)
RETURNING $table_fields;
