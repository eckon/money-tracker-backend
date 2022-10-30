.PHONY: lint start

lint:
	cargo clippy -- \
		-W clippy::pedantic \
		-W clippy::nursery \
		-W clippy::unwrap_used \
		-W clippy::expect_used

setup:
	docker compose up -d db adminer && \
		sqlx database create && \
		sqlx migrate run
