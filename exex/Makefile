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
# Features in reth/op-reth binary crate other than "ethereum" and "optimism"
BIN_OTHER_FEATURES := asm-keccak jemalloc jemalloc-prof min-error-logs min-warn-logs min-info-logs min-debug-logs min-trace-logs

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


fmt:
	cargo +nightly fmt
