.SILENT:

SHELL=/usr/bin/env bash -O globstar

all: help

test: test_unit test_clippy test_fmt ## Runs tests

build: ## Build the project
	source test-utils.sh ;\
	section "Cargo build" ;\
	cargo build --all

format:  ## Fix formatting and clippy errors
	cargo fmt --all
	cargo clippy --all --fix --allow-dirty --allow-staged

test_unit:
	source test-utils.sh ;\
	section "Cargo test" ;\
	cargo test --all --all-features

test_clippy:
	source test-utils.sh ;\
	section "Cargo clippy" ;\
	cargo clippy -- -D warnings

test_fmt:
	source test-utils.sh ;\
	section "Cargo fmt" ;\
	cargo fmt --all -- --check

help: ## Display available commands
	echo "Available make commands:"
	echo
	grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
