# Money Tracker Backend

[![Cargo Setups](https://github.com/eckon/rust-backend/actions/workflows/cargo.yml/badge.svg)](https://github.com/eckon/rust-backend/actions/workflows/cargo.yml)
[![Conventional Commits](https://github.com/eckon/rust-backend/actions/workflows/conventional-commits.yml/badge.svg)](https://github.com/eckon/rust-backend/actions/workflows/conventional-commits.yml)

**WIP** project to store costs and payments of accounts to figure out who has to pay what and how much money is spent at different places.


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
- swagger can be found under `<API>/swagger-ui`
- seeding script can be found in `./seeding/SOMEINTE`


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
