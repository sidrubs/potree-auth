.DEFAULT_GOAL := help

# Show possible commands
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  run (r)           - Run a development instance of the server"
	@echo "  build (b)         - Build the project"
	@echo "  test (t)          - Run tests with all features"
	@echo "  clippy (lint)     - Run Clippy on the workspace"
	@echo "  fmt               - Format the project using nightly"
	@echo "  docker-build      - Build Docker image 'potree-auth:latest'"
	@echo "  docker-run        - Run the Docker image 'potree-auth:latest'"

# Development group
.PHONY: run r
run r:
	cargo run

.PHONY: build b
build b:
	cargo build --all-features --all-targets

.PHONY: test t
test t:
	cargo test --all-features --all

.PHONY: clippy lint
clippy lint:
	cargo clippy --all-features --all-targets

.PHONY: fmt
fmt:
	cargo +nightly fmt

# Deployment group
.PHONY: build-release br
build-release br:
	cargo build --release

.PHONY: docker-build
docker-build:
	./scripts/build_docker.sh

.PHONY: docker-run
docker-run:
	./scripts/run_docker.sh
