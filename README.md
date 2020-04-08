# Save and forget core

## Instructions

1. Create database user

    ```shell
    createuser -P saveandforget
    ```

    Enter a password of your choice. The following instructions assume you
    used `saveandforget` as password.

    This step is **optional** and you can also use an existing database user
    for that. Just make sure to replace `saveandforget` by the database user
    of your choice in the following steps and change the `.env` file
    containing the configuration accordingly.

2. Create database

    ```shell
    createdb -O saveandforget saveandforget_db
    ```

3. Initialize database

    ```shell
    psql -U saveandforget -f sql/schema.sql saveandforget_db
    ```

    This step can be repeated and clears the database as it drops and
    recreates the schema `testing` which is used within the database.

4. Create `.env` file:

    ```ini
    SERVER_ADDR=127.0.0.1:8080
    PG.USER=saveandforget
    PG.PASSWORD=saveandforget
    PG.HOST=127.0.0.1
    PG.PORT=5432
    PG.DBNAME=saveandforget_db
    PG.POOL.MAX_SIZE=16
    ```
