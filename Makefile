.DEFAULT_GOAL := all

PROFILE := release

# Determine the operating system
UNAME := $(shell uname)

PROTOC := protoc
 ifeq ($(UNAME), Darwin)
	PROTOC = /opt/homebrew/opt/protobuf@21/bin/protoc
endif

CBINDGEN=${HOME}/.cargo/bin/cbindgen

.PHONY: check
check:
	@echo "Running Cargo check..."
	@cargo check --all --all-features --all-targets

.PHONY: test
test:
	@echo "Running Cargo test..."
	@cargo test

.PHONY: clippy
clippy:
	@echo "Running Cargo clippy..."
	@cargo clippy --all --all-features --all-targets -- -D warnings

.PHONY: deny
deny:
	@echo "Running Cargo deny..."
	@cargo deny check -c .cargo-deny.toml

.PHONY: format
format:
	@echo "Running Cargo fmt..."
	@rustup component add rustfmt --toolchain nightly
	@cargo +nightly fmt --all
format-check:
	@echo "Running Cargo fmt..."
	@rustup component add rustfmt --toolchain nightly
	@cargo +nightly fmt --all -- --check

.PHONY: pyembed
pyembed:
	@echo "Running PyO3 embed..."
	@cargo run --bin querent-pyembedder --release

.PHONY: build
build: pyembed
	@echo "Running Cargo build..."
	@cargo build --release --all-features -p querent-synapse

.PHONY: doc
doc:
	@echo "Running Cargo doc..."
	@RUSTDOCFLAGS="--enable-index-page --check -Zunstable-options" cargo doc --no-deps --all-features

.PHONY: clean
clean:
	@echo "Running Cargo clean..."
	@cargo clean

.PHONY: all
all: check test clippy deny format build doc
