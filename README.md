# testing rust backend

## todo
- [ ] try out some tester like `bacon` or `cargo install cargo-watch` -> `cargo watch -x run` for hot reload
- [ ] write seeding script to allow for easy testing of multiple parts
- [ ] swagger
- [ ] create setup for production (e.g. to use the frontend)
  - either another docker setup
  - or create an executable (probably through docker/github-actions)
- [ ] create setup completly in docker
  - so i dont need sqlx etc in my env

### NEXT
- [ ] endpoint to calculate the current state of money (snapshot)
- [ ] allow adding date information to entries
- [ ] tests
  - stuff breaks more often as I know have logic (like sorting, filtering, etc.) this should automatically be checked and not manually by scripts


## ignored features (for now)
- authentication
- different user accounts (multiple users to use the application at the same time)
  - the prototype will only have one overall "user" meaning that everything will be changed to thing (only one snapshot etc)
- graphql
  - I want to try it later on, but first I want a working prototype as I am still struggling with the new language/environment


## dev
- add `.env` file with `DATABASE_URL` and string example: `DATABASE_URL=postgres://user:password@server/db`
  - for the service to connect to (started in docker)
  - for the sqlx cli migration command
- `make setup` (runs docker compose up and migration)
- start server with `cargo run`
- add migrations (up/down) with `cargo install sqlx-cli`
  - `sqlx migrate add -r <name>`
- run the seeding script to populate the db (while the service is running)
  - `./examples/call-endpoints.sh`

- ssl
  - sudo apt-get install pkg-config libssl-dev


## examples
- https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md
