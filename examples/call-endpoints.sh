#!/usr/bin/env bash

# Endpoint: user
## POST: user
### create user
response=$(
  curl localhost:3000/user -X POST -H "Content-Type: application/json" -d '{"name":"test"}' |
    jq -r ".id"
)

## GET: user
### get user
curl "localhost:3000/user/$response" -v
