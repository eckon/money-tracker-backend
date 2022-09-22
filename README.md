# testing rust backend

## todo
- [ ] maybe try out [axum](https://github.com/tokio-rs/axum) from tokio
  - mainly as it is from axum, and im still not really happy with the current one
  - also it seems to be popular, compared to the "older" actix
- [ ] try out some tester like `bacon` or `cargo install cargo-watch` -> `cargo watch -x run` for hot reload
- [ ] write tests
- [ ] authentication (on hold)
  - seems like there is an extra package, but most likely the auth logic needs to be done manually
  - https://github.com/actix/actix-extras/tree/HEAD/actix-web-httpauth
    - this works fine, as it allows for an easy middleware as well
  - https://github.com/DDtKey/actix-web-grants/tree/main/examples/jwt-httpauth/src
  - https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/#Securing-the-API
    - tried this once, didnt work as easily as hoped, had problems with authority, and the libs were outdated as well
    - maybe just use a thrid party for that (like Auth0) probably best as this shouldnt be done lightly
- [ ] improve db
  - https://github.dev/actix/examples/tree/fbd3b228e98166ae010b0e9e612565b33a3c1699/basics/todo
- [ ] swagger is possible, but i dont like highjacking everything just for some docs
  - https://paperclip-rs.github.io/paperclip/actix-plugin.html


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
- axum
  - https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md

- https://www.vultr.com/docs/building-rest-apis-in-rust-with-actix-web/
- https://hub.qovery.com/guides/tutorial/create-a-blazingly-fast-api-in-rust-part-1/
- https://www.youtube.com/watch?v=L8tWKqSMKUI
- https://blog.logrocket.com/create-backend-api-with-rust-postgres/
- https://gitlab.com/T-x-T/txts-treasury/-/tree/main/backend
