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


## POST: account/entry
### create a new entry of account
responseEntry=$(
  curl "localhost:3000/account/$accountPay/cost" \
    -X POST \
    -H "Content-Type: application/json" \
    -d "{\"debtor_account_ids\": [\"$accountDebt\"], \"amount\": 4.12, \"description\":\"i payed\", \"tags\": [\"f\", \"b\", \"f\"], \"event_date\":\"2222-01-01\"}" |
    jq -r ".id"
)

### create seound entry for payment
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


## GET: account
### get all tags of given account
# curl "localhost:3000/account/$accountPay/tags" -v


## GET: account/entry
### get an entry of an account
# curl "localhost:3000/account/$accountPay/entry/$responseEntry" -v
