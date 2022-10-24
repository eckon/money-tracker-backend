# testing rust backend

## todo
- [ ] try out some tester like `bacon` or `cargo install cargo-watch` -> `cargo watch -x run` for hot reload
- [ ] write tests
- [ ] write seeding script to allow for easy testing of multiple parts
- [ ] authentication
- [ ] swagger
- [ ] create setup for production (e.g. to use the frontend)
  - either another docker setup
  - or create an executable (probably through docker/github-actions)


## dev
- add `.env` file with `DATABASE_URL` and string example: `DATABASE_URL=postgres://user:password@server/db`
  - for the service to connect to (started in docker)
  - for the sqlx cli migration command
- `make setup` (runs docker compose up and migration)
- start server with `cargo run`
- add migrations (up/down) with `cargo install sqlx-cli`
  - `sqlx migrate add -r <name>`

- ssl
  - sudo apt-get install pkg-config libssl-dev


## examples
- https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md
