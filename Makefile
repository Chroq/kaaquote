PHONY: default install start test

.DEFAULT_GOAL := help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

install: ## Install project's dependencies
	@echo "Install project deps"
	cargo build

start:
	@echo "Start the project"
	cargo run

release: ## Build for production and serve
	@echo "Start the project (production mode)"
	cargo run --release

test: ## Launch the project's tests
	@echo "Launch the tests"
	cargo test

debug: ## Launch the project's tests with debug
	@echo "Launch the tests as debug"
	rm -f data/kaa.json
	export RUST_BACKTRACE=1 && \
	    cargo test -- --nocapture && \
	    make start
