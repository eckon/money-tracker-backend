#!/usr/bin/env bash

# Endpoint: account
## POST: account
### create an account
responseAccount=$(
  curl "localhost:3000/account" -X POST -H "Content-Type: application/json" -d '{"name":"n"}' |
    jq -r ".id"
)

## POST: account/entry
### create a new entry of account
responseEntry=$(
  curl "localhost:3000/account/$responseAccount/entry" \
    -X POST \
    -H "Content-Type: application/json" \
    -d '{"kind": "Cost", "amount": 3.12, "description":"d", "tags": ["f", "b", "f"], "event_date":"2222-01-01"}' |
    jq -r ".id"
)

## GET: account/entry
### get an entry of an account
curl "localhost:3000/account/$responseAccount/entry/$responseEntry" -v

## GET: account
### get an account
curl "localhost:3000/account/$responseAccount" -v

## GET: account
### get all tags of given account
curl "localhost:3000/account/$responseAccount/tags" -v

## GET: account
### get all accounts
curl "localhost:3000/account" -v
