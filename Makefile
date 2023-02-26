# MAKEFLAGS := --no-print-directory --silent

default: help

help:
	@echo "Please use 'make <target>' where <target> is one of"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z\._-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

lint:
	@echo "It is advised to lint in your IDE instead of running this command, see the README on how to achieve this."
	cargo clippy

dr: docker.run
docker.run: ## Run the containers in docker (this starts the docker stack), alias: dr
	docker-compose up -d

ds: docker.stop
docker.stop: ## Stop the containers in docker (this stops the docker stack), alias: ds
	docker-compose down

restart: docker.restart
docker.restart: ## Restart the containers in docker (this restarts the docker stack), alias: restart
	docker-compose down
	docker-compose up -d

cover: ## Run the tests and generate a coverage report
	cargo llvm-cov --html
	open target/llvm-cov/html/index.html