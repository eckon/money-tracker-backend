#!/usr/bin/env bash

# Endpoint: account
## POST: account
### create an account
accountPay=$(
  curl "localhost:3000/account" -X POST -H "Content-Type: application/json" -d '{"name":"payer"}' |
    jq -r ".id"
)

### create secound account for linking to the first
accountDebt=$(
  curl "localhost:3000/account" -X POST -H "Content-Type: application/json" -d '{"name":"debtor"}' |
    jq -r ".id"
)


## POST: account/cost
### create a new cost of account
responseCost=$(
  curl "localhost:3000/account/$accountPay/cost" \
    -X POST \
    -H "Content-Type: application/json" \
    -d "{\"debtors\": [{\"account_id\": \"$accountDebt\", \"percentage\": 60}], \"amount\": 4.12, \"description\":\"i payed\", \"tags\": [\"f\", \"b\", \"f\"], \"event_date\":\"2222-01-01\"}" |
    jq -r ".id"
)


## POST: account/payment
### create a new payment of account
curl "localhost:3000/account/$accountDebt/payment" \
  -X POST \
  -H "Content-Type: application/json" \
  -d "{\"lender_account_id\": \"$accountPay\", \"amount\": 1, \"description\":\"i payed back\", \"event_date\":\"2222-01-01\"}" |
  jq -r ".id"


## GET: account
### get all accounts
curl "localhost:3000/account" -v


## GET: account
### get an account
curl "localhost:3000/account/$accountPay" -v


## GET: account/tags
### get all tags of given account
curl "localhost:3000/account/$accountPay/tags" -v


# Endpoint: cost
## GET: cost
### get all costs
curl "localhost:3000/cost" -v


# Endpoint: payment
## GET: payment
### get all payments
curl "localhost:3000/payment" -v


# Endpoint: snapshot
## GET: snapshot
### get current snapshot
curl "localhost:3000/snapshot" -v
