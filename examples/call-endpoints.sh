#!/usr/bin/env bash

# Endpoint: account
## POST: account
### create an account
responseAccount=$(
  curl "localhost:3000/account" -X POST -H "Content-Type: application/json" -d '{"name":"name"}' |
    jq -r ".id"
)

## GET: account
### get an account
curl "localhost:3000/account/$responseAccount" -v

## POST: account/entry
### create a new entry of account
responseEntry=$(
  curl "localhost:3000/account/$responseAccount/entry" \
    -X POST \
    -H "Content-Type: application/json" \
    -d '{"kind":"Cost", "amount":123.12, "description":"description", "tags": ["foo", "bar", "foo"]}' |
    jq -r ".id"
)

## GET: account/entry
### get an entry of an account
curl "localhost:3000/account/$responseAccount/entry/$responseEntry" -v

## GET: account
### get all tags of given account
curl "localhost:3000/account/$responseAccount/tags" -v
