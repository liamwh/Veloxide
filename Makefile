MAKEFLAGS := --no-print-directory --silent

default: help

help:
	@echo "Please use 'make <target>' where <target> is one of"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z\._-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

lint:
	@echo "It is advised to lint in your IDE instead of running this command"
	cargo clippy

dev: docker.run ## Run the application and supporting containers
dr: docker.run
docker.run: ## Run the containers in docker (this starts the docker stack), alias: dr
	docker-compose up -d
	sqlx migrate run
	cargo run

stop: docker.stop
ds: docker.stop
docker.stop: ## Stop the containers in docker (this stops the docker stack), alias: ds
	docker-compose down

restart: docker.restart
docker.restart: docker.stop docker.run ## Restart the containers in docker (this restarts the docker stack), alias: restart

cover: ## Generates a code coverage report to be viewed in your IDE.
	cargo llvm-cov report --lcov --output-path ./coverage/lcov.info

cover.html: ## Generate a HTML coverage report and open it
	cargo llvm-cov --html
	open target/llvm-cov/html/index.html


tools.required: ## Install the required tools for development with Veloxide
	@echo "Installing tools..."

	@echo "Installing cargo-llvm-cov (code coverage report generation: https://github.com/taiki-e/cargo-llvm-cov)"
	cargo install cargo-llvm-cov

	@echo "Installing sqlx-cli (database migrations: https://crates.io/crates/sqlx-cli)"
	cargo install sqlx-cli --no-default-features --features postgres

	@echo "Installing tools...Done"

tools.recommended: ## Install recommended tooling that isn't required
	@echo "Installing recommended tools..."

	@echo "Installing bacon (background code checker: https://github.com/Canop/bacon)"
	cargo install bacon

	@echo "Installing cargo-watch (hot reloading: https://crates.io/crates/cargo-watch)"
	cargo install cargo-watch

	@echo "Installing recommended tools... Done"

tools.all: tools.required tools.recommended ## Install all required and recommended tools