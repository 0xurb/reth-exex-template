BUILD_PATH = "target"
# Extra flags for Cargo, if needed.
CARGO_INSTALL_EXTRA_FLAGS ?=

# List of features to use when building.
# No jemalloc on Windows
ifeq ($(OS),Windows_NT)
    FEATURES ?=
else
    FEATURES ?= jemalloc
endif

# Cargo profile for builds. Default is for local builds, CI uses an override.
PROFILE ?= release

##@ Build

.PHONY: install
install: ## Build and install the reth binary under `~/.cargo/bin`.
	cargo install --path bin/reth --bin reth --force --locked \
		--features "$(FEATURES)" \
		--profile "$(PROFILE)" \
		$(CARGO_INSTALL_EXTRA_FLAGS)

.PHONY: install-op
install-op: ## Build and install the op-reth binary under `~/.cargo/bin`.
	cargo install --path bin/reth --bin op-reth --force --locked \
		--features "optimism,$(FEATURES)" \
		--profile "$(PROFILE)" \
		$(CARGO_INSTALL_EXTRA_FLAGS)

.PHONY: build
build: ## Build the reth binary into `target` directory.
	cargo build --bin reth --features "$(FEATURES)" --profile "$(PROFILE)"

.PHONY: build-op
build-op: ## Build the op-reth binary into `target` directory.
	cargo build --bin op-reth --features "optimism,$(FEATURES)" --profile "$(PROFILE)"

.PHONY: lint
lint:
	make fmt && \
	make lint-reth && \
	make lint-op-reth && \
	make lint-workspace

lint-reth:
	cargo +nightly clippy \
		--workspace \
    	--bin "reth" \
       	--features "$(FEATURES)" \
       	--fix \
       	--allow-staged \
       	--allow-dirty \
       	-- -D warnings

lint-op-reth:
	cargo +nightly clippy \
    	--workspace \
    	--bin "op-reth" \
    	--lib \
    	--examples \
    	--tests \
    	--benches \
    	--features "optimism,$(FEATURES)" \
    	-- -D warnings

lint-workspace:
	cargo +nightly clippy \
		--workspace \
		--all-features \
    	-- -D warnings
fmt:
	cargo +nightly fmt

test-reth:
	cargo test --all-features

test-doc:
	cargo test --doc --workspace --features "ethereum"
	cargo test --doc --workspace --features "optimism"

test:
	make test-reth && \
	make test-doc
