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

    Be sure that your database has permission to create the DB. If not, grant it in a database shell (run `psql`):

    ```sql
    ALTER USER saveandforget CREATEDB;
    ```

2. Create database

    ```shell
    createdb -O saveandforget saveandforget_db
    ```

To drop an existing database:
    ```shell
    dropdb saveandforget_db
    ```

3. Initialize database

    ```shell
    psql -U saveandforget -f sql/schema.sql saveandforget_db
    ```

    This step can be repeated and clears the database as it drops and
    recreates the tables.

4. Create `.env` file:

    ```ini
    FB_VERIFY_TOKEN="dev_token"
    RUST_LOG="actix_server=info,actix_web=info,saveandforget_web=debug"
    SERVER_ADDRESS="0.0.0.0:8000"
    DOWNLOAD_PATH="/home/vjousse/usr/src/saveandforget/saveandforget/downloads"
    DATABASE_URL="postgres://saveandforget:saveandforget@localhost/saveandforget_db"
    ```

## Reverse SSH tunnel

```sh
ssh -R 127.0.0.1:8080:localhost:8000 vjousse@marty.jousse.org
```

## Testing HTTP


```sh
http "https://vincent.jousse.org/sendandforget/webhook?hub.verify_token=dev_token&hub.challenge=CHALLENGE_ACCEPTED&hub.mode=subscribe"
```


## Useful links

- Tutorial with actix: https://dev.to/werner/practical-rust-web-development-api-rest-29g1
- https://medium.com/@aergonaut/writing-a-github-webhook-with-rust-part-1-rocket-4426dd06d45d
- https://developers.facebook.com/docs/messenger-platform/getting-started/quick-start
