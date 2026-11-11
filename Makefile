
.PHONY: help run-api test lint db-start db-stop db-migrate health

help:
	@echo "Available commands:"
	@echo "  make run-api"
	@echo "  make test"
	@echo "  make lint"
	@echo "  make db-start"
	@echo "  make db-stop"
	@echo "  make db-migrate"
	@echo "  make health"

run-api:
	cargo run -p api

test:
	cargo test --workspace

lint:
	cargo fmt --all -- --check

db-start:
	docker compose up -d

db-stop:
	docker compose down

db-migrate:
	atlas migrate apply --env local

health:
	curl --fail http://localhost:3000/health
