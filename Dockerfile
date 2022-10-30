FROM rust:latest

WORKDIR /app
RUN cargo install sqlx-cli
