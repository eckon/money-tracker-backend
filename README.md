# Money Tracker Backend

[![Cargo Setups](https://github.com/eckon/rust-backend/actions/workflows/cargo.yml/badge.svg)](https://github.com/eckon/rust-backend/actions/workflows/cargo.yml)
[![Conventional Commits](https://github.com/eckon/rust-backend/actions/workflows/conventional-commits.yml/badge.svg)](https://github.com/eckon/rust-backend/actions/workflows/conventional-commits.yml)


## TODO (critical)
- [ ] tests
  - stuff breaks more often as I know have logic (like sorting, filtering, etc.) this should automatically be checked and not manually by scripts
  - [ ] example is the snapshot, I know how the endresult should look like with specific data the logic needs to work
    - acc1 has x cost and y payments, acc2 has z payments this should result in acc1 oweing acc2 X and acc2 oweing acc1 Y
- [ ] write a diagram (mermaid) for the db structure to not forget what it is trying to do
- add more validation for most endpoints
  - [ ] add checks if given uuid is existing in the db
  - [ ] check if debts can be created (and remove stuff if not - transaction)
  - [ ] check if debts has max of 100% percentage
- [ ] trim data from dtos (as we can not trust the user)
  - axum does not do that by its own (find a general way to do that for all strings
  - also remove empty data in general (tags etc, make it optional?)


## TODO (important)
- [ ] decide for a name of the project
  - [ ] update rust specific files with that name
  - [ ] update git specific files with that name
  - some ideas:
    - money-dex
    - money tracker
- [ ] try out some tester like `bacon` or `cargo install cargo-watch` -> `cargo watch -x run` for hot reload
- [ ] delete endpoints (maybe one update for account, as this has linked entries)
- [ ] split parts
  - [ ] split api maybe in service and controller (and multiple parts that are combined in the mod.rs - or main.rs)
- [ ] backup data if we use it as our money setup


## TODO (nice to have) - ignored features (for now)
- authentication
- different user accounts (multiple users to use the application at the same time)
  - the prototype will only have one overall "user" meaning that everything will be changed to thing (only one snapshot etc)
- graphql
  - I want to try it later on, but first I want a working prototype as I am still struggling with the new language/environment
- swagger


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


## prod
- run `docker compose up`
  - will create
    - db
    - adminer for db interaction
    - backend via rust
  - will run the db migration
  - will start the backend
  - no envs need to be setup
    - *BUT* it will overwrite the local `.env` if it already exists


## dev
- add `.env` file with `DATABASE_URL/API_ADDR`
  - just copy the `.env.example` to `.env`
  - `DATABASE_URL`
    - for the service to connect to (started in docker)
    - for the sqlx cli migration command
  - `API_ADDR`
    - for the server and docker
- run `make setup` (runs docker compose up and migration)
- start server with `cargo run`
- add migrations (up/down) with `cargo install sqlx-cli`
  - `sqlx migrate add -r <name>`
- run the seeding script to populate the db (while the service is running)
  - `./examples/call-endpoints.sh`
- updates of database needs to be regenerated
  - so that sqlx can be run in offline mode
  - `cargo sqlx prepare`
  - and commit the `sqlx-data.json`

### possible needed libs
- ssl
  - sudo apt-get install pkg-config libssl-dev
