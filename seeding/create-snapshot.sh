#!/usr/bin/env bash

# get bearer token via auth endpoint
bearer="BEARER_TOKEN"

# create account for payer
accountPay=$(
  curl "localhost:3000/account" \
    -X POST \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $bearer" \
    -d '{"name":"payer"}' |
    jq -r ".id"
)

# create account for debtor
accountDebt=$(
  curl "localhost:3000/account" \
    -X POST \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $bearer" \
    -d '{"name":"debtor"}' |
    jq -r ".id"
)


# create costs from payer
curl "localhost:3000/account/$accountPay/cost" \
  -X POST \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $bearer" \
  -d \
    "{
      \"debtors\": [
        { \"account_id\": \"$accountPay\", \"percentage\": 20 },
        { \"account_id\": \"$accountDebt\", \"percentage\": 80 }
      ],
      \"amount\": 4.12,
      \"description\":\"i payed\",
      \"tags\": [\"f\", \"b\", \"f\"],
      \"event_date\":\"2222-01-01\"
    }" > /dev/null

# create payment from debtor
curl "localhost:3000/account/$accountDebt/payment" \
  -X POST \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $bearer" \
  -d \
    "{
      \"lender_account_id\": \"$accountPay\",
      \"amount\": 1,
      \"description\":\"i payed back\",
      \"event_date\":\"2222-01-01\"
    }" > /dev/null
