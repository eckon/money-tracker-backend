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
- [ ] figure out how to handle enums in sqlx (currently i just ignore it by casting to `_`)
- [ ] delete endpoints (maybe one update for account, as this has linked entries)
- [ ] diagram for the db structure

### NEXT
- [ ] endpoint to calculate the current state of money (snapshot)
- [ ] update account/entry to be linked to one or multiple accounts
  - because someone who pays need to say who has to pay back
    - this should be linked to multiple accounts
    - these accounts will probably need percantage of how much they owe
  - because someone who pays back needs to say to whom
    - this most likely needs only one account (as 1 pays 1 back)
- [ ] maybe differ payment and cost completly if they dont really belong togehter
- [ ] tests
  - stuff breaks more often as I know have logic (like sorting, filtering, etc.) this should automatically be checked and not manually by scripts


## ignored features (for now)
- authentication
- different user accounts (multiple users to use the application at the same time)
  - the prototype will only have one overall "user" meaning that everything will be changed to thing (only one snapshot etc)
- graphql
  - I want to try it later on, but first I want a working prototype as I am still struggling with the new language/environment

## idea
- people pay parts for goods (these should be noted down)
- person1 pays for food, they pay all of it but person2 owes them 50%
- person2 needs to have costs of this of 50%
- can create cost (when someone pays) and payments (when someone give money back)
- need to link costs to accounts
- need to link payment to account

### schema (do it with mermaid or similar later)
- account as the central block which holds data to the person
- cost as the thing that holds data about a real live transaction (e.g. shopping)
- debt as the thing which results out of a cost for others that need to repay (e.g. people that shopped as well but did not pay yet)
- payment as the thing which repays the cost (and counteracts the debt)

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
