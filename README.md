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
First implementation used the official actix example mostly combined with the official diesel example

- https://www.vultr.com/docs/building-rest-apis-in-rust-with-actix-web/
- https://github.dev/actix/examples/tree/master/databases/diesel
- https://hub.qovery.com/guides/tutorial/create-a-blazingly-fast-api-in-rust-part-1/
- https://www.youtube.com/watch?v=L8tWKqSMKUI
- https://blog.logrocket.com/create-backend-api-with-rust-postgres/
- https://diesel.rs/guides/getting-started
