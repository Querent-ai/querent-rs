PROFILE := release

# Determine the operating system
UNAME := $(shell uname)

PROTOC := protoc
 ifeq ($(UNAME), Darwin)
	PROTOC = /opt/homebrew/opt/protobuf@21/bin/protoc
endif

debug ?=
target ?= $(shell rustc -vV | sed -n 's|host: ||p')
arch = $(shell uname -m)

ifdef debug
  cargo_build_opts :=
  target_type := debug
else
  cargo_build_opts := --release
  target_type = release
endif

ifneq ($(target),)
    cargo_build_opts += --target $(target)
endif


CBINDGEN=${HOME}/.cargo/bin/cbindgen
export PYO3_CONFIG_FILE = $(CURDIR)/build/pyo3-build-config-file-$(target).txt
$(info PYO3_CONFIG_FILE = $(PYO3_CONFIG_FILE))

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

.PHONY: build
build:
	@echo "Running Cargo build..."
	@env -u PYO3_CONFIG_FILE cargo build $(cargo_build_opts) --all-features

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
