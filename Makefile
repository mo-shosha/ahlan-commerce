
.PHONY: help start stop run-api test lint db-start db-run db-health db-stop db-migrate health

COMPOSE := docker compose
DB_SERVICE := db
DB_USER := postgres
DB_NAME := ahlan_commerce

AHLAN_DB_PORT ?= 5433
DATABASE_URL ?= postgres://postgres:postgres@localhost:$(AHLAN_DB_PORT)/$(DB_NAME)
export DATABASE_URL

help:
	@echo "Available commands:"
	@echo "  make start"
	@echo "  make stop"
	@echo "  make run-api"
	@echo "  make test"
	@echo "  make lint"
	@echo "  make db-start"
	@echo "  make db-run"
	@echo "  make db-health"
	@echo "  make db-stop"
	@echo "  make db-migrate"
	@echo "  make health"

start:
	dekit up
	dekit attach

stop:
	dekit down

run-api:
	cargo run -p api

test:
	cargo test --workspace

lint:
	cargo fmt --all -- --check

db-start:
	$(COMPOSE) up -d --wait

db-run:
	$(COMPOSE) up --detach $(DB_SERVICE)
	$(COMPOSE) logs --follow $(DB_SERVICE)

db-health:
	$(COMPOSE) exec -T $(DB_SERVICE) pg_isready --host=127.0.0.1 --username=$(DB_USER) --dbname=$(DB_NAME)

db-stop:
	$(COMPOSE) stop $(DB_SERVICE)

db-migrate:
	atlas migrate apply --env local

health:
	curl --fail http://localhost:3000/health
