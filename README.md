# testing rust backend

## todo
- try out some tester like `bacon` or `cargo install cargo-watch` -> `cargo watch -x run` for hot reload

## dev
- docker compose up
- add `.env` file with `DATABASE_URL` and string example: `DATABASE_URL=postgres://user:password@server/db`
  - for the service to connect to (started in docker)
  - for the sqlx cli migration command
- start server with `cargo run`

- sqlx migration
  - install `cargo install sqlx-cli`
  - `sqlx database create` create the db
  - `sqlx migrate add <name>`
  - `sqlx migrate run`
    - do it manually for now

- ssl
  - sudo apt-get install pkg-config libssl-dev

## examples
- https://github.dev/actix/examples/tree/fbd3b228e98166ae010b0e9e612565b33a3c1699/basics/todo
  - for sqlx in actix
- https://www.vultr.com/docs/building-rest-apis-in-rust-with-actix-web/
- https://hub.qovery.com/guides/tutorial/create-a-blazingly-fast-api-in-rust-part-1/
- https://www.youtube.com/watch?v=L8tWKqSMKUI
- https://blog.logrocket.com/create-backend-api-with-rust-postgres/
