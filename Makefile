
.PHONY: help start stop run-api test lint db-start db-run db-health db-stop db-migrate cornucopia-generate health

COMPOSE := docker compose
DB_SERVICE := db
DB_USER := postgres
DB_NAME := ahlan_commerce

AHLAN_DB_PORT ?= 5433
DATABASE_URL ?= postgres://postgres:postgres@localhost:$(AHLAN_DB_PORT)/$(DB_NAME)
export DATABASE_URL

CORNUCOPIA_SERVER_URL ?= postgres://postgres:postgres@localhost:$(AHLAN_DB_PORT)
CORNUCOPIA_QUERIES := db/queries
CORNUCOPIA_DESTINATION := packages/db/generated
MIGRATION_FILES := $(sort $(wildcard db/migrations/*.sql))
CORNUCOPIA_TMP := $(CURDIR)/packages/db

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
	@echo "  make cornucopia-generate"
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

cornucopia-generate: export TMP := $(CORNUCOPIA_TMP)
cornucopia-generate: export TEMP := $(CORNUCOPIA_TMP)
cornucopia-generate: export TMPDIR := $(CORNUCOPIA_TMP)
cornucopia-generate:
	cornucopia fresh --url $(CORNUCOPIA_SERVER_URL) --queries-path $(CORNUCOPIA_QUERIES) --destination $(CORNUCOPIA_DESTINATION) --sync false --async true $(MIGRATION_FILES)

health:
	curl --fail http://localhost:3000/health
